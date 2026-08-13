use crate::timers::TimerQueue;
use android_activity::AndroidAppWaker;
use gpui::{PlatformDispatcher, Priority, RunnableVariant};
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

const MIN_BACKGROUND_THREADS: usize = 2;

struct TimerTask {
    deadline: Instant,
    runnable: RunnableVariant,
}

pub(crate) struct AndroidDispatcher {
    main_thread_id: thread::ThreadId,
    main_sender: flume::Sender<RunnableVariant>,
    background_sender: flume::Sender<RunnableVariant>,
    timer_sender: flume::Sender<TimerTask>,
    waker: AndroidAppWaker,
    _background_threads: Vec<thread::JoinHandle<()>>,
}

// AndroidAppWaker 本身已在 android-activity 中标注 `unsafe impl Send + Sync`
// （内部为 ALooper 的引用计数指针，跨线程调用安全），flume::Sender<T> 在
// T: Send 时天然 Send + Sync；因此 AndroidDispatcher 的所有字段均已满足
// Send + Sync，无需再手写 unsafe impl。

impl AndroidDispatcher {
    /// 必须在 android_main 线程上创建（该线程被视为 GPUI 主线程）。
    /// 返回的 receiver 由平台主循环在每次 poll 后 drain。
    pub(crate) fn new(waker: AndroidAppWaker) -> (Arc<Self>, flume::Receiver<RunnableVariant>) {
        let (main_sender, main_receiver) = flume::unbounded::<RunnableVariant>();
        let (background_sender, background_receiver) = flume::unbounded::<RunnableVariant>();
        let thread_count = thread::available_parallelism()
            .map_or(MIN_BACKGROUND_THREADS, |n| n.get().max(MIN_BACKGROUND_THREADS));

        let mut background_threads = (0..thread_count)
            .map(|i| {
                let receiver = background_receiver.clone();
                thread::Builder::new()
                    .name(format!("gpui-worker-{i}"))
                    .spawn(move || {
                        while let Ok(runnable) = receiver.recv() {
                            runnable.run();
                        }
                    })
                    .expect("failed to spawn background worker thread")
            })
            .collect::<Vec<_>>();

        let (timer_sender, timer_receiver) = flume::unbounded::<TimerTask>();
        let timer_thread = thread::Builder::new()
            .name("gpui-timer".to_owned())
            .spawn(move || run_timer_loop(timer_receiver))
            .expect("failed to spawn timer thread");
        background_threads.push(timer_thread);

        let dispatcher = Arc::new(Self {
            main_thread_id: thread::current().id(),
            main_sender,
            background_sender,
            timer_sender,
            waker,
            _background_threads: background_threads,
        });
        (dispatcher, main_receiver)
    }
}

fn run_timer_loop(receiver: flume::Receiver<TimerTask>) {
    let mut queue = TimerQueue::new();
    loop {
        let recv_result = match queue.next_deadline() {
            Some(deadline) => {
                let timeout = deadline.saturating_duration_since(Instant::now());
                if timeout.is_zero() {
                    Err(flume::RecvTimeoutError::Timeout)
                } else {
                    receiver.recv_timeout(timeout)
                }
            }
            None => receiver.recv().map_err(|_| flume::RecvTimeoutError::Disconnected),
        };
        match recv_result {
            Ok(task) => queue.insert(task.deadline, task.runnable),
            Err(flume::RecvTimeoutError::Timeout) => {}
            Err(flume::RecvTimeoutError::Disconnected) => break,
        }
        // 与 LinuxDispatcher 一致：到期的 runnable 直接在定时器线程上运行。
        for runnable in queue.pop_due(Instant::now()) {
            runnable.run();
        }
    }
}

impl PlatformDispatcher for AndroidDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    // PoC 忽略 Priority（FIFO）：gpui::queue 的优先级通道在 Android 上未编译，
    // 且 PoC 负载单一；二期如需优先级可自建多级队列。
    fn dispatch(&self, runnable: RunnableVariant, _priority: Priority) {
        if let Err(error) = self.background_sender.send(runnable) {
            // 后台线程池仅在进程退出时消失；参照 LinuxDispatcher，
            // runnable 可能持有 !Send 状态，禁止在错误路径 drop。
            std::mem::forget(error.into_inner());
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        match self.main_sender.send(runnable) {
            Ok(()) => self.waker.wake(),
            // main receiver 已被 drop 意味着应用正在退出；同上禁止 drop。
            Err(error) => std::mem::forget(error.into_inner()),
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let task = TimerTask { deadline: Instant::now() + duration, runnable };
        if let Err(error) = self.timer_sender.send(task) {
            std::mem::forget(error.into_inner());
        }
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        // Android 上不设实时调度优先级（需要额外权限），普通线程即可。
        if let Err(error) = thread::Builder::new().name("gpui-realtime".to_owned()).spawn(f) {
            log::error!("failed to spawn realtime thread: {error}");
        }
    }
}

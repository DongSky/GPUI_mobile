//! dispatch_after 用的最小堆定时队列；纯逻辑，host 可测。
use std::collections::BinaryHeap;
use std::time::Instant;

struct Entry<T> {
    deadline: Instant,
    sequence: u64,
    payload: T,
}

impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline && self.sequence == other.sequence
    }
}
impl<T> Eq for Entry<T> {}
impl<T> PartialOrd for Entry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Entry<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap 是最大堆，反转得到最早 deadline 优先。
        other
            .deadline
            .cmp(&self.deadline)
            .then(other.sequence.cmp(&self.sequence))
    }
}

pub struct TimerQueue<T> {
    heap: BinaryHeap<Entry<T>>,
    next_sequence: u64,
}

impl<T> TimerQueue<T> {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new(), next_sequence: 0 }
    }

    pub fn insert(&mut self, deadline: Instant, payload: T) {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.heap.push(Entry { deadline, sequence, payload });
    }

    pub fn next_deadline(&self) -> Option<Instant> {
        self.heap.peek().map(|entry| entry.deadline)
    }

    pub fn pop_due(&mut self, now: Instant) -> Vec<T> {
        let mut due = Vec::new();
        while self.heap.peek().is_some_and(|entry| entry.deadline <= now) {
            if let Some(entry) = self.heap.pop() {
                due.push(entry.payload);
            }
        }
        due
    }
}

impl<T> Default for TimerQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn pop_due_returns_only_expired_in_deadline_order() {
        let mut queue = TimerQueue::new();
        let now = Instant::now();
        queue.insert(now + Duration::from_millis(50), "late");
        queue.insert(now + Duration::from_millis(10), "early");
        queue.insert(now + Duration::from_millis(500), "future");
        let due = queue.pop_due(now + Duration::from_millis(100));
        assert_eq!(due, vec!["early", "late"]);
        assert_eq!(queue.next_deadline(), Some(now + Duration::from_millis(500)));
    }

    #[test]
    fn empty_queue_has_no_deadline() {
        let queue: TimerQueue<()> = TimerQueue::new();
        assert_eq!(queue.next_deadline(), None);
    }
}

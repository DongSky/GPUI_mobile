//! NativeActivity IME groundwork.
//!
//! `ImeSession` mirrors Android `InputConnection` (`commitText`, compose,
//! delete, cursor-anchor). `update_ime_position` fills the session and emits a
//! host-testable JNI call plan for `InputMethodManager` (`toggleSoftInput` is
//! the NativeActivity-safe show/hide). A live flush also constructs a dummy
//! `android.view.View` + `BaseInputConnection` (`hasCode=false` still cannot
//! `RegisterNatives` a Java peer).

use std::cell::Cell;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

/// Logical caret rectangle: `x, y, width, height` in device-independent pixels.
pub type ImeBoundsDp = [f32; 4];

/// Store the last IME caret rectangle (test hook + Android window field).
pub fn record_ime_position(slot: &Cell<Option<ImeBoundsDp>>, x: f32, y: f32, w: f32, h: f32) {
    slot.set(Some([x, y, w, h]));
}

pub fn last_ime_position(slot: &Cell<Option<ImeBoundsDp>>) -> Option<ImeBoundsDp> {
    slot.get()
}

/// `PlatformWindow::update_ime_position` feeds this from a GPUI caret `Bounds`.
pub fn record_caret_rect(slot: &Cell<Option<ImeBoundsDp>>, rect: (f32, f32, f32, f32)) {
    record_ime_position(slot, rect.0, rect.1, rect.2, rect.3);
}

/// Window-space caret rect from a field origin plus a local caret rect.
pub fn bounds_from_field_caret(
    field_x: f32,
    field_y: f32,
    caret: (f32, f32, f32, f32),
) -> ImeBoundsDp {
    [field_x + caret.0, field_y + caret.1, caret.2, caret.3]
}

/// Requests a later JNI `InputMethodManager` can consume.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImeRequest {
    ShowSoftInput,
    HideSoftInput,
    UpdateCursorAnchor,
}

/// Host-testable InputConnection session (no JNI).
#[derive(Clone, Debug, PartialEq)]
pub struct ImeSession {
    text: String,
    sel_start: usize,
    sel_end: usize,
    composing: Option<(usize, usize)>,
    pub bounds: Option<ImeBoundsDp>,
    pub shown: bool,
    pub last_request: Option<ImeRequest>,
}

impl Default for ImeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ImeSession {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            sel_start: 0,
            sel_end: 0,
            composing: None,
            bounds: None,
            shown: false,
            last_request: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn selection(&self) -> (usize, usize) {
        let a = self.sel_start.min(self.sel_end);
        let b = self.sel_start.max(self.sel_end);
        (a, b)
    }

    pub fn composing(&self) -> Option<(usize, usize)> {
        self.composing
    }

    pub fn show_soft_input(&mut self) {
        self.shown = true;
        self.last_request = Some(ImeRequest::ShowSoftInput);
    }

    pub fn hide_soft_input(&mut self) {
        self.shown = false;
        self.last_request = Some(ImeRequest::HideSoftInput);
    }

    /// `InputConnection.updateCursorAnchorInfo` / `update_ime_position`.
    pub fn update_cursor_anchor(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.bounds = Some([x, y, w, h]);
        self.shown = true;
        self.last_request = Some(ImeRequest::UpdateCursorAnchor);
    }

    pub fn set_selection(&mut self, start: usize, end: usize) {
        let n = self.text.chars().count();
        self.sel_start = start.min(n);
        self.sel_end = end.min(n);
    }

    pub fn get_text_before_cursor(&self, n: usize) -> String {
        let (start, _) = self.selection();
        let chars: Vec<char> = self.text.chars().collect();
        let from = start.saturating_sub(n);
        chars[from..start].iter().collect()
    }

    pub fn get_text_after_cursor(&self, n: usize) -> String {
        let (_, end) = self.selection();
        let chars: Vec<char> = self.text.chars().collect();
        let to = (end + n).min(chars.len());
        chars[end..to].iter().collect()
    }

    pub fn get_selected_text(&self) -> String {
        let (start, end) = self.selection();
        self.text.chars().skip(start).take(end - start).collect()
    }

    /// `InputConnection.commitText`.
    pub fn commit_text(&mut self, text: &str, _new_cursor_position: i32) {
        self.replace_selection(text);
        self.composing = None;
    }

    /// `InputConnection.setComposingText` (underlined composing region).
    pub fn set_composing_text(&mut self, text: &str, _new_cursor_position: i32) {
        if let Some((a, b)) = self.composing {
            self.sel_start = a;
            self.sel_end = b;
        }
        self.replace_selection(text);
        let end = self.sel_start;
        let start = end.saturating_sub(text.chars().count());
        self.composing = if text.is_empty() {
            None
        } else {
            Some((start, end))
        };
    }

    pub fn finish_composing_text(&mut self) {
        self.composing = None;
    }

    /// `InputConnection.deleteSurroundingText`.
    pub fn delete_surrounding_text(&mut self, before_length: usize, after_length: usize) {
        let (start, end) = self.selection();
        let mut chars: Vec<char> = self.text.chars().collect();
        let del_end = (end + after_length).min(chars.len());
        if del_end > end {
            chars.drain(end..del_end);
        }
        let del_start = start.saturating_sub(before_length);
        if start > del_start {
            chars.drain(del_start..start);
        }
        self.text = chars.into_iter().collect();
        let caret = del_start;
        self.sel_start = caret;
        self.sel_end = caret;
        self.composing = None;
    }

    fn replace_selection(&mut self, insert: &str) {
        let (start, end) = self.selection();
        let mut chars: Vec<char> = self.text.chars().collect();
        chars.drain(start..end);
        for (i, ch) in insert.chars().enumerate() {
            chars.insert(start + i, ch);
        }
        self.text = chars.into_iter().collect();
        let caret = start + insert.chars().count();
        self.sel_start = caret;
        self.sel_end = caret;
    }
}

/// JNI class / method / signature a NativeActivity can `FindClass` / `GetMethodID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JniMethod {
    pub class: &'static str,
    pub name: &'static str,
    pub sig: &'static str,
}

/// `InputMethodManager.SHOW_FORCED` — show without a focused `View`.
pub const IMM_SHOW_FORCED: i32 = 2;
/// `InputMethodManager.HIDE_IMPLICIT_ONLY`.
pub const IMM_HIDE_IMPLICIT_ONLY: i32 = 1;

pub const JNI_CONTEXT_GET_SYSTEM_SERVICE: JniMethod = JniMethod {
    class: "android/content/Context",
    name: "getSystemService",
    sig: "(Ljava/lang/String;)Ljava/lang/Object;",
};

pub const JNI_IMM_TOGGLE_SOFT_INPUT: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputMethodManager",
    name: "toggleSoftInput",
    sig: "(II)V",
};

pub const JNI_IMM_SHOW_SOFT_INPUT: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputMethodManager",
    name: "showSoftInput",
    sig: "(Landroid/view/View;I)Z",
};

pub const JNI_IMM_HIDE_SOFT_INPUT: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputMethodManager",
    name: "hideSoftInputFromWindow",
    sig: "(Landroid/os/IBinder;I)Z",
};

pub const JNI_IMM_UPDATE_CURSOR_ANCHOR: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputMethodManager",
    name: "updateCursorAnchorInfo",
    sig: "(Landroid/view/View;Landroid/view/inputmethod/CursorAnchorInfo;)V",
};

pub const JNI_VIEW_CTOR: JniMethod = JniMethod {
    class: "android/view/View",
    name: "<init>",
    sig: "(Landroid/content/Context;)V",
};

pub const JNI_BASE_IC_CTOR: JniMethod = JniMethod {
    class: "android/view/inputmethod/BaseInputConnection",
    name: "<init>",
    sig: "(Landroid/view/View;Z)V",
};

pub const JNI_IC_COMMIT_TEXT: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputConnection",
    name: "commitText",
    sig: "(Ljava/lang/CharSequence;I)Z",
};

pub const JNI_IC_SET_COMPOSING_TEXT: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputConnection",
    name: "setComposingText",
    sig: "(Ljava/lang/CharSequence;I)Z",
};

pub const JNI_IC_FINISH_COMPOSING: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputConnection",
    name: "finishComposingText",
    sig: "()Z",
};

pub const JNI_IC_DELETE_SURROUNDING: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputConnection",
    name: "deleteSurroundingText",
    sig: "(II)Z",
};

pub const JNI_IC_SET_SELECTION: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputConnection",
    name: "setSelection",
    sig: "(II)Z",
};

pub const JNI_IC_GET_TEXT_BEFORE: JniMethod = JniMethod {
    class: "android/view/inputmethod/InputConnection",
    name: "getTextBeforeCursor",
    sig: "(II)Ljava/lang/CharSequence;",
};

pub fn jni_input_method_manager_table() -> &'static [JniMethod] {
    &[
        JNI_CONTEXT_GET_SYSTEM_SERVICE,
        JNI_IMM_TOGGLE_SOFT_INPUT,
        JNI_IMM_SHOW_SOFT_INPUT,
        JNI_IMM_HIDE_SOFT_INPUT,
        JNI_IMM_UPDATE_CURSOR_ANCHOR,
    ]
}

pub fn jni_input_connection_table() -> &'static [JniMethod] {
    &[
        JNI_IC_COMMIT_TEXT,
        JNI_IC_SET_COMPOSING_TEXT,
        JNI_IC_FINISH_COMPOSING,
        JNI_IC_DELETE_SURROUNDING,
        JNI_IC_SET_SELECTION,
        JNI_IC_GET_TEXT_BEFORE,
    ]
}

/// Packed `CursorAnchorInfo` insertion-marker + selection (no JNI yet).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CursorAnchorInfoPayload {
    pub composing_start: i32,
    pub composing_end: i32,
    pub selection_start: i32,
    pub selection_end: i32,
    /// left, top, right, bottom in window px.
    pub insertion_marker: [f32; 4],
}

/// JNI calls a NativeActivity `JNIEnv` should issue for the last IME request.
#[derive(Clone, Debug, PartialEq)]
pub enum JniImmCall {
    GetSystemService {
        name: &'static str,
    },
    ToggleSoftInput {
        show_flags: i32,
        hide_flags: i32,
    },
    NewView,
    NewBaseInputConnection,
    UpdateCursorAnchorInfo {
        payload: CursorAnchorInfoPayload,
    },
}

impl ImeSession {
    pub fn cursor_anchor_payload(&self) -> CursorAnchorInfoPayload {
        let (sel_start, sel_end) = self.selection();
        let (comp_start, comp_end) = self.composing.unwrap_or((sel_start, sel_start));
        let [x, y, w, h] = self.bounds.unwrap_or([0.0, 0.0, 0.0, 0.0]);
        CursorAnchorInfoPayload {
            composing_start: comp_start as i32,
            composing_end: comp_end as i32,
            selection_start: sel_start as i32,
            selection_end: sel_end as i32,
            insertion_marker: [x, y, x + w, y + h],
        }
    }

    /// NativeActivity-safe IMM plan: `getSystemService` + `toggleSoftInput`,
    /// plus `updateCursorAnchorInfo` when caret bounds exist.
    pub fn jni_imm_calls(&self) -> Vec<JniImmCall> {
        let mut calls = vec![JniImmCall::GetSystemService {
            name: "input_method",
        }];
        match self.last_request {
            Some(ImeRequest::HideSoftInput) => {
                calls.push(JniImmCall::ToggleSoftInput {
                    show_flags: 0,
                    hide_flags: IMM_HIDE_IMPLICIT_ONLY,
                });
            }
            Some(ImeRequest::ShowSoftInput) | Some(ImeRequest::UpdateCursorAnchor) => {
                calls.push(JniImmCall::ToggleSoftInput {
                    show_flags: IMM_SHOW_FORCED,
                    hide_flags: 0,
                });
                calls.push(JniImmCall::NewView);
                calls.push(JniImmCall::NewBaseInputConnection);
                if self.bounds.is_some() {
                    calls.push(JniImmCall::UpdateCursorAnchorInfo {
                        payload: self.cursor_anchor_payload(),
                    });
                }
            }
            None => {}
        }
        calls
    }
}

/// Dispatch a JNI `InputConnection` native method onto the session.
pub fn dispatch_native_input_connection(
    session: &mut ImeSession,
    method: &str,
    text: Option<&str>,
    a: i32,
    b: i32,
) -> Option<String> {
    match method {
        "commitText" => {
            session.commit_text(text.unwrap_or(""), a);
            None
        }
        "setComposingText" => {
            session.set_composing_text(text.unwrap_or(""), a);
            None
        }
        "finishComposingText" => {
            session.finish_composing_text();
            None
        }
        "deleteSurroundingText" => {
            session.delete_surrounding_text(a.max(0) as usize, b.max(0) as usize);
            None
        }
        "setSelection" => {
            session.set_selection(a.max(0) as usize, b.max(0) as usize);
            None
        }
        "getTextBeforeCursor" => Some(session.get_text_before_cursor(a.max(0) as usize)),
        "getTextAfterCursor" => Some(session.get_text_after_cursor(a.max(0) as usize)),
        _ => None,
    }
}

/// Apply a GPUI `update_ime_position` rect to both the legacy slot and the session.
pub fn apply_update_ime_position(
    slot: &Cell<Option<ImeBoundsDp>>,
    session: &std::cell::RefCell<ImeSession>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    record_ime_position(slot, x, y, w, h);
    session.borrow_mut().update_cursor_anchor(x, y, w, h);
}

/// `RegisterNatives` layout for a NativeActivity `InputConnection` peer.
pub fn jni_register_natives_input_connection() -> &'static [JniMethod] {
    jni_input_connection_table()
}

/// JNI method name a `JNIEnv` would invoke for `call`.
pub fn jni_imm_call_name(call: &JniImmCall) -> &'static str {
    match call {
        JniImmCall::GetSystemService { .. } => "getSystemService",
        JniImmCall::ToggleSoftInput { .. } => "toggleSoftInput",
        JniImmCall::NewView => "<init>",
        JniImmCall::NewBaseInputConnection => "<init>",
        JniImmCall::UpdateCursorAnchorInfo { .. } => "updateCursorAnchorInfo",
    }
}

/// Pending IMM JNI calls filled by `update_ime_position` (no live `JNIEnv` yet).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImeJniQueue {
    pending: Vec<JniImmCall>,
}

impl ImeJniQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sync_from(&mut self, session: &ImeSession) {
        self.pending = session.jni_imm_calls();
    }

    pub fn pending(&self) -> &[JniImmCall] {
        &self.pending
    }

    pub fn take(&mut self) -> Vec<JniImmCall> {
        std::mem::take(&mut self.pending)
    }

    /// Host-testable `JNIEnv` walk: records method names in call order.
    pub fn dry_run_jni_env(&self) -> Vec<&'static str> {
        self.pending.iter().map(jni_imm_call_name).collect()
    }
}

/// `JNINativeMethod` row a NativeActivity `RegisterNatives` table would bind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JniNativeMethodDesc {
    pub name: &'static str,
    pub signature: &'static str,
    pub handler: &'static str,
}

pub fn jni_native_method_descriptors() -> Vec<JniNativeMethodDesc> {
    jni_register_natives_input_connection()
        .iter()
        .map(|m| JniNativeMethodDesc {
            name: m.name,
            signature: m.sig,
            handler: m.name,
        })
        .collect()
}

/// Host-testable `JNIEnv` operations (FindClass / GetMethodID / CallVoidMethod).
pub trait JniEnvSink {
    fn find_class(&mut self, class: &str);
    fn get_method_id(&mut self, class: &str, name: &str, sig: &str);
    fn call_void(&mut self, name: &str);
    fn new_object(&mut self, class: &str, sig: &str) {
        self.find_class(class);
        self.get_method_id(class, "<init>", sig);
        self.call_void("<init>");
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecordingJniSink {
    pub log: Vec<String>,
}

impl JniEnvSink for RecordingJniSink {
    fn find_class(&mut self, class: &str) {
        self.log.push(format!("FindClass {class}"));
    }
    fn get_method_id(&mut self, class: &str, name: &str, sig: &str) {
        self.log.push(format!("GetMethodID {class}.{name}{sig}"));
    }
    fn call_void(&mut self, name: &str) {
        self.log.push(format!("CallVoidMethod {name}"));
    }
}

/// Walk `queue` as a NativeActivity `JNIEnv` would (`getSystemService` + IMM + View IC).
pub fn flush_ime_jni_queue(sink: &mut dyn JniEnvSink, queue: &ImeJniQueue) -> usize {
    let mut n = 0usize;
    for call in queue.pending() {
        match call {
            JniImmCall::GetSystemService { name } => {
                sink.find_class(JNI_CONTEXT_GET_SYSTEM_SERVICE.class);
                sink.get_method_id(
                    JNI_CONTEXT_GET_SYSTEM_SERVICE.class,
                    JNI_CONTEXT_GET_SYSTEM_SERVICE.name,
                    JNI_CONTEXT_GET_SYSTEM_SERVICE.sig,
                );
                sink.call_void(name);
                n += 1;
            }
            JniImmCall::ToggleSoftInput { .. } => {
                sink.find_class(JNI_IMM_TOGGLE_SOFT_INPUT.class);
                sink.get_method_id(
                    JNI_IMM_TOGGLE_SOFT_INPUT.class,
                    JNI_IMM_TOGGLE_SOFT_INPUT.name,
                    JNI_IMM_TOGGLE_SOFT_INPUT.sig,
                );
                sink.call_void(JNI_IMM_TOGGLE_SOFT_INPUT.name);
                n += 1;
            }
            JniImmCall::NewView => {
                sink.new_object(JNI_VIEW_CTOR.class, JNI_VIEW_CTOR.sig);
                n += 1;
            }
            JniImmCall::NewBaseInputConnection => {
                sink.new_object(JNI_BASE_IC_CTOR.class, JNI_BASE_IC_CTOR.sig);
                n += 1;
            }
            JniImmCall::UpdateCursorAnchorInfo { .. } => {
                sink.find_class(JNI_IMM_UPDATE_CURSOR_ANCHOR.class);
                sink.get_method_id(
                    JNI_IMM_UPDATE_CURSOR_ANCHOR.class,
                    JNI_IMM_UPDATE_CURSOR_ANCHOR.name,
                    JNI_IMM_UPDATE_CURSOR_ANCHOR.sig,
                );
                sink.call_void(JNI_IMM_UPDATE_CURSOR_ANCHOR.name);
                n += 1;
            }
        }
    }
    n
}

static ATTACHED_JNI_ENV: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ATTACHED_VM: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ATTACHED_ACTIVITY: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Store the NativeActivity `JNIEnv*` (null clears). Safe to call from host tests.
pub unsafe fn attach_jni_env(env: *mut c_void) {
    ATTACHED_JNI_ENV.store(env, Ordering::SeqCst);
}

pub fn attached_jni_env() -> *mut c_void {
    ATTACHED_JNI_ENV.load(Ordering::SeqCst)
}

/// Bind NativeActivity `JavaVM*` + activity jobject (from `AndroidApp`).
pub unsafe fn attach_native_activity(vm: *mut c_void, activity: *mut c_void) {
    ATTACHED_VM.store(vm, Ordering::SeqCst);
    ATTACHED_ACTIVITY.store(activity, Ordering::SeqCst);
    if !vm.is_null() {
        unsafe { attach_jni_env(vm) };
    }
}

pub unsafe fn detach_native_activity() {
    ATTACHED_VM.store(std::ptr::null_mut(), Ordering::SeqCst);
    ATTACHED_ACTIVITY.store(std::ptr::null_mut(), Ordering::SeqCst);
    unsafe { attach_jni_env(std::ptr::null_mut()) };
}

pub fn attached_native_activity() -> (*mut c_void, *mut c_void) {
    (
        ATTACHED_VM.load(Ordering::SeqCst),
        ATTACHED_ACTIVITY.load(Ordering::SeqCst),
    )
}

/// Flush the IMM queue when a `JNIEnv` is attached. Host tests pass a dummy
/// non-null pointer; a live JVM is still required for `CallVoidMethod`.
pub fn flush_if_attached(queue: &ImeJniQueue) -> Result<usize, &'static str> {
    if attached_jni_env().is_null() {
        Err("no JNIEnv")
    } else {
        Ok(queue.pending().len())
    }
}

/// Flush via NativeActivity handles. On device this walks the JNI vtable;
/// on host it dry-runs once both pointers are attached.
pub fn flush_native_activity_imm(queue: &ImeJniQueue) -> Result<usize, &'static str> {
    let (vm, activity) = attached_native_activity();
    if vm.is_null() || activity.is_null() {
        return Err("no NativeActivity");
    }
    #[cfg(target_os = "android")]
    {
        unsafe { flush_toggle_soft_input_jni(vm, activity, queue) }
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = (vm, activity);
        Ok(queue.pending().len())
    }
}

#[cfg(target_os = "android")]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn flush_toggle_soft_input_jni(
    vm: *mut c_void,
    activity: *mut c_void,
    queue: &ImeJniQueue,
) -> Result<usize, &'static str> {
    use jni_sys::{
        jboolean, jvalue, JNIEnv, JavaVM, JNI_OK, JNI_TRUE, JNI_VERSION_1_6, jobject,
    };
    let mut show_flags = 0i32;
    let mut hide_flags = 0i32;
    let mut saw_toggle = false;
    let mut want_view = false;
    let mut want_ic = false;
    for call in queue.pending() {
        match call {
            JniImmCall::ToggleSoftInput {
                show_flags: s,
                hide_flags: h,
            } => {
                show_flags = *s;
                hide_flags = *h;
                saw_toggle = true;
            }
            JniImmCall::NewView => want_view = true,
            JniImmCall::NewBaseInputConnection => want_ic = true,
            _ => {}
        }
    }
    if !saw_toggle && !want_view && !want_ic {
        return Ok(queue.pending().len());
    }
    let vm = vm as *mut JavaVM;
    if vm.is_null() {
        return Err("null JavaVM");
    }
    let invoke = unsafe { &(**vm).v1_2 };
    let mut env: *mut JNIEnv = std::ptr::null_mut();
    let rc = (invoke.GetEnv)(vm, &mut env as *mut _ as *mut *mut c_void, JNI_VERSION_1_6);
    if rc != JNI_OK || env.is_null() {
        let rc = (invoke.AttachCurrentThread)(
            vm,
            &mut env as *mut _ as *mut *mut c_void,
            std::ptr::null_mut(),
        );
        if rc != JNI_OK || env.is_null() {
            return Err("GetEnv");
        }
    }
    let jni = unsafe { &(**env).v1_2 };
    let jni_ok = |env: *mut JNIEnv| -> Result<(), &'static str> {
        if (jni.ExceptionCheck)(env) {
            (jni.ExceptionClear)(env);
            Err("jni exception")
        } else {
            Ok(())
        }
    };
    let ctx_cls = (jni.FindClass)(env, b"android/content/Context\0".as_ptr().cast());
    jni_ok(env)?;
    if ctx_cls.is_null() {
        return Err("FindClass Context");
    }
    let get_svc = (jni.GetMethodID)(
        env,
        ctx_cls,
        b"getSystemService\0".as_ptr().cast(),
        b"(Ljava/lang/String;)Ljava/lang/Object;\0".as_ptr().cast(),
    );
    jni_ok(env)?;
    if get_svc.is_null() {
        return Err("getSystemService");
    }
    let name = (jni.NewStringUTF)(env, b"input_method\0".as_ptr().cast());
    jni_ok(env)?;
    if name.is_null() {
        return Err("NewStringUTF");
    }
    let args_svc = [jvalue { l: name }];
    let imm = (jni.CallObjectMethodA)(env, activity as jobject, get_svc, args_svc.as_ptr());
    jni_ok(env)?;
    if imm.is_null() {
        return Err("input_method");
    }
    let imm_cls = (jni.FindClass)(
        env,
        b"android/view/inputmethod/InputMethodManager\0".as_ptr().cast(),
    );
    jni_ok(env)?;
    if imm_cls.is_null() {
        return Err("FindClass IMM");
    }
    let toggle = (jni.GetMethodID)(
        env,
        imm_cls,
        b"toggleSoftInput\0".as_ptr().cast(),
        b"(II)V\0".as_ptr().cast(),
    );
    jni_ok(env)?;
    if toggle.is_null() {
        return Err("toggleSoftInput");
    }
    let args_toggle = [jvalue { i: show_flags }, jvalue { i: hide_flags }];
    if saw_toggle {
        (jni.CallVoidMethodA)(env, imm, toggle, args_toggle.as_ptr());
        jni_ok(env)?;
    }

    if want_view {
        let view_cls = (jni.FindClass)(env, b"android/view/View\0".as_ptr().cast());
        jni_ok(env)?;
        if view_cls.is_null() {
            return Err("FindClass View");
        }
        let view_ctor = (jni.GetMethodID)(
            env,
            view_cls,
            b"<init>\0".as_ptr().cast(),
            b"(Landroid/content/Context;)V\0".as_ptr().cast(),
        );
        jni_ok(env)?;
        if view_ctor.is_null() {
            return Err("View.<init>");
        }
        let args_view = [jvalue {
            l: activity as jobject,
        }];
        let view = (jni.NewObjectA)(env, view_cls, view_ctor, args_view.as_ptr());
        jni_ok(env)?;
        if view.is_null() {
            return Err("new View");
        }
        if want_ic {
            let ic_cls = (jni.FindClass)(
                env,
                b"android/view/inputmethod/BaseInputConnection\0".as_ptr().cast(),
            );
            jni_ok(env)?;
            if ic_cls.is_null() {
                return Err("FindClass BaseInputConnection");
            }
            let ic_ctor = (jni.GetMethodID)(
                env,
                ic_cls,
                b"<init>\0".as_ptr().cast(),
                b"(Landroid/view/View;Z)V\0".as_ptr().cast(),
            );
            jni_ok(env)?;
            if ic_ctor.is_null() {
                return Err("BaseInputConnection.<init>");
            }
            let args_ic = [
                jvalue { l: view },
                jvalue {
                    z: JNI_TRUE as jboolean,
                },
            ];
            let _ic = (jni.NewObjectA)(env, ic_cls, ic_ctor, args_ic.as_ptr());
            jni_ok(env)?;
        }
    }

    Ok(queue.pending().len())
}

/// Record caret + session + the IMM JNI queue a NativeActivity should flush.
pub fn apply_update_ime_position_queued(
    slot: &Cell<Option<ImeBoundsDp>>,
    session: &std::cell::RefCell<ImeSession>,
    queue: &std::cell::RefCell<ImeJniQueue>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    apply_update_ime_position(slot, session, x, y, w, h);
    queue.borrow_mut().sync_from(&session.borrow());
}

/// Dispatch every `RegisterNatives` InputConnection method name onto `session`.
pub fn dispatch_registered_input_connection(
    session: &mut ImeSession,
    method: &str,
    text: Option<&str>,
    a: i32,
    b: i32,
) -> Option<String> {
    let known = jni_register_natives_input_connection()
        .iter()
        .any(|m| m.name == method);
    if !known && method != "getTextAfterCursor" {
        return None;
    }
    dispatch_native_input_connection(session, method, text, a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn records_caret_rect_for_update_ime_position() {
        let slot = Cell::new(None);
        assert_eq!(last_ime_position(&slot), None);
        record_ime_position(&slot, 16.0, 16.0, 2.0, 24.0);
        assert_eq!(last_ime_position(&slot), Some([16.0, 16.0, 2.0, 24.0]));
        record_caret_rect(&slot, (20.0, 8.0, 2.0, 24.0));
        assert_eq!(last_ime_position(&slot), Some([20.0, 8.0, 2.0, 24.0]));
    }

    #[test]
    fn input_connection_commit_and_delete() {
        let mut ime = ImeSession::new();
        ime.show_soft_input();
        assert!(ime.shown);
        ime.commit_text("hel", 1);
        ime.commit_text("lo", 1);
        assert_eq!(ime.text(), "hello");
        assert_eq!(ime.get_text_before_cursor(3), "llo");
        ime.set_selection(0, 2);
        assert_eq!(ime.get_selected_text(), "he");
        ime.commit_text("He", 1);
        assert_eq!(ime.text(), "Hello");
        ime.set_selection(5, 5);
        ime.delete_surrounding_text(1, 0);
        assert_eq!(ime.text(), "Hell");
        ime.hide_soft_input();
        assert!(!ime.shown);
        assert_eq!(ime.last_request, Some(ImeRequest::HideSoftInput));
    }

    #[test]
    fn composing_region_and_cursor_anchor() {
        let mut ime = ImeSession::new();
        ime.set_composing_text("ni", 1);
        assert_eq!(ime.text(), "ni");
        assert_eq!(ime.composing(), Some((0, 2)));
        ime.set_composing_text("你好", 1);
        assert_eq!(ime.text(), "你好");
        ime.finish_composing_text();
        assert_eq!(ime.composing(), None);
        ime.update_cursor_anchor(24.0, 12.0, 2.0, 24.0);
        assert_eq!(ime.bounds, Some([24.0, 12.0, 2.0, 24.0]));
        assert_eq!(ime.last_request, Some(ImeRequest::UpdateCursorAnchor));
        let mapped = bounds_from_field_caret(8.0, 16.0, (16.0, 16.0, 2.0, 24.0));
        assert_eq!(mapped, [24.0, 32.0, 2.0, 24.0]);
    }

    #[test]
    fn apply_update_ime_position_fills_slot_and_session() {
        let slot = Cell::new(None);
        let session = RefCell::new(ImeSession::new());
        apply_update_ime_position(&slot, &session, 10.0, 20.0, 2.0, 24.0);
        assert_eq!(last_ime_position(&slot), Some([10.0, 20.0, 2.0, 24.0]));
        assert_eq!(session.borrow().bounds, Some([10.0, 20.0, 2.0, 24.0]));
        assert!(session.borrow().shown);
    }

    #[test]
    fn jni_imm_plan_and_input_connection_native_dispatch() {
        assert_eq!(JNI_IMM_TOGGLE_SOFT_INPUT.sig, "(II)V");
        assert_eq!(JNI_IC_COMMIT_TEXT.sig, "(Ljava/lang/CharSequence;I)Z");
        assert!(
            jni_input_method_manager_table()
                .iter()
                .any(|m| m.name == "toggleSoftInput")
        );
        assert!(
            jni_input_connection_table()
                .iter()
                .any(|m| m.name == "commitText")
        );

        let mut ime = ImeSession::new();
        dispatch_native_input_connection(&mut ime, "commitText", Some("hi"), 1, 0);
        assert_eq!(ime.text(), "hi");
        assert_eq!(
            dispatch_native_input_connection(&mut ime, "getTextBeforeCursor", None, 1, 0)
                .as_deref(),
            Some("i")
        );
        ime.update_cursor_anchor(8.0, 16.0, 2.0, 24.0);
        let calls = ime.jni_imm_calls();
        assert!(matches!(
            calls[0],
            JniImmCall::GetSystemService { name: "input_method" }
        ));
        assert!(matches!(
            calls[1],
            JniImmCall::ToggleSoftInput {
                show_flags: IMM_SHOW_FORCED,
                hide_flags: 0
            }
        ));
        assert!(matches!(calls[2], JniImmCall::NewView));
        assert!(matches!(calls[3], JniImmCall::NewBaseInputConnection));
        match &calls[4] {
            JniImmCall::UpdateCursorAnchorInfo { payload } => {
                assert_eq!(payload.insertion_marker, [8.0, 16.0, 10.0, 40.0]);
                assert_eq!(payload.selection_end, 2);
            }
            other => panic!("expected cursor-anchor, got {other:?}"),
        }
    }

    #[test]
    fn jni_queue_and_register_natives_after_caret() {
        assert!(
            jni_register_natives_input_connection()
                .iter()
                .any(|m| m.name == "commitText" && m.sig == JNI_IC_COMMIT_TEXT.sig)
        );
        let slot = Cell::new(None);
        let session = RefCell::new(ImeSession::new());
        let queue = RefCell::new(ImeJniQueue::new());
        apply_update_ime_position_queued(&slot, &session, &queue, 8.0, 16.0, 2.0, 24.0);
        let names = queue.borrow().dry_run_jni_env();
        assert_eq!(
            names,
            [
                "getSystemService",
                "toggleSoftInput",
                "<init>",
                "<init>",
                "updateCursorAnchorInfo"
            ]
        );
        match &queue.borrow().pending()[1] {
            JniImmCall::ToggleSoftInput {
                show_flags,
                hide_flags,
            } => {
                assert_eq!(*show_flags, IMM_SHOW_FORCED);
                assert_eq!(*hide_flags, 0);
            }
            other => panic!("expected toggleSoftInput, got {other:?}"),
        }
        let mut ime = ImeSession::new();
        dispatch_registered_input_connection(&mut ime, "commitText", Some("ok"), 1, 0);
        assert_eq!(ime.text(), "ok");
        assert!(
            dispatch_registered_input_connection(&mut ime, "notAMethod", None, 0, 0).is_none()
        );
        let mut sink = RecordingJniSink::default();
        let n = flush_ime_jni_queue(&mut sink, &queue.borrow());
        assert_eq!(n, 5);
        assert!(sink.log.iter().any(|s| s.contains("toggleSoftInput")));
        assert!(sink.log.iter().any(|s| s.contains("android/view/View")));
        assert!(
            sink.log
                .iter()
                .any(|s| s.contains("android/view/inputmethod/BaseInputConnection"))
        );
        assert!(
            jni_native_method_descriptors()
                .iter()
                .any(|m| m.name == "commitText" && m.handler == "commitText")
        );
        assert_eq!(flush_if_attached(&queue.borrow()), Err("no JNIEnv"));
        unsafe { attach_jni_env(0x1 as *mut _) };
        let _clear = scopeguard_clear_env();
        assert_eq!(flush_if_attached(&queue.borrow()), Ok(5));
    }

    #[test]
    fn native_activity_attach_dry_runs_imm_flush() {
        assert_eq!(
            flush_native_activity_imm(&ImeJniQueue::new()),
            Err("no NativeActivity")
        );
        let slot = Cell::new(None);
        let session = RefCell::new(ImeSession::new());
        let queue = RefCell::new(ImeJniQueue::new());
        apply_update_ime_position_queued(&slot, &session, &queue, 8.0, 16.0, 2.0, 24.0);
        unsafe { attach_native_activity(0x1 as *mut _, 0x2 as *mut _) };
        let _clear = scopeguard_clear_native();
        assert_eq!(flush_native_activity_imm(&queue.borrow()), Ok(5));
        assert_eq!(flush_if_attached(&queue.borrow()), Ok(5));
    }

    fn scopeguard_clear_native() -> impl Drop {
        struct Clear;
        impl Drop for Clear {
            fn drop(&mut self) {
                unsafe { detach_native_activity() };
            }
        }
        Clear
    }

    fn scopeguard_clear_env() -> impl Drop {
        struct Clear;
        impl Drop for Clear {
            fn drop(&mut self) {
                unsafe { detach_native_activity() };
            }
        }
        Clear
    }
}

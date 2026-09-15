//! NativeActivity IME groundwork.
//!
//! `ImeSession` mirrors Android `InputConnection` (`commitText`, compose,
//! delete, cursor-anchor). `update_ime_position` fills the session and emits a
//! host-testable JNI call plan for `InputMethodManager` (`toggleSoftInput` is
//! the NativeActivity-safe show/hide). A live flush constructs a dummy
//! `android.view.View`, prefers `dev.gpui.material.NativeInputConnection`
//! (`hasCode=true`), and builds a real `CursorAnchorInfo` via
//! `CallVoidMethod` / `CallObjectMethod`. `RegisterNatives` binds
//! `JNINativeMethod.fnPtr` onto the JNI-mangled
//! `Java_dev_gpui_material_NativeInputConnection_native*` exports.

use std::cell::Cell;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Mutex;

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

/// Java peer compiled into the APK (`application hasCode=true`).
pub const NATIVE_IC_CLASS: &str = "dev/gpui/material/NativeInputConnection";
pub const NATIVE_IC_HAS_CODE: bool = true;

pub const JNI_NATIVE_IC_CTOR: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "<init>",
    sig: "(Landroid/view/View;ZJ)V",
};

/// Two-arg fallback when the handle ctor is missing.
pub const JNI_NATIVE_IC_CTOR_LEGACY: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "<init>",
    sig: "(Landroid/view/View;Z)V",
};

/// Catalog / NativeActivity session id bound into `NativeInputConnection`.
pub const NATIVE_IC_SESSION_HANDLE: i64 = 1;

pub const JNI_NATIVE_COMMIT_TEXT: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "nativeCommitText",
    sig: "(JLjava/lang/String;I)Z",
};

pub const JNI_NATIVE_SET_COMPOSING: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "nativeSetComposingText",
    sig: "(JLjava/lang/String;I)Z",
};

pub const JNI_NATIVE_FINISH_COMPOSING: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "nativeFinishComposingText",
    sig: "(J)Z",
};

pub const JNI_NATIVE_DELETE_SURROUNDING: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "nativeDeleteSurroundingText",
    sig: "(JII)Z",
};

pub const JNI_NATIVE_SET_SELECTION: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "nativeSetSelection",
    sig: "(JII)Z",
};

pub const JNI_NATIVE_GET_TEXT_BEFORE: JniMethod = JniMethod {
    class: NATIVE_IC_CLASS,
    name: "nativeGetTextBeforeCursor",
    sig: "(JI)Ljava/lang/String;",
};

/// `RegisterNatives` table — only `native` methods on `NativeInputConnection`.
pub fn jni_native_peer_table() -> &'static [JniMethod] {
    &[
        JNI_NATIVE_COMMIT_TEXT,
        JNI_NATIVE_SET_COMPOSING,
        JNI_NATIVE_FINISH_COMPOSING,
        JNI_NATIVE_DELETE_SURROUNDING,
        JNI_NATIVE_SET_SELECTION,
        JNI_NATIVE_GET_TEXT_BEFORE,
    ]
}

/// Map a Java `native*` name onto the InputConnection handler.
pub fn native_peer_handler(name: &str) -> Option<&'static str> {
    match name {
        "nativeCommitText" => Some("commitText"),
        "nativeSetComposingText" => Some("setComposingText"),
        "nativeFinishComposingText" => Some("finishComposingText"),
        "nativeDeleteSurroundingText" => Some("deleteSurroundingText"),
        "nativeSetSelection" => Some("setSelection"),
        "nativeGetTextBeforeCursor" => Some("getTextBeforeCursor"),
        _ => None,
    }
}

pub const JNI_CAI_BUILDER_CTOR: JniMethod = JniMethod {
    class: "android/view/inputmethod/CursorAnchorInfo$Builder",
    name: "<init>",
    sig: "()V",
};

pub const JNI_CAI_SET_SELECTION: JniMethod = JniMethod {
    class: "android/view/inputmethod/CursorAnchorInfo$Builder",
    name: "setSelectionRange",
    sig: "(II)Landroid/view/inputmethod/CursorAnchorInfo$Builder;",
};

pub const JNI_CAI_SET_INSERTION: JniMethod = JniMethod {
    class: "android/view/inputmethod/CursorAnchorInfo$Builder",
    name: "setInsertionMarkerLocation",
    sig: "(FFFFI)Landroid/view/inputmethod/CursorAnchorInfo$Builder;",
};

pub const JNI_CAI_BUILD: JniMethod = JniMethod {
    class: "android/view/inputmethod/CursorAnchorInfo$Builder",
    name: "build",
    sig: "()Landroid/view/inputmethod/CursorAnchorInfo;",
};

/// `CursorAnchorInfo.FLAG_HAS_VISIBLE_REGION`.
pub const CAI_FLAG_HAS_VISIBLE_REGION: i32 = 1;

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
    NewNativeInputConnection,
    RegisterNatives {
        handle: i64,
    },
    NewCursorAnchorInfo {
        payload: CursorAnchorInfoPayload,
    },
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
                calls.push(JniImmCall::RegisterNatives {
                    handle: NATIVE_IC_SESSION_HANDLE,
                });
                calls.push(JniImmCall::NewNativeInputConnection);
                if self.bounds.is_some() {
                    let payload = self.cursor_anchor_payload();
                    calls.push(JniImmCall::NewCursorAnchorInfo { payload });
                    calls.push(JniImmCall::UpdateCursorAnchorInfo { payload });
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
        JniImmCall::NewNativeInputConnection => "NativeInputConnection",
        JniImmCall::RegisterNatives { .. } => "RegisterNatives",
        JniImmCall::NewCursorAnchorInfo { .. } => "CursorAnchorInfo$Builder",
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

/// `RegisterNatives` rows for the Java `native*` methods (not BaseIC overrides).
pub fn jni_native_peer_descriptors() -> Vec<JniNativeMethodDesc> {
    jni_native_peer_table()
        .iter()
        .map(|m| JniNativeMethodDesc {
            name: m.name,
            signature: m.sig,
            handler: native_peer_handler(m.name).unwrap_or(m.name),
        })
        .collect()
}

/// JNI auto-link prefix for `dev.gpui.material.NativeInputConnection`.
pub const JNI_MANGLED_PREFIX: &str = "Java_dev_gpui_material_NativeInputConnection_";

pub fn jni_mangled_name(native_method: &str) -> String {
    format!("{JNI_MANGLED_PREFIX}{native_method}")
}

/// `JNINativeMethod` row with a live `fnPtr` (mangled export or RegisterNatives).
#[derive(Clone, Copy, Debug)]
pub struct JniNativeFnPtr {
    pub name: &'static str,
    pub signature: &'static str,
    pub handler: &'static str,
    pub mangled: &'static str,
    pub fn_ptr: *const c_void,
}

unsafe impl Send for JniNativeFnPtr {}
unsafe impl Sync for JniNativeFnPtr {}

impl PartialEq for JniNativeFnPtr {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.signature == other.signature
            && self.handler == other.handler
            && self.mangled == other.mangled
            && self.fn_ptr == other.fn_ptr
    }
}

impl Eq for JniNativeFnPtr {}

fn jni_text_from_arg(env: *mut c_void, text: *mut c_void) -> String {
    #[cfg(target_os = "android")]
    {
        android_jstring_utf8(env, text)
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = env;
        if text.is_null() {
            return String::new();
        }
        unsafe { std::ffi::CStr::from_ptr(text.cast()) }
            .to_string_lossy()
            .into_owned()
    }
}

fn jni_string_to_java(env: *mut c_void, text: &str) -> *mut c_void {
    #[cfg(target_os = "android")]
    {
        android_new_utf8(env, text)
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = (env, text);
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_dev_gpui_material_NativeInputConnection_nativeCommitText(
    env: *mut c_void,
    _clazz: *mut c_void,
    handle: i64,
    text: *mut c_void,
    new_cursor: i32,
) -> u8 {
    let s = jni_text_from_arg(env, text);
    dispatch_native_peer(handle, "nativeCommitText", Some(&s), new_cursor, 0);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_dev_gpui_material_NativeInputConnection_nativeSetComposingText(
    env: *mut c_void,
    _clazz: *mut c_void,
    handle: i64,
    text: *mut c_void,
    new_cursor: i32,
) -> u8 {
    let s = jni_text_from_arg(env, text);
    dispatch_native_peer(handle, "nativeSetComposingText", Some(&s), new_cursor, 0);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_dev_gpui_material_NativeInputConnection_nativeFinishComposingText(
    _env: *mut c_void,
    _clazz: *mut c_void,
    handle: i64,
) -> u8 {
    dispatch_native_peer(handle, "nativeFinishComposingText", None, 0, 0);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_dev_gpui_material_NativeInputConnection_nativeDeleteSurroundingText(
    _env: *mut c_void,
    _clazz: *mut c_void,
    handle: i64,
    before: i32,
    after: i32,
) -> u8 {
    dispatch_native_peer(handle, "nativeDeleteSurroundingText", None, before, after);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_dev_gpui_material_NativeInputConnection_nativeSetSelection(
    _env: *mut c_void,
    _clazz: *mut c_void,
    handle: i64,
    start: i32,
    end: i32,
) -> u8 {
    dispatch_native_peer(handle, "nativeSetSelection", None, start, end);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_dev_gpui_material_NativeInputConnection_nativeGetTextBeforeCursor(
    env: *mut c_void,
    _clazz: *mut c_void,
    handle: i64,
    n: i32,
) -> *mut c_void {
    let text = dispatch_native_peer(handle, "nativeGetTextBeforeCursor", None, n, 0)
        .unwrap_or_default();
    jni_string_to_java(env, &text)
}

/// Live `JNINativeMethod.fnPtr` table for `RegisterNatives`.
pub fn jni_native_fn_ptr_table() -> &'static [JniNativeFnPtr] {
    &[
        JniNativeFnPtr {
            name: JNI_NATIVE_COMMIT_TEXT.name,
            signature: JNI_NATIVE_COMMIT_TEXT.sig,
            handler: "commitText",
            mangled: "Java_dev_gpui_material_NativeInputConnection_nativeCommitText",
            fn_ptr: Java_dev_gpui_material_NativeInputConnection_nativeCommitText as *const c_void,
        },
        JniNativeFnPtr {
            name: JNI_NATIVE_SET_COMPOSING.name,
            signature: JNI_NATIVE_SET_COMPOSING.sig,
            handler: "setComposingText",
            mangled: "Java_dev_gpui_material_NativeInputConnection_nativeSetComposingText",
            fn_ptr: Java_dev_gpui_material_NativeInputConnection_nativeSetComposingText
                as *const c_void,
        },
        JniNativeFnPtr {
            name: JNI_NATIVE_FINISH_COMPOSING.name,
            signature: JNI_NATIVE_FINISH_COMPOSING.sig,
            handler: "finishComposingText",
            mangled: "Java_dev_gpui_material_NativeInputConnection_nativeFinishComposingText",
            fn_ptr: Java_dev_gpui_material_NativeInputConnection_nativeFinishComposingText
                as *const c_void,
        },
        JniNativeFnPtr {
            name: JNI_NATIVE_DELETE_SURROUNDING.name,
            signature: JNI_NATIVE_DELETE_SURROUNDING.sig,
            handler: "deleteSurroundingText",
            mangled: "Java_dev_gpui_material_NativeInputConnection_nativeDeleteSurroundingText",
            fn_ptr: Java_dev_gpui_material_NativeInputConnection_nativeDeleteSurroundingText
                as *const c_void,
        },
        JniNativeFnPtr {
            name: JNI_NATIVE_SET_SELECTION.name,
            signature: JNI_NATIVE_SET_SELECTION.sig,
            handler: "setSelection",
            mangled: "Java_dev_gpui_material_NativeInputConnection_nativeSetSelection",
            fn_ptr: Java_dev_gpui_material_NativeInputConnection_nativeSetSelection as *const c_void,
        },
        JniNativeFnPtr {
            name: JNI_NATIVE_GET_TEXT_BEFORE.name,
            signature: JNI_NATIVE_GET_TEXT_BEFORE.sig,
            handler: "getTextBeforeCursor",
            mangled: "Java_dev_gpui_material_NativeInputConnection_nativeGetTextBeforeCursor",
            fn_ptr: Java_dev_gpui_material_NativeInputConnection_nativeGetTextBeforeCursor
                as *const c_void,
        },
    ]
}

static SESSION_REGISTRY: Mutex<Option<HashMap<i64, ImeSession>>> = Mutex::new(None);

pub fn bind_session_handle(handle: i64, session: ImeSession) {
    let mut guard = SESSION_REGISTRY.lock().expect("session registry");
    guard.get_or_insert_with(HashMap::new).insert(handle, session);
}

pub fn unbind_session_handle(handle: i64) {
    if let Ok(mut guard) = SESSION_REGISTRY.lock() {
        if let Some(map) = guard.as_mut() {
            map.remove(&handle);
        }
    }
}

pub fn session_handle_text(handle: i64) -> Option<String> {
    let guard = SESSION_REGISTRY.lock().ok()?;
    guard.as_ref()?.get(&handle).map(|s| s.text().to_string())
}

/// Dispatch a Java `native*` method through the bound session handle.
pub fn dispatch_native_peer(
    handle: i64,
    native_name: &str,
    text: Option<&str>,
    a: i32,
    b: i32,
) -> Option<String> {
    let handler = native_peer_handler(native_name)?;
    let mut guard = SESSION_REGISTRY.lock().ok()?;
    let session = guard.as_mut()?.get_mut(&handle)?;
    dispatch_native_input_connection(session, handler, text, a, b)
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
    fn register_natives(&mut self, class: &str, methods: &[JniMethod]) {
        self.find_class(class);
        for method in methods {
            self.get_method_id(class, method.name, method.sig);
        }
        self.call_void("RegisterNatives");
    }
    fn bind_native_fn_ptr(&mut self, mangled: &str, ptr: *const c_void) {
        let _ = (mangled, ptr);
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
    fn bind_native_fn_ptr(&mut self, mangled: &str, _ptr: *const c_void) {
        self.log.push(format!("fnPtr {mangled}"));
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
            JniImmCall::NewNativeInputConnection => {
                sink.new_object(JNI_NATIVE_IC_CTOR.class, JNI_NATIVE_IC_CTOR.sig);
                n += 1;
            }
            JniImmCall::RegisterNatives { .. } => {
                sink.register_natives(NATIVE_IC_CLASS, jni_native_peer_table());
                for row in jni_native_fn_ptr_table() {
                    sink.bind_native_fn_ptr(row.mangled, row.fn_ptr);
                }
                n += 1;
            }
            JniImmCall::NewCursorAnchorInfo { .. } => {
                sink.new_object(JNI_CAI_BUILDER_CTOR.class, JNI_CAI_BUILDER_CTOR.sig);
                sink.get_method_id(
                    JNI_CAI_SET_SELECTION.class,
                    JNI_CAI_SET_SELECTION.name,
                    JNI_CAI_SET_SELECTION.sig,
                );
                sink.call_void(JNI_CAI_SET_SELECTION.name);
                sink.get_method_id(
                    JNI_CAI_SET_INSERTION.class,
                    JNI_CAI_SET_INSERTION.name,
                    JNI_CAI_SET_INSERTION.sig,
                );
                sink.call_void(JNI_CAI_SET_INSERTION.name);
                sink.get_method_id(JNI_CAI_BUILD.class, JNI_CAI_BUILD.name, JNI_CAI_BUILD.sig);
                sink.call_void(JNI_CAI_BUILD.name);
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
fn android_jstring_utf8(env: *mut c_void, text: *mut c_void) -> String {
    use jni_sys::{jstring, JNIEnv};
    if env.is_null() || text.is_null() {
        return String::new();
    }
    let env = env as *mut JNIEnv;
    let jni = unsafe { &(**env).v1_2 };
    let mut is_copy = 0;
    let chars = (jni.GetStringUTFChars)(env, text as jstring, &mut is_copy);
    if chars.is_null() {
        return String::new();
    }
    let s = unsafe { std::ffi::CStr::from_ptr(chars) }
        .to_string_lossy()
        .into_owned();
    (jni.ReleaseStringUTFChars)(env, text as jstring, chars);
    s
}

#[cfg(target_os = "android")]
fn android_new_utf8(env: *mut c_void, text: &str) -> *mut c_void {
    use jni_sys::JNIEnv;
    if env.is_null() {
        return std::ptr::null_mut();
    }
    let env = env as *mut JNIEnv;
    let jni = unsafe { &(**env).v1_2 };
    let c = std::ffi::CString::new(text).unwrap_or_default();
    (jni.NewStringUTF)(env, c.as_ptr()) as *mut c_void
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
    let mut want_native_ic = false;
    let mut want_register = false;
    let mut session_handle = NATIVE_IC_SESSION_HANDLE;
    let mut cai_payload: Option<CursorAnchorInfoPayload> = None;
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
            JniImmCall::NewNativeInputConnection => want_native_ic = true,
            JniImmCall::RegisterNatives { handle } => {
                want_register = true;
                session_handle = *handle;
            }
            JniImmCall::NewCursorAnchorInfo { payload }
            | JniImmCall::UpdateCursorAnchorInfo { payload } => {
                cai_payload = Some(*payload);
            }
            _ => {}
        }
    }
    if !saw_toggle && !want_view && !want_ic && !want_native_ic && !want_register && cai_payload.is_none() {
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
        if want_ic || want_native_ic {
            let mut ic_cls = (jni.FindClass)(
                env,
                b"dev/gpui/material/NativeInputConnection\0".as_ptr().cast(),
            );
            if (jni.ExceptionCheck)(env) || ic_cls.is_null() {
                (jni.ExceptionClear)(env);
                ic_cls = (jni.FindClass)(
                    env,
                    b"android/view/inputmethod/BaseInputConnection\0"
                        .as_ptr()
                        .cast(),
                );
                jni_ok(env)?;
            }
            if ic_cls.is_null() {
                return Err("FindClass InputConnection");
            }
            if want_register {
                // Bind `native*` methods on NativeInputConnection — never BaseIC.
                let mut names = Vec::new();
                let mut sigs = Vec::new();
                for method in jni_native_fn_ptr_table() {
                    names.push(std::ffi::CString::new(method.name).unwrap_or_default());
                    sigs.push(std::ffi::CString::new(method.signature).unwrap_or_default());
                }
                let rows: Vec<jni_sys::JNINativeMethod> = jni_native_fn_ptr_table()
                    .iter()
                    .enumerate()
                    .map(|(i, method)| jni_sys::JNINativeMethod {
                        name: names[i].as_ptr(),
                        signature: sigs[i].as_ptr(),
                        fnPtr: method.fn_ptr as *mut c_void,
                    })
                    .collect();
                let _ = (jni.RegisterNatives)(
                    env,
                    ic_cls,
                    rows.as_ptr(),
                    rows.len() as jni_sys::jint,
                );
                if (jni.ExceptionCheck)(env) {
                    (jni.ExceptionClear)(env);
                }
            }
            let mut ic_ctor = (jni.GetMethodID)(
                env,
                ic_cls,
                b"<init>\0".as_ptr().cast(),
                b"(Landroid/view/View;ZJ)V\0".as_ptr().cast(),
            );
            if (jni.ExceptionCheck)(env) || ic_ctor.is_null() {
                (jni.ExceptionClear)(env);
                ic_ctor = (jni.GetMethodID)(
                    env,
                    ic_cls,
                    b"<init>\0".as_ptr().cast(),
                    b"(Landroid/view/View;Z)V\0".as_ptr().cast(),
                );
                jni_ok(env)?;
                if ic_ctor.is_null() {
                    return Err("InputConnection.<init>");
                }
                let args_ic = [
                    jvalue { l: view },
                    jvalue {
                        z: JNI_TRUE as jboolean,
                    },
                ];
                let _ic = (jni.NewObjectA)(env, ic_cls, ic_ctor, args_ic.as_ptr());
                jni_ok(env)?;
            } else {
                let args_ic = [
                    jvalue { l: view },
                    jvalue {
                        z: JNI_TRUE as jboolean,
                    },
                    jvalue { j: session_handle },
                ];
                let _ic = (jni.NewObjectA)(env, ic_cls, ic_ctor, args_ic.as_ptr());
                jni_ok(env)?;
            }
        }

        if let Some(payload) = cai_payload {
            let b_cls = (jni.FindClass)(
                env,
                b"android/view/inputmethod/CursorAnchorInfo$Builder\0"
                    .as_ptr()
                    .cast(),
            );
            jni_ok(env)?;
            if b_cls.is_null() {
                return Err("FindClass CursorAnchorInfo$Builder");
            }
            let b_ctor = (jni.GetMethodID)(
                env,
                b_cls,
                b"<init>\0".as_ptr().cast(),
                b"()V\0".as_ptr().cast(),
            );
            jni_ok(env)?;
            if b_ctor.is_null() {
                return Err("Builder.<init>");
            }
            let builder = (jni.NewObjectA)(env, b_cls, b_ctor, std::ptr::null());
            jni_ok(env)?;
            if builder.is_null() {
                return Err("new Builder");
            }
            let set_sel = (jni.GetMethodID)(
                env,
                b_cls,
                b"setSelectionRange\0".as_ptr().cast(),
                b"(II)Landroid/view/inputmethod/CursorAnchorInfo$Builder;\0"
                    .as_ptr()
                    .cast(),
            );
            jni_ok(env)?;
            let args_sel = [
                jvalue {
                    i: payload.selection_start,
                },
                jvalue {
                    i: payload.selection_end,
                },
            ];
            let _ = (jni.CallObjectMethodA)(env, builder, set_sel, args_sel.as_ptr());
            jni_ok(env)?;
            let set_ins = (jni.GetMethodID)(
                env,
                b_cls,
                b"setInsertionMarkerLocation\0".as_ptr().cast(),
                b"(FFFFI)Landroid/view/inputmethod/CursorAnchorInfo$Builder;\0"
                    .as_ptr()
                    .cast(),
            );
            jni_ok(env)?;
            let [l, t, _r, btm] = payload.insertion_marker;
            let args_ins = [
                jvalue { f: l },
                jvalue { f: t },
                jvalue { f: t + (btm - t) * 0.8 },
                jvalue { f: btm },
                jvalue {
                    i: CAI_FLAG_HAS_VISIBLE_REGION,
                },
            ];
            let _ = (jni.CallObjectMethodA)(env, builder, set_ins, args_ins.as_ptr());
            jni_ok(env)?;
            let build = (jni.GetMethodID)(
                env,
                b_cls,
                b"build\0".as_ptr().cast(),
                b"()Landroid/view/inputmethod/CursorAnchorInfo;\0"
                    .as_ptr()
                    .cast(),
            );
            jni_ok(env)?;
            let cai = (jni.CallObjectMethodA)(env, builder, build, std::ptr::null());
            jni_ok(env)?;
            if cai.is_null() {
                return Err("CursorAnchorInfo.build");
            }
            let update = (jni.GetMethodID)(
                env,
                imm_cls,
                b"updateCursorAnchorInfo\0".as_ptr().cast(),
                b"(Landroid/view/View;Landroid/view/inputmethod/CursorAnchorInfo;)V\0"
                    .as_ptr()
                    .cast(),
            );
            jni_ok(env)?;
            if !update.is_null() {
                let args_upd = [jvalue { l: view }, jvalue { l: cai }];
                (jni.CallVoidMethodA)(env, imm, update, args_upd.as_ptr());
                jni_ok(env)?;
            }
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

/// Catalog-focused caret → session + IMM queue (GPUI `Window` has no caret API).
/// `rect` is window-space `(x, y, w, h)` from `text_field::catalog_ime_from_editor`.
pub fn apply_catalog_editor_caret(
    slot: &Cell<Option<ImeBoundsDp>>,
    session: &std::cell::RefCell<ImeSession>,
    queue: &std::cell::RefCell<ImeJniQueue>,
    rect: (f32, f32, f32, f32),
) {
    apply_update_ime_position_queued(slot, session, queue, rect.0, rect.1, rect.2, rect.3);
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
        assert!(matches!(
            calls[3],
            JniImmCall::RegisterNatives {
                handle: NATIVE_IC_SESSION_HANDLE
            }
        ));
        assert!(matches!(calls[4], JniImmCall::NewNativeInputConnection));
        match &calls[5] {
            JniImmCall::NewCursorAnchorInfo { payload } => {
                assert_eq!(payload.insertion_marker, [8.0, 16.0, 10.0, 40.0]);
                assert_eq!(payload.selection_end, 2);
            }
            other => panic!("expected CursorAnchorInfo builder, got {other:?}"),
        }
        match &calls[6] {
            JniImmCall::UpdateCursorAnchorInfo { payload } => {
                assert_eq!(payload.insertion_marker, [8.0, 16.0, 10.0, 40.0]);
            }
            other => panic!("expected updateCursorAnchorInfo, got {other:?}"),
        }
    }

    #[test]
    fn jni_queue_and_register_natives_after_caret() {
        let _attach = ATTACH_TEST_LOCK.lock().unwrap();
        unsafe { detach_native_activity() };
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
                "RegisterNatives",
                "NativeInputConnection",
                "CursorAnchorInfo$Builder",
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
        assert_eq!(n, 7);
        assert!(sink.log.iter().any(|s| s.contains("toggleSoftInput")));
        assert!(sink.log.iter().any(|s| s.contains("android/view/View")));
        assert!(
            sink.log
                .iter()
                .any(|s| s.contains("dev/gpui/material/NativeInputConnection"))
        );
        assert!(
            sink.log
                .iter()
                .any(|s| s.contains("CursorAnchorInfo$Builder"))
        );
        assert!(sink.log.iter().any(|s| s.contains("setInsertionMarkerLocation")));
        assert_eq!(NATIVE_IC_HAS_CODE, true);
        assert_eq!(JNI_CAI_SET_SELECTION.name, "setSelectionRange");
        assert!(
            jni_native_method_descriptors()
                .iter()
                .any(|m| m.name == "commitText" && m.handler == "commitText")
        );
        assert!(
            jni_native_peer_descriptors()
                .iter()
                .any(|m| m.name == "nativeCommitText" && m.handler == "commitText")
        );
        assert_eq!(JNI_NATIVE_IC_CTOR.sig, "(Landroid/view/View;ZJ)V");
        assert_eq!(JNI_NATIVE_COMMIT_TEXT.class, NATIVE_IC_CLASS);
        assert_eq!(native_peer_handler("commitText"), None);
        assert_eq!(
            jni_mangled_name("nativeCommitText"),
            "Java_dev_gpui_material_NativeInputConnection_nativeCommitText"
        );
        assert!(
            jni_native_fn_ptr_table()
                .iter()
                .all(|m| !m.fn_ptr.is_null() && m.mangled.starts_with(JNI_MANGLED_PREFIX))
        );
        bind_session_handle(NATIVE_IC_SESSION_HANDLE, ImeSession::new());
        dispatch_native_peer(
            NATIVE_IC_SESSION_HANDLE,
            "nativeCommitText",
            Some("peer"),
            1,
            0,
        );
        assert_eq!(
            session_handle_text(NATIVE_IC_SESSION_HANDLE).as_deref(),
            Some("peer")
        );
        unbind_session_handle(NATIVE_IC_SESSION_HANDLE);
        bind_session_handle(NATIVE_IC_SESSION_HANDLE, ImeSession::new());
        let payload = b"fnPtr\0";
        let ok = Java_dev_gpui_material_NativeInputConnection_nativeCommitText(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            NATIVE_IC_SESSION_HANDLE,
            payload.as_ptr() as *mut c_void,
            1,
        );
        assert_eq!(ok, 1);
        assert_eq!(
            session_handle_text(NATIVE_IC_SESSION_HANDLE).as_deref(),
            Some("fnPtr")
        );
        unbind_session_handle(NATIVE_IC_SESSION_HANDLE);
        assert!(sink.log.iter().any(|s| s.contains("RegisterNatives")));
        assert!(sink.log.iter().any(|s| s.contains("nativeCommitText")));
        assert!(sink.log.iter().any(|s| s.contains("fnPtr Java_dev_gpui_material_NativeInputConnection_nativeCommitText")));
        assert_eq!(flush_if_attached(&queue.borrow()), Err("no JNIEnv"));
        unsafe { attach_jni_env(0x1 as *mut _) };
        let _clear = scopeguard_clear_env();
        assert_eq!(flush_if_attached(&queue.borrow()), Ok(7));
    }

    #[test]
    fn native_activity_attach_dry_runs_imm_flush() {
        let _attach = ATTACH_TEST_LOCK.lock().unwrap();
        unsafe { detach_native_activity() };
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
        assert_eq!(flush_native_activity_imm(&queue.borrow()), Ok(7));
        assert_eq!(flush_if_attached(&queue.borrow()), Ok(7));
        let slot2 = Cell::new(None);
        let session2 = RefCell::new(ImeSession::new());
        let queue2 = RefCell::new(ImeJniQueue::new());
        apply_catalog_editor_caret(&slot2, &session2, &queue2, (24.0, 32.0, 2.0, 24.0));
        assert_eq!(last_ime_position(&slot2), Some([24.0, 32.0, 2.0, 24.0]));
    }

    static ATTACH_TEST_LOCK: Mutex<()> = Mutex::new(());

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

package dev.gpui.material;

import android.view.View;
import android.view.inputmethod.BaseInputConnection;

/**
 * NativeActivity IME peer. {@code FindClass("dev/gpui/material/NativeInputConnection")}
 * succeeds when the application is packaged with {@code hasCode=true}.
 *
 * <p>{@code RegisterNatives} binds the {@code native*} methods on this class — not
 * {@link BaseInputConnection} overrides, which are not {@code native}. Java
 * {@code commitText}/{@code setComposingText} forward to those natives when a
 * session handle is set.
 */
public final class NativeInputConnection extends BaseInputConnection {
    private final long sessionHandle;

    public NativeInputConnection(View view, boolean fullEditor) {
        this(view, fullEditor, 0L);
    }

    public NativeInputConnection(View view, boolean fullEditor, long sessionHandle) {
        super(view, fullEditor);
        this.sessionHandle = sessionHandle;
    }

    public long sessionHandle() {
        return sessionHandle;
    }

    @Override
    public boolean commitText(CharSequence text, int newCursorPosition) {
        if (sessionHandle != 0
                && nativeCommitText(
                        sessionHandle, text == null ? "" : text.toString(), newCursorPosition)) {
            return true;
        }
        return super.commitText(text, newCursorPosition);
    }

    @Override
    public boolean setComposingText(CharSequence text, int newCursorPosition) {
        if (sessionHandle != 0
                && nativeSetComposingText(
                        sessionHandle, text == null ? "" : text.toString(), newCursorPosition)) {
            return true;
        }
        return super.setComposingText(text, newCursorPosition);
    }

    @Override
    public boolean finishComposingText() {
        if (sessionHandle != 0 && nativeFinishComposingText(sessionHandle)) {
            return true;
        }
        return super.finishComposingText();
    }

    @Override
    public boolean deleteSurroundingText(int beforeLength, int afterLength) {
        if (sessionHandle != 0
                && nativeDeleteSurroundingText(sessionHandle, beforeLength, afterLength)) {
            return true;
        }
        return super.deleteSurroundingText(beforeLength, afterLength);
    }

    @Override
    public boolean setSelection(int start, int end) {
        if (sessionHandle != 0 && nativeSetSelection(sessionHandle, start, end)) {
            return true;
        }
        return super.setSelection(start, end);
    }

    static native boolean nativeCommitText(long sessionHandle, String text, int newCursorPosition);

    static native boolean nativeSetComposingText(
            long sessionHandle, String text, int newCursorPosition);

    static native boolean nativeFinishComposingText(long sessionHandle);

    static native boolean nativeDeleteSurroundingText(
            long sessionHandle, int beforeLength, int afterLength);

    static native boolean nativeSetSelection(long sessionHandle, int start, int end);

    static native String nativeGetTextBeforeCursor(long sessionHandle, int n);
}

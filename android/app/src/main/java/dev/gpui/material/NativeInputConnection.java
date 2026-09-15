package dev.gpui.material;

import android.view.View;
import android.view.inputmethod.BaseInputConnection;

/**
 * NativeActivity IME peer. {@code FindClass("dev/gpui/material/NativeInputConnection")}
 * succeeds when the application is packaged with {@code hasCode=true}.
 *
 * <p>InputConnection methods stay on {@link BaseInputConnection}. The Rust
 * {@code ImeSession} records an equivalent JNI plan; {@code RegisterNatives}
 * still needs matching {@code native} methods before a live bind.
 */
public final class NativeInputConnection extends BaseInputConnection {
    public NativeInputConnection(View view, boolean fullEditor) {
        super(view, fullEditor);
    }
}

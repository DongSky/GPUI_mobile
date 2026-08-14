#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
if ! adb get-state >/dev/null 2>&1; then
    echo "starting emulator $AVD_NAME ..."
    nohup emulator -avd "$AVD_NAME" -netdelay none -netspeed full >/tmp/emulator.log 2>&1 &
    adb wait-for-device
fi
until [ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; do
    sleep 2
done
adb install -r android/app/build/outputs/apk/debug/app-debug.apk
adb logcat -c
adb shell am start -n "$APP_ID/android.app.NativeActivity"

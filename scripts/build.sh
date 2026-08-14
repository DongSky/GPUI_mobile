#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
rm -rf android/app/src/main/jniLibs
cargo ndk -t arm64-v8a --platform 30 -o android/app/src/main/jniLibs build -p "${DEMO_CRATE:-hello_gpui}" --release
(cd android && "$GRADLE_BIN" assembleDebug)
echo "APK: android/app/build/outputs/apk/debug/app-debug.apk"

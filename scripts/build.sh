#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh

if [[ -z "${ANDROID_HOME:-}" || ! -d "${ANDROID_HOME}" ]]; then
  echo "ANDROID_HOME is not set or missing. Run scripts/setup-android-sdk.sh first." >&2
  exit 1
fi
if [[ -z "${GRADLE_BIN:-}" ]]; then
  echo "No Gradle binary found. Add android/gradlew or install Gradle 8.14.x." >&2
  exit 1
fi
if ! command -v cargo-ndk >/dev/null 2>&1 && ! command -v cargo >/dev/null; then
  echo "cargo is required" >&2
  exit 1
fi
if ! command -v cargo-ndk >/dev/null 2>&1; then
  echo "cargo-ndk is required: cargo install cargo-ndk" >&2
  exit 1
fi

printf 'sdk.dir=%s\n' "${ANDROID_HOME}" > android/local.properties

rm -rf android/app/src/main/jniLibs
cargo ndk -t arm64-v8a --platform 30 -o android/app/src/main/jniLibs build -p "${DEMO_CRATE:-component_demo}" --release
(cd android && "${GRADLE_BIN}" assembleDebug)
echo "APK: android/app/build/outputs/apk/debug/app-debug.apk"

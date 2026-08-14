#!/bin/bash
cd "$(dirname "$0")/.."
source scripts/env.sh
adb logcat -d -v time | grep -E "hello_gpui|RustStdoutStderr|wgpu|gpui|AndroidRuntime|libc.*Fatal" || true

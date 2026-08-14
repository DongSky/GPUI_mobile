#!/bin/bash
cd "$(dirname "$0")/.."
source scripts/env.sh
adb logcat -d -v time | grep -E "hello_gpui|component_demo|PANIC|RustStdoutStderr|wgpu|gpui|AndroidRuntime|libc.*Fatal" || true
# grep 无匹配时 exit 1 是预期，不视为失败

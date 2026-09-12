#!/bin/bash
# Install a Linux/macOS Android SDK sufficient to assemble the debug APK.
# Safe to re-run. Requires network + Java 17+.
set -euo pipefail

SDK_ROOT="${ANDROID_HOME:-${HOME}/Android/Sdk}"
CMDLINE_VERSION="${CMDLINE_VERSION:-13114758}"
PACKAGES=(
  "platform-tools"
  "platforms;android-35"
  "build-tools;35.0.0"
  "ndk;25.1.8937393"
)

mkdir -p "${SDK_ROOT}/cmdline-tools"
if [[ ! -x "${SDK_ROOT}/cmdline-tools/latest/bin/sdkmanager" ]]; then
  os="$(uname -s)"
  case "${os}" in
    Linux) zip="commandlinetools-linux-${CMDLINE_VERSION}_latest.zip" ;;
    Darwin) zip="commandlinetools-darwin-${CMDLINE_VERSION}_latest.zip" ;;
    *) echo "unsupported OS ${os}" >&2; exit 1 ;;
  esac
  tmp="$(mktemp -d)"
  curl -fsSL "https://dl.google.com/android/repository/${zip}" -o "${tmp}/tools.zip"
  unzip -q "${tmp}/tools.zip" -d "${tmp}"
  rm -rf "${SDK_ROOT}/cmdline-tools/latest"
  mkdir -p "${SDK_ROOT}/cmdline-tools/latest"
  # The zip contains a top-level cmdline-tools/ directory.
  mv "${tmp}/cmdline-tools/"* "${SDK_ROOT}/cmdline-tools/latest/"
  rm -rf "${tmp}"
fi

# `yes` exits 141 (SIGPIPE) when sdkmanager closes stdin; do not fail the script.
set +o pipefail
yes | "${SDK_ROOT}/cmdline-tools/latest/bin/sdkmanager" --sdk_root="${SDK_ROOT}" --licenses >/dev/null
set -o pipefail
"${SDK_ROOT}/cmdline-tools/latest/bin/sdkmanager" --sdk_root="${SDK_ROOT}" "${PACKAGES[@]}"

echo "ANDROID_HOME=${SDK_ROOT}"
echo "Installed: ${PACKAGES[*]}"

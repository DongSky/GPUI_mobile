#!/bin/bash
# Shared build/deploy environment. Usage: source scripts/env.sh
#
# Overrides (all optional):
#   ANDROID_HOME, ANDROID_NDK_HOME, JAVA_HOME, GRADLE_BIN, AVD_NAME, APP_ID, DEMO_CRATE

if [[ "${OSTYPE:-}" == darwin* ]]; then
  : "${ANDROID_HOME:=$HOME/Library/Android/sdk}"
  if [[ -z "${JAVA_HOME:-}" && -d "/Applications/Android Studio.app/Contents/jbr/Contents/Home" ]]; then
    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
  fi
else
  : "${ANDROID_HOME:=${HOME}/Android/Sdk}"
  if [[ -z "${JAVA_HOME:-}" ]] && command -v java >/dev/null 2>&1; then
    _java_bin="$(readlink -f "$(command -v java)")"
    export JAVA_HOME="$(dirname "$(dirname "$_java_bin")")"
  fi
fi

export ANDROID_HOME
if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
  if [[ -d "${ANDROID_HOME}/ndk/25.1.8937393" ]]; then
    export ANDROID_NDK_HOME="${ANDROID_HOME}/ndk/25.1.8937393"
  else
    _ndk_latest="$(ls -1d "${ANDROID_HOME}/ndk/"* 2>/dev/null | sort -V | tail -n 1 || true)"
    if [[ -n "${_ndk_latest}" ]]; then
      export ANDROID_NDK_HOME="${_ndk_latest}"
    fi
  fi
fi

export PATH="${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/emulator:${PATH}"

if [[ -z "${GRADLE_BIN:-}" ]]; then
  _here="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" 2>/dev/null && pwd || true)"
  if [[ -n "${_here}" && -x "${_here}/../android/gradlew" ]]; then
    export GRADLE_BIN="$(cd "${_here}/.." && pwd)/android/gradlew"
  elif [[ -x "${PWD}/android/gradlew" ]]; then
    export GRADLE_BIN="${PWD}/android/gradlew"
  else
    _gradle_glob=("${HOME}/.gradle/wrapper/dists/gradle-8.14.3-bin"/*/gradle-8.14.3/bin/gradle)
    if [[ -x "${_gradle_glob[0]:-}" ]]; then
      export GRADLE_BIN="${_gradle_glob[0]}"
    elif command -v gradle >/dev/null 2>&1; then
      export GRADLE_BIN="$(command -v gradle)"
    fi
  fi
fi

export AVD_NAME="${AVD_NAME:-Medium_Phone_API_35}"
export APP_ID="${APP_ID:-dev.youkai.hellogpui}"
export DEMO_CRATE="${DEMO_CRATE:-component_demo}"

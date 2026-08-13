#!/bin/bash
# 构建/部署脚本共用环境。用法: source scripts/env.sh
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/25.1.8937393"
export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
_gradle_glob=("$HOME/.gradle/wrapper/dists/gradle-8.14.3-bin"/*/gradle-8.14.3/bin/gradle)
export GRADLE_BIN="${_gradle_glob[0]}"
export AVD_NAME="Medium_Phone_API_35"
export APP_ID="dev.youkai.hellogpui"

ANDROID_NDK_HOME=~/downloads/sdk-tools/ndk-bundle  cargo ndk --platform 21 --target armv7-linux-androideabi  build --release
ANDROID_NDK_HOME=~/downloads/sdk-tools/ndk-bundle  cargo ndk --platform 21 --target aarch64-linux-android  build --release
ANDROID_NDK_HOME=~/downloads/sdk-tools/ndk-bundle  cargo ndk --platform 21 --target x86_64-linux-android  build --release
ANDROID_NDK_HOME=~/downloads/sdk-tools/ndk-bundle  cargo ndk --platform 21 --target i686-linux-android  build --release
cp .target/aarch64-linux-android/release/libhanzi_lookup.so ~/projects/chinese_character_recognition/android/src/main/jniLibs/arm64-v8a/
cp .target/armv7-linux-androideabi/release/libhanzi_lookup.so ~/projects/chinese_character_recognition/android/src/main/jniLibs/armeabi-v7a/
cp .target/x86_64-linux-android/release/libhanzi_lookup.so ~/projects/chinese_character_recognition/android/src/main/jniLibs/x86_64/
cp .target/i686-linux-android/release/libhanzi_lookup.so ~/projects/chinese_character_recognition/android/src/main/jniLibs/x86/



# Crypto betting app

SDD: <https://docs.google.com/document/d/1U1PDZC_sZY5XeCc71WyDy0mWSrCJs-WU7Pba7gEZzaA/edit>

## Project structure

- `front` - the frontend part of the project. It is a [vite] JS+Rust(WASM) app.
- `app` - an [Apache Cordova] app that wraps the frontend part into a mobile app.

## Initial setup

*TODO: consider preparing a Docker image with the development environment*.

### Initial setup: Picking the low hanging fruits

- Navigate to the directory where you want to keep the directory with the project:

```console
cd .../path/to/your/directory
```

- Ensure you have [`git`](https://git-scm.com/) installed:

```console
git --version
```

- Clone the project:

```console
git clone https://github.com/JohnScience/betting_app
```

- Navigate to the directory with the project:

```console
cd betting_app
```

- Ensure you have [`npm`](https://www.npmjs.com/) installed:

```console
npm --version
```

- Install the JS dependencies for both the frontend and the app:

```console
cd app && npm install && cd ../front && npm install
```

- Ensure you have [Rust](https://www.rust-lang.org/tools/install) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.

```console
rustc --version
wasm-pack --version
```

- Ensure you have [Android Studio](https://developer.android.com/studio) installed.

- Ensure your `ANDROID_HOME` environment variable is set to the path to your Android SDK directory, e.g. `C:\Users\USER\AppData\Local\Android\Sdk`.

```console
# On Windows
echo %ANDROID_HOME%
# On Linux/MacOS
echo $ANDROID_HOME
```

- Ensure that you have the Android tools in your `PATH` environment variable:

  - `ANDROID_HOME/tools`,
  - `ANDROID_HOME/tools/bin`,
  - `ANDROID_HOME/platform-tools`.

### Initial setup: JDK & Gradle

Satisfying Gradle's [Compatibility Matrix](https://docs.gradle.org/current/userguide/compatibility.html) can be difficult because, at the time of writing this, it paints a too optimistic picture. The latest Gradle (8.3) doesn't support the latest JDK version (20). You can use the tools that works for you but the following setup is known to work:

- Ensure you have JDK of version `Amazon Corretto 19.0.2` installed. To do that, you can open Android Studio and navigate to File -> Settings -> Build, Execution, Deployment -> Build Tools -> Gradle -> Gradle JDK. [Screenshot](https://i.imgur.com/DPucauu.png). If there's no menu where you can pick a Gradle JDK, you might need to create a new dummy Android project first.

- Add the path to `Amazon Corretto 19.0.2` JDK (e.g. `C:\Users\USER\.jdks\corretto-19.0.2`) to `JAVA_HOME` environment variable.

```console
# On Windows
echo %JAVA_HOME%
# On Linux/MacOS
echo $JAVA_HOME
```

- Add `JAVA_HOME/bin` to your `PATH` environment variable.

```console
javac --version
```

- Ensure you have Gradle 7.6 installed at `<...>/.jdks/../.gradle/wrapper/dists`. The directory is likely to be called `gradle-7.6-all`. If it is not there yet, you can install it from <https://gradle.org/releases/> and put there (for convenience).

- Ensure you set `GRADLE_HOME` to the path of your gradle binaries (e.g. `C:\Users\USER\.gradle\wrapper\dists\gradle-7.6-all\9f832ih6bniajn45pbmqhk2cw\gradle-7.6`). Note that the structure of the directory that was produced by Android Studio also contains a directory whose name is a hash.

- Add `GRADLE_HOME/bin` to your `PATH` environment variable.

```console
gradle --version
```

### Initial setup: The final touches

Picking the right version of Android SDK and NDK can also be tricky. The following setup is known to work:

- Open Android Studio and navigate to Tools -> SDK Manager -> Languages & Frameworks -> Android SDK -> SDK Platforms. Even though Cordova currently imposes [`android:minSdkVersion`](https://developer.android.com/guide/topics/manifest/uses-sdk-element#min) of `24` and [`android:targetSdkVersion`](https://developer.android.com/guide/topics/manifest/uses-sdk-element#target) of `33`, you have to ensure that only `Android SDK platform 33` under `Android 13.0 ("Tiramisu")` is checked. [Screenshot](https://i.imgur.com/aTA1bTC.png).

- Change the tab from `SDK Platforms` to `SDK Tools` and check `NDK (Side by side)` of version `25.0.8775105`. *TODO: consider choosing a newer version of NDK.*

- Click `Apply` and wait for the SDK and NDK to be installed.

- Set `NDK_HOME` environment variable as `ANDROID_HOME/ndk/25.0.8775105`.

```console
# On Windows
echo %NDK_HOME%
# On Linux/MacOS
echo $NDK_HOME
```

## Intended workflow

Change directory to the `front` where nearly all the work will be done:

```console
cd front
```

Assuming you have the dependencies installed, you can tweak the frontend part and see the changes.

```console
npm run dev
```

Once you are happy with the changes, you can build the frontend part for testing it on a mobile device:

```console
npm run test-on-<android|ios>
```

*TODO: add iOS target, possibly via [cordova-ios], and check if it works as intended.*

*Note: testing on a mobile device takes more than a minute.*

[vite]: https://vitejs.dev/
[Apache Cordova]: https://cordova.apache.org/
[cordova-ios]: https://github.com/apache/cordova-ios

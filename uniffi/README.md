This file is out of date

# Usage

- Setup Rust + NDK using [this guide](https://sal.dev/android/intro-rust-android-uniffi/) (step 1).
- Comment out these lines in Cargo.toml
```
#[[bin]]
#name = "uniffi-bindgen"
#path = "uniffi-bindgen.rs"
```
- `cd src/main/rust`
- Generate Rust libraries using this command (you can omit --release, but the debug libraries will be very big).

```cargo build --release```


- Uncomment the following code in Cargo.toml (to avoid this step, we'll need to move `uniffi-bindgen` to [its own crate](https://mozilla.github.io/uniffi-rs/tutorial/foreign_language_bindings.html#multi-crate-workspaces) later):

```
#[[bin]]
#name = "uniffi-bindgen"
#path = "uniffi-bindgen.rs"
```

- Build this module using the normal methods

Example of using the bindings:

`Log.i("DPP", "systemIds: ${systemIds()}")`

# Possible issues

```is aarch64-linux-android-ar installed?```

Can be solved by making copies of `llvm-ar` in the NDK folder as `aarch64-linux-android-ar` and other architectures: https://stackoverflow.com/questions/69945638/do-rust-and-cargo-1-56-simply-not-work-with-android-ndk-r23-without-binutils

```i686-linux-android-clang: error: not ranlib, ar, lib or dlltool```

Similary to above, copy `i686-linux-android23-clang` as `i686-linux-android-clang`

`i686-linux-android21-clang failed to execute` - remove lower API binaries from the toolchain folder.

There might be more elegant solutions to these issues.

# clamav-sys

clamav-sys is a minimal Rust interface around [libclamav](https://github.com/Cisco-Talos/clamav).
This package is not supposed to be used stand-alone, but only through a safe
wrapper such as [clamav-async-rs](https://github.com/Cisco-Talos/clamav-async-rs).

## Building

To compile and run `clamav-sys` directly or as a dependency of `clamav-async-rs`
or a downstream application, you need:
- `libclamav` and its development headers
- OpenSSL development headers

### How discovery works

`clamav-sys` looks for dependencies in this order:

1. If `CLAMAV_LIBRARY` and `CLAMAV_INCLUDE` are set, those paths are used.
2. Otherwise, `pkg-config` is used on Unix, Linux, and macOS.
3. Otherwise, `vcpkg` is used on Windows.

If `OPENSSL_INCLUDE` is set, it is added to the header search path for bindgen.

### Override variables

- `CLAMAV_LIBRARY`: Exact path to the ClamAV library file, such as
  `libclamav.so`, `libclamav.dylib`, `libclamav.a`, or `clamav.lib`
- `CLAMAV_INCLUDE`: One or more include directories containing `clamav.h`
- `OPENSSL_INCLUDE`: Include directory containing `openssl/ssl.h`
- `CLAMAV_STATIC`: On Windows, set to `1` to link `CLAMAV_LIBRARY` as a static
  library when it points to a `.lib` file

### Linux / Unix / macOS

Install the ClamAV development package and OpenSSL headers. If they are
installed in standard system locations, `pkg-config` is usually sufficient.

If ClamAV is installed in a non-standard location, set `CLAMAV_LIBRARY` and
`CLAMAV_INCLUDE`. Set `OPENSSL_INCLUDE` if OpenSSL headers are also outside the
default include paths.

Example in `sh`:
```sh
export CLAMAV_INCLUDE="$HOME/clams/1.5.2/include"
export CLAMAV_LIBRARY="$HOME/clams/1.5.2/lib/libclamav.so"
export OPENSSL_INCLUDE="$HOME/.mussels/install/host-static/include"
export LD_LIBRARY_PATH="$HOME/clams/1.5.2/lib"
cargo test
```

### macOS using Homebrew

Install the dependencies:
```sh
brew install clamav openssl@3
```

Then set:
```sh
export OPENSSL_INCLUDE=/opt/homebrew/opt/openssl@3/include
```

### Windows using vcpkg

If you use `vcpkg`, set `$env:VCPKG_ROOT` to your `vcpkg` installation and set
`$env:VCPKGRS_DYNAMIC=1` to use dynamic linking. The default method often does
not work because `pdcurses` does not support the `x64-windows-static-md`
triplet.

Install LLVM for `bindgen`, which requires `libclang` on Windows. Set
`$env:LIBCLANG_PATH` to the directory containing `libclang.dll` or `clang.dll`.
For example:
```ps1
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

See the [vcpkg crate's documentation](https://docs.rs/vcpkg) for more details.

> _Gotchas_:
>
> Windows has its own version of a zlib DLL that is incompatible with vcpkg.
>
> If building from PowerShell, the process may just hang without showing any
> output.
> If building from `cmd.exe`, a dialog box may pop up with this error message:
> > "The procedure entry point gzdirect could not be located in the dynamic link
> library"
>
> If this happens, you will want to make sure that the vcpkg dynamic libraries
> in your `PATH` variable precede the Windows one.
> ```ps1
> $env:PATH = "$env:VCPKG_ROOT\installed\x64-windows\bin\;$env:PATH"
> ```

### Windows without vcpkg

If not using vcpkg, you must use the override variables described above so that
`cargo build` can find the required files.

You must also install LLVM for `bindgen`, which requires `libclang` on
Windows. Set `LIBCLANG_PATH` to the directory containing `libclang.dll` or
`clang.dll`.

If `CLAMAV_LIBRARY` points to a static `.lib`, also set
`$env:CLAMAV_STATIC = "1"`.

Example in PowerShell:
```powershell
$env:CLAMAV_LIBRARY = "$env:HOME\Downloads\clamav-1.5.2\clamav.lib"
$env:CLAMAV_INCLUDE = "$env:HOME\Downloads\clamav-1.5.2\include"
$env:CLAMAV_STATIC = "1"
$env:OPENSSL_INCLUDE = "$env:HOME\Downloads\openssl\include"
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
$env:PATH = "$env:HOME\Downloads\clamav-1.5.2;$env:PATH"
```

## Versioning

The version number of `libclamav-sys` tracks ClamAV's version number. That is,
you'll require at least ClamAV 1.0.0 to build `libclamav-sys` 1.0.0. As ClamAV
usually doesn't do breaking API changes, you'll be able to use `libclamav-sys`
with newer ClamAV versions.

No attempt at preserving downward compatibility (using a `libclamav-sys` with
a version number greater than ClamAV's) is made.

# README

## Usage

### Note on Rundll32

Rundll32 will attach the DLL before it does anything else. 
This means that at minimum we can expect the following functions to be called:

- `DllMain()`
- function which handles `DLL_PROCESS_ATTACH` event
- function which handles `DLL_PROCESS_DETACH` event


### Simulate an Exe Importing the DLL

```sh
rundll32.exe .\target\debug\hello.dll,DllMain
```

```sh
cargo run
```

### Call a Symbol Exported in DLL

```sh
rundle32.exe .\target\debug\hello.dll,hello
```

## References

- [hinstance location](https://github.com/rust-lang/rust/issues/84981)
- https://chuongdong.com/malware%20development/2020/06/09/rust-ransomware1/
- [winapi showWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
- [win errors enum ?](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-)
- [winapi vs windows-rs](https://medium.com/@SecSamDev/rustylib-wars-winapi-vs-windows-rs-9b503049460)


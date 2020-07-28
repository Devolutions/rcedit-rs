# rcedit-rs
This library provides Windows PE resource patching possibilities. Currently, it can be used
only to patch the main application icon, language-independent resource strings, and RCDATA.
Works only on the Windows platform.

It is based on the
[electron/rcedit](https://github.com/electron/rcedit) C++ application code, while introducing a few
additional changes:
- ATL libraries are not required anymore, so library code can be built using VS Build Tools
- Rcedit CLI application code was removed - it is not needed for the library crate
- Added C shim for Rust <-> C++ FFI

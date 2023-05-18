# Wasdon

Yet another WASM-compatible layer for UdonVM (to run WASM/WASI binary on VRChat).

## Current status

This is working in progress and not ready for use yet.

It's currently focused on developing an **AOT binary translator** to Udon Assembly (its text representation) to support full compatibility with WASI/WASM.

## Roadmap

- [ ] WASM support

- [ ] WASI support

- [ ] Add runtime support
  - This feature will require the AOT translator of WASI because it will provide a WASI executable as a runtime.

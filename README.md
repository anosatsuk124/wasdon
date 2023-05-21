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

## LICENSE

This project is licensed under the Apache 2.0 license with the LLVM exception. See [LICENSE](LICENSE) for more details.

```
    Copyright 2023- Satsuki Akiba <anosatsuk124@gmail.com>

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
```

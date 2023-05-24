# Linear Memory Conversion strategy

## Problem

In WebAssembly, there's a linear memory that is used to store all variables. It's a big array of bytes. In Udon, there's no way to represent linear memory because Udon doesn't provide any methods to store values as bytes array.

## Solution

In UdonVM, all types can treat as a `object (SystemObject)`. So the solution is to represent linear memory as a `object[]`.

## Example

```uasm
.data_start
  _stack: %SystemObjectArray, null
  _stack_length: %SystemInt32, 10
  _a: %SystemInt32, 0
  _0_index: %SystemInt32, 0
  _result: %SystemObject, null
.data_end

.code_start
  .export _start
  _start:
    PUSH, _stack_length
    PUSH, _stack
    EXTERN, "SystemObjectArray.__ctor__SystemInt32__SystemObjectArray"
    PUSH, _stack
    PUSH, _a
    PUSH, _0_index
    EXTERN, "SystemObjectArray.__SetValue__SystemObject_SystemInt32__SystemVoid"
    PUSH, _stack
    PUSH, _0_index
    PUSH, _result
    EXTERN, "SystemObjectArray.__GetValue__SystemInt32__SystemObject"
    PUSH, _result
    EXTERN, "UnityEngineDebug.__Log__SystemObject__SystemVoid"
.code_end
```

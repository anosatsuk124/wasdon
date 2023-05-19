(module
  (type (;0;) (func (param i32)))
  (func $hello (;0;) (type 0) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    i32.const 1048576
    local.set 4
    local.get 3
    local.get 4
    i32.store offset=8
    i32.const 13
    local.set 5
    local.get 3
    local.get 5
    i32.store offset=12
    i32.const 42
    local.set 6
    local.get 3
    local.get 6
    i32.store
    local.get 3
    local.get 4
    i32.store offset=4
    local.get 3
    i32.load
    local.set 7
    local.get 3
    i32.load offset=4
    local.set 8
    local.get 0
    local.get 8
    i32.store offset=4
    local.get 0
    local.get 7
    i32.store
    return
  )
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048589)
  (global (;2;) i32 i32.const 1048592)
  (export "memory" (memory 0))
  (export "hello" (func $hello))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data $.rodata (;0;) (i32.const 1048576) "Hello, World!")
)
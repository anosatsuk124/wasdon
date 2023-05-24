(;
 #[repr(C)]
 struct Hello {
  a: i32,
  b: i32,
 }
 Hello { a: 1, b: 2 }
;)

(module
  (memory 1)  
  (global $stack (mut i32) (i32.const 0))
  (func $hello (;0;)
    global.get $stack
    i32.const 1
    i32.store
    global.get $stack
    i32.const 2
    i32.store offset=4
    return
  )
  (func $get_hello_a (;1;) (result i32)
    global.get $stack
    i32.load
    return
  )
  (func $get_hello_b (;2;) (result i32)
    global.get $stack
    i32.load offset=4
    return
  )
  (export "hello" (func $hello))
  (export "get_hello_a" (func $get_hello_a))
  (export "get_hello_b" (func $get_hello_b))
)

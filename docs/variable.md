# Variables Conversion strategy

## Problem

### Global vs. Local

In Udon Assembly, there's no difference between a global variable and a local one. But in WebAssembly, there is.

There's a need to convert local variables to global ones with a way to keep them distinguishable from the original global ones.

### Avoidance of name collision

In Udon Assembly, there are some possible name collisions of reserved variables.

## Solution

The solution is to generate the name of all variables with the rules below:

### Rules (in order)

- If the variable is a local one, prepend `{function_name}_L{local_index}__` to its name.

  - `function_name` is the name of the function in which the variable should be called.
  - `local_index` is the index of the variable in the function.

- If the variable is a global one, prepend `G__` to its name.

- Prepend `__` to the name of all variables.

# Unnamed Vegetable

unv is a toy language made for my Principles of Programming Languages class final project

## Features

### Datatypes
Int, Bool, Closure

### Functions
Let, plus, minus, multiply, geq, leq, equal, not, if/else, recursion

## Example

```
let a = 5
let b = 10

let add(x)(y) = x + y

let rec(x) =
    if x <= 1:
        return 1
    else:
        return rec(x-1) * x

let tmp = add(a)

println!(tmp(b))
println!(rec(a))
```

## Other
Other concepts to explore

- Static type checking before running program
- tail optimization
- simd support
- rational numbers
- a borrow checker
- cps dynamic optimization


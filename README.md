# Unnamed Vegetable

unv is a toy language made for my Principles of Programming Languages class final project

## Features

### Datatypes
Int, Bool, Closure

### Functions
Let, plus, minus, multiply, geq, leq, equal, less, greater, not, and, or, if/else, recursion

## Example

```
let a = 5
let b = 10

let add(x)(y) = x + y

let rec(x) =
    if x <= 1
        1
    else
        rec(x-1) * x

let tmp = add(a)

print(tmp(b))
println(rec(a))
```

## Other
Other concepts to explore

- tail optimization
- inf sized ints
- rational numbers
- a borrow checker
- cps dynamic optimization
<!-- // add comments to unv in the form of "//" -->
<!-- future add? struct, return, yeild, monads, strings, lists -->
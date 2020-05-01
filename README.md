# Unnamed Vegetable

unv is a toy language made for my Principles of Programming Languages class final project

## Features

### Datatypes
Int, Bool, Closure

### Functions
Let, plus, minus, multiply, geq, leq, equal, less, greater, not, and, or, if/else, recursion

## Example

```
let x = 2 * 2
let y = x + 5

let rec(x) =
    if x <= 1
        1
    else
        rec(x-1) * x

println y
println 3 >= 2
println rec(5)
```

## Other concepts to explore

- tail optimization
- inf sized ints
- rational numbers
- a borrow checker
- cps dynamic optimization
- add comments to unv in the form of "//"
- consider adding: struct, return, yeild, monads, strings, lists
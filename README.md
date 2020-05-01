# Unnamed Vegetable

unv is a toy language made for my Principles of Programming Languages class final project

[Youtube video](https://youtu.be/OMJgpqiTJSs)

## Features

### Datatypes
Int, Bool, Closure

### What you can do
- let
- print
- println
- Equal
- NotEqual
- Greater
- GreaterEqual
- Less
- LessEqual
- Minus
- Plus
- Div
- Mult
- Not
- Negate
- Groupings
- IfElse
- FunCall (with recursion)

## Example

```
let x = 2 + 1 * 2
let y = x + 5
let z = (2 + 1) * 2

let rec(x) =
    if x <= 1
        1
    else
        rec(x-1) * x

print x
print y
println z
println 3 >= 2
println rec(5)
```

## Other concepts to explore

- inf sized ints
- rational numbers
- a borrow checker
- cps dynamic optimization
- add comments to unv in the form of "//"
- consider adding: struct, return, yield, monads, strings, lists
Constraints are a concept which generalizes what types do -- constrain your codebase regarding which operations can be done in certain values.

But there's more: Constraints can actually enable new things to happen that couldn't before.

Let's check it out.

### Constraining

```nano
# A constraint is a function which returns a 'boolean'.
fn IsPositive(num: i32) -> num > 0

# The 'satisfies' operator '|-' can be used to constrain
# a binding to require first checking for a constraint.
fn sqrt(num: i32 |- IsPositive) -> ...

let myNum = 4

sqrt(myNum)
# Error: Value lacks constraint 'IsPositive(num)' required by parameter.

if IsPositive(myNum) then (
    sqrt(myNum)
    # If the constraint is checked in, say, the condition
    # of an if expression, then you can use it here.
)

# Like usual, you can use expressions in the constraint position, including functions.
# So... generic constraints!
fn GreaterThan(threshold: i32) -> fn num -> num > threshhold

fn sqrt2(num: i32 |- GreaterThan(0)) -> ...
```

### Enabling

You can use constraints to tell the compiler information about how it can manipulate your values. The compiler must hold you to your word, so this might introduce changes in how you can use this value elsewhere.

```nano
let a: list(i32) = list [ 1, 2, 3 ]
let a |- heap_alloc

# Inference ripples backwards here,
# making the list [1, 2, 3] be allocated on the heap.
```
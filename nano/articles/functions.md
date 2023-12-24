## Functions

A function is a value that represents an expression packed and saved for later.

```nano
let say_hi = fn -> print "Hi!"

say_hi()
# Hi!
```
A function can declare some local bindings (its parameters) that will be filled in with values whenever the function is 'called'.
```nano
let greet = fn who -> print "Hello there, {who}!"
let double = fn x -> x * 2
```
You can 'call' that function later, passing parameters if necessary, to fill in the expression and then evaluate it.

```nano
greet("Reader")
# Hello there, Reader!

let result = double(4)
print result  # 8
```

You can call a function without parenthesis if it takes exactly one argument.

```nano
let result = double 4
```

You can explicitly name each parameter when calling the function.

```nano
greet(who: "Reader")
```

Though a function returns a single expression, you can use a grouping '()' to evaluate several expressions.
```nano
let big_function = fn -> (
    print "One"
    print "Two"
    print "Three!"
)

let result = big_function()
###
One
Two
Three!
###

# the value stored in result is "Three!" since it was the result
# of the last expression in the grouping.
```

There's a shorthand syntax for declaring a binding to a function value. You saw this when we where declaring our main function.

```nano
fn main -> print "Hello, World!"

# Speaking of which, now you know what 'print' is --- it is a function!
```

### Generic Functions

You may also declare function templates, which require additional information before evaluating to a valid function you can use.

```nano
# This function takes a pair of values of unspecified type,
# then a single value of *the same type*.
# It returns a quintuple of that type.

fn(T) foo(a: [ T, T ], b: T): [ T, T, T, T ] -> (
    [ ...a, b ]
)
```

Generic functions may lack an implementation.

```nano
fn(T) make_big(value: T);
```
In which case you can add your own somewhere else, perhaps even in another file.

```nano
impl(T = i32) make_big (value) -> value * 100

impl(T = string) make_big (value) -> value.as_uppercase()
```
This is how you can achieve the equivalent to polymorphism in nano (this is most similar to Rust's traits) and have one function change its behaviour depending on the types of the values passed to its parameters.

Speaking of 'types', let's learn what are they!
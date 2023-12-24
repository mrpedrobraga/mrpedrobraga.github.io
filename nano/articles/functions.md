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
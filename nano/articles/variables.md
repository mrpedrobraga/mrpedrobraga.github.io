## Variables

A variable is a lexical node in your code, where you can tell 'nano' to 'remember' a value for later usage.

You create a variable with the `let` keyword, possibly giving it a type and an initializer expression.

```nano
let a: int = 3
```

This expression can be anything, as long as the result is of the correct type.

```nano
let a: int = 2 + 2
#   ^ a will be '4'.
```

You can, then, use 'a' somewhere else and its value will be embedded there.

```nano
let a: int = 3 * 5
let b: int = 1 - a
#   ^ b will be -14
```

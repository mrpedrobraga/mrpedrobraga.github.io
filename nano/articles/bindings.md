## Bindings

Let's say you have some complex expression that you want to print twice to the standard output.

```nano
# Don't worry about this mysterious 'print' syntax yet.
print 2 + 2
print 2 + 2
```

Simply writing the expression works, but if your expression is computationally expensive or has side-effects you will run into problems.

If you were talking to a friend, you could say "calculate an expression... let's name its result 'a'... now print a, then print a again."

Well, the compiler is your friend, and your way of expressing this thought is through a binding.

```nano
let a = 2 + 2

print a
print a
```

A binding doesn't necessarily correlates to a memory allocation.
What it is: a semantic tool in your conversation with the compiler.

In fact, this code is likely to compile to the equivalent of:

```nano
print 4
print 4
```

This optimization can be made because the choice of bindings and values well informs the compiler of your 'intent' for the program.

If you create a binding that holds a structure value, its valid member accesses are themselves considered bindings.

```nano
let a = { name: "William", age: 20 }

a.name  # A binding!
```
In that way, the previous code is exactly equivalent to:
```nano
let a_name = "William"
let b_age = 20

a_name
```
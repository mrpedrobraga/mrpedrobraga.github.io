Expressions are how you put building blocks together to create a program.

In nano, everything is an expression.

### Constants

A single literal is an expression. It represents a value, which is the one you wrote.

```nano
1.0   # Actually an expression. It represents the value '1.0'.
```

### Operations

nano has a few builtin operators that can combine other expressions into a new, bigger expression.

```nano
2 + 2  # Binary '+' operator, takes expressions on the left and on the right.
```

Any expression you create has its own value, which follows some rule (depending on the expression building blocks you used). In this case, the value of the expression is '4'.

(I will not enumerate all the operators here, but you can look 'em all up in [References])

Wait, I said any *expression* can be used with operators -- not just constants. And, yeah, check this out.

```nano
2 + 2 + 2
# ^ This is a binary operation that has
# Left: 2
# Right: 2 + 2
```

There are intricate rules for how operators group themselves (associativity and precedence) which I won't get into here, but if you want to be more specific with where each expression go you'll need Groupings.

### Groupings

```nano
(2 * 3) + (1 * 2)
# Binary '+' operator that has
# Left: 2 * 3
# Right: 1 * 2
```

### Containers

You can wrap expressions within containers to create structured values.

```nano
# []s create a linear structure.
[ 1, 2, 3 ]

# {}s create a named-property structure.
{ name: "William", age: 20 }
```
Structured values are many values reinterpreted into a single value, so it can be moved around in memory as a whole.

### Member access

```nano
# You can use '.' to access a member from it by passing its offset from the start.
[ 1, 2, 3 ].0   # evaluates to '1', the number at offset 0.

{ name: "William", age: 20 }.age  # evaluates to '20'.
```

### ...and the list goes on!

There are, of course, many more kinds of intricate expressions you can make. We'll, again, learn as we go.
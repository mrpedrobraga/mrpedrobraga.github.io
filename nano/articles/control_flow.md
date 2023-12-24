There are a series of operators that have special builtin support for dealing with groupings of expressions.

### pipe

A pipe can unfold a deeply nested expression into a series of consecutive expressions.

It is a good way of tracking changed that happen to a single value.

```nano
# Instead of 

double(sum(3, triple(4)))

# we can use

4 |> triple _ |> sum(3, _) |> double _ 
```

The input expression is effectively splatted onto the placeholder `_` on the right-hand side.

### return

`return` stops execution of the closest parent function scope.

```nano
fn main -> (
    print "Hello, world!"
    return
    print "This will never run!"
)
```

`return` can be given a value, which will be the value observed by the function's call-site.

```nano
fn double(x) -> (
    return x * 2
)

let y = double(3) # value is 6
```

### if/unless

`if` evaluates an expression, and if it evaluates to 'true', evaluates the other expression.

```nano
if 3 > 4 then "Lorem ipsum"
```

`unless` evaluates an expression, and if it evaluates to 'false', evaluates the other expression.

```nano
unless 3 > 4 then "Lorem ipsum"
```

### else/elif/elun

`else` can be added to an if/unless expression with an expression that will be evaluated if the condition fails.

```nano
if 3 > 4 then "A" else "B"
```

`elif` `elun` can be used as a combination of `else` and a new `if/unless`.
```nano
if 3 > 4 then "A" elif 3 > 2 then "B" else "C"
unless 3 > 4 then "A" elun 3 > 2 then "B" else "C"
```

### loop

`loop` executes an expression again and again forever.

```nano
loop print "Hi"
```

### continue/break

Within a loop (of any kind, not just `loop` loops) you can use `continue` to move to the next iteration of the loop, optionally providing a value.

```nano
loop (
    print "Hi"
    continue 
    print "This won't run"
)
```

You can use `break` to get out of a loop early.

```nano
loop (
    print "Hi"
    break
)
```

### for 

`for` iterates over a collection and evaluates an expression again and again based on the iterations.

```nano
for i in [1, 2, 3] do (
    print i
)

# 1
# 2
# 3
```

### match

`match` blocks match a value to a pattern matcher.

```nano
let foo = [ 1, 2, 3, 4 ]

let bar = match foo with ?{
    [ i32, (i32*) as R ] -> R
}

# bar = [ 2, 3, 4 ]
```

Pattern matching is such an important topic that it'll get its own dedicated chapter.
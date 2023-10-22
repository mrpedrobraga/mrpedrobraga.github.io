## Functions

Functions are a special kind of value that represents a
section of code. Functions can be called to execute that code.

The builtin function "exit" exits the program.

```nano
exit()
```

Functions can take parameters. Say, for example, print,
will print its parameter to the standard output.

```nano
print "Hello world"
```

You can create your own functions - they're just values, like
strings or ints.

```nano
let printHi = fn -> print "Hi!"

let printNTimes = fn what, n -> for n do print what

let printTwice = fn what -> printNTimes(what, 2)
```

There's a shorthand for creating a function and naming it:

```nano
fn printThrice (what) -> printNTimes(what, 3)
```

Functions calls will have a "return value." It'll be of some type,
and you can fit it whenver you can fit that type.

```nano
# Ooo

fn double (x) -> x * 2

let n: int = double(3) + 4
```

The return type is inferred, but you can force its return type
to be something.

This is a good practice, because it will keep you from returning
a wrong value accidentally.

```nano
fn triple(x): int -> x * 3
```

The functions we declared can be used with values of many types as parameters.

```nano
fn merge(a, b) -> a + b

merge(1, 2)  # 3
merge("Hey", " there") # "Hey there"
# merge("Hey", 2) will cause a compilation error, though...
```

You can restrict which values can be passed by
typing your input parameters.

```nano
fn merge (a: string, b: string) -> a + b
```

Oh, almost forgot to type the return value!

```nano
fn merge (a: string, b: string): string -> a + b
```

### Functions are awesome.

Here's a list of cool things you can do with nano functions.

```nano
# Buckle up!

# Call with named parameters.
merge(a: "Hey", b: " There")
```

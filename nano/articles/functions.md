## Functions

Functions are a special kind of value that represents a
section of code. Functions can be called to execute/evaluate that code.

The builtin function "exit" exits the program.

```nano
exit()
```

Functions can take parameters. Say, for example, print,
will print its parameter to the standard output.

```nano
print "Hello world"
```

Yes! All along, 'print' was a function!

Functions will be available for you, giving you their... functionality... to use!

You can create your own functions - they're just values, like
strings or ints.

```nano
# fn -> <expression>

let printHi = fn -> print "Hi!"

let printNTimes = fn what, n -> for n do print what

let printTwice = fn what -> printNTimes(what, 2)
```

There's a shorthand for creating a function and naming it:

```nano
fn printThrice (what) -> printNTimes(what, 3)
```

Functions calls will have a "return value." It'll be of some type,
and you can fit it wherever you can fit that type.

```nano
fn double (x) -> x * 2

let n: int = double(3) + 4
# The return type of 'double(3)' here is 'int'.
# The return value of 'double(3)' is 6.
# Therefore, it can be used as if it was ain it.
```

The return type is inferred, but you can explicitly declare a return type for your function.

This is a good practice, because it will keep you from returning a wrong value accidentally, as well as off-loading some of the work nano has to do.

```nano
fn triple(x): int -> x * 3
```

The functions we declared can be used with values of many types as parameters.

```nano
fn merge(a, b) -> a + b

merge(1, 2)  # 3
merge("Hey", " there") # "Hey there"
# merge("Hey", 2) will cause a compilation error, though...
# because you can't '+' a string with an int.
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

Now we're thinking with types.

### Control flow

You can call 'return' inside a function to quickly exit its scope. You may also pass an expression to it, that will become the return value of the function.

The expression must match the function's return type.

```nano
fn count_to_x(x: int): string -> (
	if x < 0 then return "Can't count to negative numbers."

	for i in x do (
		print i
	)

	return "Counted!"
)
```

If you omit a return type on your function, the return expressions are where the return type will be inferred from.

```nano
fn get_value() -> (
	if mysterious_condition then (
		return "Two hundred"
	) else (
		return 200
	)
)

# The return type of 'getValue' is inferred as 'string | int'
```

'return' can be used for constraint narrowing on unnested scopes:

```nano
fn round(value: int | float) -> (
	# No need to floor if the value is an int
	if value is int then return value

	# Since the case where 'value' is an int causes
	# the function to stop,
	# type of 'value' down here is narrowed to 'float'!!!
	let b: float = value

	return floor(b)
)
```

#### Quick talk

The reason nano allows not explicitly typing your parameters and return is for faster prototyping. After you're finished testing ideas, you should probably type things explicitly.

You don't have to go everywhere and do it by hand, though, this should be very easily achieved by using macros & queries, which we'll learn about in the future.

### Functions are awesome.

Here's a list of cool things you can do with nano functions.

```nano
# Buckle up!

# Call with named parameters.
merge(a: "Hey", b: " There")

# Unwrap containers into parameters
let params = ["Hey", "There"]
merge(...params) # Same as doing `merge("Hey", "There")`.

# Omit parenthesis when calling a function with one or
# more parameters:
# Makes it look like a keyword!
merge "Hey", "There"

# You can partially fill a function call and save it for later.

let merge_with_hey = merge("Hey", ...)
#                                ^ placeholder notation

# Then, you call with the remaining parameters.
merge_with_hey("There")

# This allows you to keep the sources of truth low
# when creating functions that do the same thing:

let multiply = fn a, b -> a * b
let double = multiply(..., 2)
#                      ^ reads as 'multiply "something" with 2'
```

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

## Signals & State

A simple and helpful construct nano has is the 'signal'. A signal is a callable value just like a function.

```nano
let started_to_rain = signal()
# or shorthand 'signal started_to_rain'
fn get_umbrella() -> print 'Got umbrella!'

started_to_rain::connect get_umbrella

started_to_rain()
# (prints 'Got umbrella!')
```

A signal creates a 'hookup' where you can connect two parties' behaviours without one having to have knowledge of the other. Here, neither the started_to_rain signal not the get_umbrella function are referring explicitly to each other.

A signal can send data when emitted to the functions...

```nano
let my_signal: signal<int>
my_signal(3)
```

But it doesn't have any memory, it doesn't store the last value it was emitted with.

For that, we have the love child of 'signal' and 'slot'...

```nano
let my_int = state 3

my_int.changed::connect fn x -> print 'State is now {x}.'

# A state is a reference type identical to slot,
# but it has a signal 'changed' which will emit
# every time the state changes.

# You'll also find some stateful variants of other reference types.
let my_list = state_list ["a", "b", "c"]
```

### Template parameters

Remember generic types? Well, you can make generic functions.

```nano
fn self_pair<T> (value: T): Pair<T> -> {a: value, b: value}

self_pair<int>(3)
#          ^ filling the paramter

self_pair(10)
# Without filling it, T is inferred from usage.
```

### 'About' (self) parameters.

A function can have a function named "self," in which case it can be called on the declared type for self using '::'

```nano
fn double(self: int) -> (
	self * 2
)

let a: int = 3
let b = double(a)
let c = a::double()
```

This kind of function is called a 'method' of int (the used type).

You can create a method right inside a struct, and then you don't need to specify a 'self' parameter.

This method can only be called as a method of 'Person', too.

```nano
struct Person {
	name: string

	fn greet() -> (
		print "Hi, I am {name}!"
	)
}

let a: Person = Person()
a.greet()
```

You can override the behaviour of structures with some builtin methods.

```nano
struct Person {
	name: string

	# Getters and setters for the properties
	get name -> "I'll tell you when i die!"
	set name (value: string) -> name = "New name: " + value

	# Getters and setters for itself
	get as int -> name.size

	# When a person is created it now requires a name.
	construct(name: string) -> (
		self.name = name
	)

	# Called when value goes out of scope.
	destruct -> (
		print "'{name}' died."
	)
}

(
	let a: Person = Person("Agatha")
	print a.name   # "I'll tell you when i die!"
	print (a as int) # 6

	# 'a' goes out of scope
	# "Agatha died."
)
```

This allows you to create constructs that are very comfortable to use.

### Operators are Functions!

Remember? I said that!

The operator '+' is just syntactic sugar for the method core::$"+"

```nano
# In core library:

fn $"+" (self: int, other: int) -> nano.sum(self, other)
```

You can create your own operation for your types by defining your own implementation of $"+".

```nano
fn $"+" (self: Person, other: Person) ->
	Person(self.name + " and "+ other.name)
```

Wait? How can we declare a function named $"+" if one already exists?

### Functional polymorphism

You can declare functions with the same name, using the fn shorthand syntax... as long as their parameter list is different in quantity or types.

```nano
fn add (a: int, b: int) -> ...
fn add (a: string, b: string) -> ...
```

### Functions are sweet.

Here's a non exhaustive list of cool syntactic sugar you can do with nano functions.

```nano
# Buckle up!

# Call with named parameters.
merge(a: "Hey", b: " There")

# Spill containers into their parameter list when calling.
let params = ["Hey", "There"]
merge(...params) # Same as doing `merge("Hey", "There")`.

let params2 = {a: "Hey", b: "There"}
merge(...params2) # Same as doing `merge(a: "Hey", b: "There")`.

# Omit parenthesis when calling a function with one or
# more parameters:
# Makes it look like a keyword!
merge "Hey", "There"

# You can partially fill a function call and save it for later.

let merge_with_hey = merge("Hey", ...)
#                                  ^ placeholder notation

# Then, you call it with the remaining parameters.
merge_with_hey("There")

# Shorthand for
# `let merge_with_key = fn b -> merge("Hey", b)`

# This allows you to keep the sources of truth low
# when creating functions that do the same thing:

let multiply = fn a, b -> a * b
let double = multiply(..., 2)
#                      ^ reads as 'multiply "something" with 2'

# A function can collect several arguments when called
# into a single array.

# This function here has only one array parameter.
fn printAll(...numbers: array<int>) -> (
	print numbers
)

# But gets called with three non-array parameters.
printAll(1, 2, 3)
# ^ prints [ 1, 2, 3 ]
```

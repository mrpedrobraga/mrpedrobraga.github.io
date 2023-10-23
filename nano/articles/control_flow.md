## Control Flow

With all you've learned so far, you can only make programs that calculate the same set of things every time.

```nano
print "Hello there!"
print "Heyo!!!"
```

You may want to choose different "paths" for your code depending on some truth, for example.

### if/unless

```nano
let it_rained = false

# Executes the expression if the condition is true
# if <expression> then <expression>
if it_rained then print "Hi"

# Executes the expression unless the condition is true
unless it_rained then print "Going outside!!!"
```

You might want to execute more than a single action after checking a condition. You can do that by using groupings.

```nano
let it_rained = false

if it_rained then (
	print "I love rain"
	print "Rainnn!!!"
	print "AAAAAAAAAAAAA i lov erain so muchchhhh!!1!!"
)
```

You can also do groupings on the condition expression.

```nano
if (
	let a = 3
	let b = 4
	a + b > 9   # <- the return value of a grouping will be the last expression evaluated within it.
) then print "a + b is bigger than 9 or something"
```

if/unless are expressions themselves, so they return a value (the value they return is the value of the expression after the 'then').

```nano
let carrying = if it_rained then "umbrella"
```

But, wait!
Since this code runs conditionally... what if the condition fails? What does it return?

Good catch, eagle-eyed reader... Well, you just ran into the first appearance of the builtin type 'nothing'.

```nano
let carrying: string | nothing = if it_rained then "umbrella"

# The type nothing is so cool,
# only one value can be assigned to it: itself.

# It really works when disjuncted with another type,
# it encapsulates the idea of a value not existing.
```

But, wait! -- you ask, again -- A string | nothing isn't useful at all! I can't use it somewhere that requires a 'string'.

Ahem!

When creating a new scope with an if expression, you can narrow the type of a variable to one of the variants of the disjunction by checking it.

```nano
let carrying: string | nothing = if it_rained then "umbrella"
let b: string

if carrying is string then (
	b = carrying
	# Inside this scope, 'carrying' is of type 'string'
)
if carrying is nothing then print "Was carrying nothing..."
```

This is called "constraint narrowing," and will be explorer further when we learn more about constraints.

'unless' works in the complementary manner.

```nano
let value: string | nothing = ...

unless value is nothing then (
	# Value will be 'string' here...
)
```

### else

On the previous code block, you can see that i checked if 'carrying' was a string, then I checked if it was 'nothing'.

There's a better way of evaluating an expression if the previous if/unless condition check failed.

```nano
if carrying is string then (
	b = carrying
	# The type of 'carrying' here is 'string'
) else (
	# The type of 'carrying' here is 'nothing',
	# Because the previous 'if' expression consumed it.

	print "Was carrying nothing"
)
```

You can run an else block that makes another check of its own.

```nano
let value: int | float | bool = ...

if value is int then (
	# 'value' is 'int' here
) elif value is float (
	# 'value' is 'float' here
) else (
	# 'value' is 'bool' here
)

unless value is int (
	# 'value' is 'float | bool' here.
) elun value is float (
	# 'value' is 'int | bool' here.
) else (
	# 'value' is 'int | float' here.
)
```

### Pattern matching

Doing a bunch of if/else expressions to check a bunch of things is so clunky and tedious to write... well...

```nano
match
	when value is int then ()
	when value is float then ()
	else ()
```

It will also do the thing where it narrows the type of disjunctions just as well.

But, we can do one better. Notice that all the checks are checking something about 'value'. In special, we're doing a type check.

A type is also a special kind of value called a 'pattern matcher' -- in a match statement we can use it against a value to see if there's a match.

```nano
match value
	with int then ()
	with float then ()
	else ()
```

If you don't have a pattern matcher, you can still use a common 'value' there and check it against expressions.

```nano
let value = ...

match value
	when 5 then ()
	when 2 + 2 then ()
	else ()
```

Types aren't the only pattern matchers.
In a few sections you'll not only learn about some other things that are secretly pattern matchers, but also learn how to create your own!!!

### Pipes

_Super Mario Bros Warp Pipe Jingle_

Sometimes, nested ((((parenthesis)))) aren't the best way of writing your expressions.

Consider the following example code:

```nano
fn double(x) -> x * 2
fn triple(x) -> x * 3
fn sum(a, b) -> a + b

let a = 3

let result = sum(double(triple(a)), 3)
```

Your eyes have to go aaaalll the way to the middle of the expression and work outwards from there.

Now consider this code:

```nano
let result = a |> triple _ |> double _ |> sum _, 3

# Pipe operator |> = '|' + '>'
```

You first triple, then double, then sum with three. So much cleaner, right?

```nano
# You can even format it like this:

let result = a
	|> triple _
	|> double _
	|> sum _, 3
```

There is a special variant of the pipe operator (destructuring pipe) that works on collections such as arrays.

```nano
let c = [1, 2, 3] *|> _ * 2
print c   # [2, 4, 6]
```

### Loops

What if you want to do the same thing many times?

What if you want to do the same thing many times?

What if you want to do the same thing many times?

What if you want to do the same thing many times?

What if you want to do the same thing many ti-

```nano
# A loop expression will execute its scope over and over again forever.
loop (
	print "What if you want to do the same thing many times?"
)
```

Now this isn't very useful, but luckily you can break out of a loop with 'break'.

```nano
let should_keep_running = true

loop (
	# if 'should_keep_running' is assigned anywhere from this scope, the loop might break

	if not should_keep_running then break
)
```

You may provide an argument to 'break' to be the value of the loop expression.

```nano
let a = loop ( break "This is a useless loop!" )

print a   # "This is a useless loop!"
```

The previous code block pattern is so useful that there's two shorthands for it.

```nano
let a = 10

while a > 0 do (
	print("a is now {a}")
	a -= 1
	#  ^ subtracting-assignment operator
)
```

'while' will keep rerunning the scope while the condition remains true. 'until', on the other hand, will keep rerunning the scope until the condition evaluates to true.

```nano
let a = 0

until a == 10 do (
	print("a is now {a}")
	a += 1
)

# 'until' is to 'while' what 'unless' is to 'if'.
```

There is the 'continue' keyword, which moves finishes the current run of the loop and moves to the next.

```nano
loop (
	print "Hi"
	continue
	print "Hello" # this code never runs
)
```

You may provide an argument to 'continue' to be the value of the current execution of the loop expression.

```nano
let x = 0

let a = loop (
	x += 1
	if x < 3 then continue "This is still a useless loop!";
	break
)

print a
# [
#   "This is still a useless loop!",
#   "This is still a useless loop!",
#   "This is still a useless loop!"
# ]
```

Both while and until will do the type-constrainy thing that if and unless do.

There is another loop syntax that spawns executions of its expression based on an 'iterator.'

```nano
for 10 do (
#    ^ an int n has an associated iterator, which spawns
#      n executions.
	print "Hey"
)

# This will print 'Hey' ten times.
```

Iterators will return a value associated with each spawned execution (each iteration). You can capture it and use it.

```nano
for i in 10 do (
	print i
)

# This will print the numbers:
# 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
```

You can iterate over lots of things:

```nano
# Ranges
for i in 4..10 do (...)
for i in 9...23 do (...)

# Collections
for el in ["Hey", "there", "bro"] do (
	print el
)

# Strings
for char in "Hey there bro" do (
	print char
)
```

But there's more that you can do other than 'doing' something for each iteration.

```nano
# Selecting results into an array
let a = select i * 2 for i in 0..10
print a   # [0, 2, 4, 8, 10, 12, 14, 16, 18]

# Selecting results but only sometimes
select i for i in 0..10 when i > 5

# Logic checks
some i > 5 for i in 0..10
every i > 5 for i in 0..10

# Counting iterations
count i for i in 0..10

# You can apply 'when' filters to anything:
# 'some', 'every' and 'count'...
count i for i in 0..10 when i > 5
```

Lastly, you can do things with values across iterations.
The return of such expression won't be a collection, but a single value.

```nano
let a = for a, b across [1, 2, 3] select a + b
print a   # 6
```

### The future

There's more ways of controlling the flow of your program, but you'll learn more about them as you go.

### Section for nerds

Welcome to the section for extra nerds, again!

A scope like this is also a 'lexical' scope, that is, it limits the variables that you declare inside it to only be addressable from within the scope.

```nano
(
	let a = 3
)

print a
#     ^ symbol 'a' not found within scope
```

Also, whenever a scope ends, whatever memory is tied to it is also freed.

This imposes some interesting restrictions as to how we can create memory in our programs, which we'll learn more about in the future.

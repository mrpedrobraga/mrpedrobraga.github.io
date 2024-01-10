Pattern matching is one very big powerful concept in nano.

A pattern matcher is something that can be matched against to assert the format of a value.

## Normal values

Matching against normal values simply checks for equality.

```nano
let foo = 3

foo ?|> 3
# Evaluates to 'Result.Ok([])'

foo ?|> 4
# Evaluates to 'Result.Err([])'
```

The result of a matching operation is typically the `standard` library type `Result(T)`.

## Constraints

Matching agaisnt constraints (again, functions which return boolean) will call the function on the value.

```nano
fn IsPositive(num: i32) -> num > 0

let result = 10 ?|> IsPositive
# Evaluates to Result.Ok([])
```

## variants

Variants of a `variants` type are pattern matchers and can be used to match values. This is the idiomatic way of detecting which of the variants a certain value currently is.

```nano
variants Names {
    Ollie
    Luna
    Keichii
}

let foo = Names.Luna

foo ?|> Names.Luna
# Evaluates to 'Result.Ok([])'
```

## Match groupings

With match groupings, it's possible to create a single matcher that matches multiple submatchers. Match groupings also allow matching to interface with control flow, evaluating an expression when a specific arm is matched.

```nano
let foo = Names.Luna

foo ?|> {
    Ollie => print "Awesome!"
    Luna => print "Luna is cool!"
    Keichii => print "Not sure about this guy..."
}

###
Match groupings are standalone values which can be saved
in bindings.

let cases = {
    Ollie => print "Awesome!"
    Luna => print "Luna is cool!"
    Keichii => print "Not sure about this guy..."
}
###
```
Match groupings are great for correlating inputs with outputs. It's better than an anonymous struct because its keys are pattern matchers and each arm is only evaluated when it is matched.
```nano
let num = 0

let num2 = num ?|> {
    0 => print 'Zero'
    1 => print 'One'
}

###
If instead using

let num2 = {
    0: print 'Zero'
    1: print 'One'
}[num]

it would print both cases ahead of time.
Also it would look uglier with the input at the end.
###
```

## Capturing

Matching is essentially breaking a structure into component parts. With capturing, you can get the values associated with a fragment of the matched input.

```nano
# Capturing the content of a variant type.

variants NumOrText {
    Num(i32)
    Text(string)
}

let foo = NumOrText.Text("Oops")

let r = foo ?|> {
    Text(v) => print v
    # since 'Text' matches, its string content can be used
}

# r's type is "Result(NumOrText)"
```

With a match grouping or with clever control flow, you can exhaust all the possible match options for a type.

```nano
let foo = NumOrText.Text("Oops")

let r = foo ?|> {
    Num(n) => print "Num: {n}"
    Text(t) => print "Text: {t}"
}

# r's type is no longer `Result(string)`, but plain
# `string`, since the match statement exhausts
# all the variant's possibilities making the matcher always
# succeed in every case.

# This is also possible if a match case diverges
# (breaks the parent control flow) since that case never
# has to produce a value for the match expression.

let r2 = foo ?|> {
    Num(n) -> print "Num: {n}"
    Text(t) -> return # diverges!
}
```

## Linear structures

Things become interesting when you start matching against linear structures.

```nano
# Imagine a format that consists of an array of mostly integers,
# but with one single string somewhere within.
let my_tuple = [ 1, 2, "Three!", 4 ]
let my_tuple_2 = [ 1, 2, 3, 4, "Five!", 6 ]

# The format for such a construct would be:
let StrangeType = [ i32..., string, i32... ]

# This matches "one or more i32"
# Then one string
# Then one "one or more i32"

let my_tuple_3 = ["Bob", 20]
let my_tuple_4 = ["Alice"]

let StrangeType2 = [ string, i32? ]
# The '?', read "maybe", optionally matches the preceding fragment.
# So this matches "one string"
# Then (optionally) one i32.
```
You can, of course, also use capturing with linear structures. This is done with the `as` keyword --- and is used everywhere.

```nano
# Let's match this list and capture the string
let value = [ 1, 2, 3, "Four", 5, 6 ]

let string_part = value ?|> [ i32..., string as R, i32... ]

# string_part = Result.Ok({ R: "Four" })
# Aha! If you've been wondering what's the content of the result
# (since it was previously always returning Result.Ok([]))
# It is an object with the captured fragments!

string_part ?|> {
    Ok(captures) => print captures.R
}
```
The syntax for destructuring a structure into local bindings looks a lot like pattern matching, right? Well, that's no coincidence: it IS pattern matching.

```nano
let value = [1, 2, 3, 4, "String", 5, 6]
let [ i32..., string as x, i32... ] = value
```

## Named-property structures

```nano
let value = {
    name: "Dess"
    age: 7
}

let IsAdult = {
    name: string
    age: fn x -> x > 18 
}

let result = value ?|> IsAdult
```

Mixing linear-structure and named-property-structure matching, you can match arbitrarily nested objects.

```nano
let value2 = {
    name: "Dess"
    values: [
        [1, 2, 3, "Four"]
        [1, 3, 4, 5, "Text"]
        ["More Text"]
    ]
}

let result = values ?|> {
    name: string
    values: [
        [ i32...?, string as R ]...
    ]
}

# Evaluates to:

###
    Result.Ok(
        {
            R: ["Four", "Text", "More Text"]
        }
    )
###
```


## Matching Code

Remember how in the Hello World example, it is specifically said that your code is better thought of as a program (ran in compile time) that declares bindings for the compiler?

```nano
from standard import { print }

# Declaring a binding named "main".
@entry fn main -> print "Hello, world!"
```

Turns out that you can match your very code with a pattern matcher by using a few builtin code-pattern-matchers.

### Existential pattern matchers

```nano
let a = 3

some (let a : string)
# This matches if, somewhere, there's a declaration
# of a symbol named 'a' with type string.

# You can evaluate this pattern matcher with the 'exists'
# keyword.

exists some (let a : string)

# Checking if there's any declaration of a certain type
# within scope.

let IntPair = [ int, int ]

exists some (let ... : IntPair)

# You can use capturing to capture the declared symbols.

let result = exists some (let ... : IntPair) as impl

###
result = {
    impl: [ ... ]
}
###
```

You can also look for more kinds of declarations.

```nano
# Struct with a certain name
some (struct QuirkyName)

# Function that returns a certain type
some (fn (...) -> IntPair)

# Implementation of generic function
some (impl(T = string) sum)
```

```nano
from core import { sum }

# Declaring a constraint that there must be some implementation
# of sum for the type T.

# The constraint passes the input through a matcher,
# which returns a value of type Result({ sum: ... }, [])
fn Summable(U) -> U ?|> (
    (some impl(T = _) sum) as sum
)

# Then, we can use that constraint in the generic type here.
fn(T |- Summable) total_sum(col: array(T)) -> (
    col.reduce(sum)
)

# Now, after we give that implementation...
impl fn(T = string) sum (a: T, b: T): T -> (
    "{a} + {b}"
)

# The compiler knows which implementation of sum to use here.
print total_sum ["Hey", "There", "Buddy"]
```

If, for some reason, you can't infer which implementation to use at compile-time, for example, if you have a struct that has a summable inside:

```nano
struct Summables {
    field1: Summable
    field2: Summable
}
```

You can't directly use `Summable` as type -- it is a constraint, after all. A constraint doesn't specify data layout, which is needed for the creation of this struct; To create a type based on the constraint, you'll need to use dynamic bindings.

```nano
struct Summables {
    field1: Dyn(Summable)
    field2: Dyn(Summable)
}

let foo = Summables {
    field1: "Hello"    # "Hello" gets converted to Dyn(Summable)
    field2: 8
}

###
    Dyn(Summable) looks somewhat like:
    {
        value: "Hello"
        bindings: {
            sum: view<function>
        }
    }
###
```
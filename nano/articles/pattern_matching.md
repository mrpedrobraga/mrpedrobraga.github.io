Pattern matching is one very big powerful concept in nano.

A pattern matcher is something that can be matched against to assert the format of a value.

## Normal values

Matching against normal values simply checks for equality.

```nano
let foo = 3

match foo with 3
# Evaluates to 'Result.Ok(3)'

match foo with 4
# Evaluates to 'Result.Err(4)'
```

The result of a matching operation is typically the `standard` library type `Result(T)`.

## Constraints

Matching agaisnt constraints (again, functions which return boolean) will call the function on the value.

```nano
fn IsPositive(num: i32) -> num > 0

let result = match 10 with IsPositive
# Evaluates to Result.Ok(10)
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

match foo with Names.Luna
# Evaluates to 'Result.Ok(nothing)'
```

## Match groupings

With match groupings, it's possible to create a single matcher that matches multiple submatchers. Match groupings also allow matching to interface with control flow.

```nano
let foo = Names.Luna

match foo with ?(
    Ollie -> print "Awesome!"
    Luna -> print "Luna is cool!"
    Keichii -> print "Not sure about this guy..."
)
```

The match grouping syntax allows for "capturing" for variants that have an associated type (discriminated unions).

```nano
variants NumOrText {
    Num(i32)
    Text(i32)
}

let foo = NumOrText.Text("Oops")

let r = match foo with ?(
    Text(v) -> print v
)

# r's type is "Result(NumOrText)"
```

With a match grouping or with clever control flow, you can exhaust all the possible match options for a type.

```nano
let foo = NumOrText.Text("Oops")

let r = match foo with ?(
    Num(n) -> print "Num: {n}"
    Text(t) -> print "Text: {t}"
)

# r's type is "string", since the match statement exhausts
# all the variant's possibilities making the matcher always
# succeed in *some* case.

# This is also possible if a match case diverges
# (breaks the parent control flow) since that case never
# has to produce a value for the match expression.

let r2 = match foo with ?(
    Num(n) -> print "Num: {n}"
    Text(t) -> return # diverges!
)
```

## Linear structures

Things become interesting when you start matching against linear structures.

```nano
# Imagine a format that consists of an array of mostly integers,
# but with one single string somewhere within.
let my_tuple = [ 1, 2, "Three!", 4 ]
let my_tuple_2 = [ 1, 2, 3, 4, "Five!", 6 ]

# The type for such a construct would be:
let StrangeType = [ ...array(i32), string, ...array(i32) ]

# But, if we get some unknown array in runtime, how do we actually
# check for that?

let value: array = ...

# In a regex-inspired fashion, you can represent portions of
# a linear structure with *, + or ?.

match value with [ i32*, string, i32* ]
```

Destructuring a linear structure into local bindings uses pattern matching syntax. So you can do this:

```nano
let value = [1, 2, 3, 4, "String", 5, 6]
let [ i32*, string as x, i32* ] = value
```

Using match groupings you can also capture parts of the structure.

Suppose you want to capture the string.

```nano
let value = [ 1, 2, 3, "Four", 5, 6 ]

let string_part = match value with ?(
    [ i32*, string as R, i32* ] -> R
)

# string_part = Result.Ok("Four")
```
There's a shorthand syntax to matching a value into component parts: the matching pipe.

```nano
let value = [ 1, 2, 3, "Four", 5, 6 ]

let my_match = value ?|> [ i32*, string, i32* ]
#     ^ Evaluates to Result.Ok([ [ 1, 2, 3 ], "Four", [ 5, 6 ] ])
#       Notice that the match result always has exactly
#       three fragments when it succeeds.

my_match[1]
# Access the matched string.

let my_match2 = value ?|> [ i32*, string as lone_string, i32* ]

my_match2.lone_string
# Access the matched string by name.
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

let result = match value with IsAdult

# into match groupings

let value2 = {
    name: "Dess"
    values: [
        [1, 2, 3, "Four"]
        [1, 3, 4, 5, "Text"]
    ]
}

let result = match value2 with {
    name: string
    values: [ [ i32*, string as R ]+ ]
}

# Evaluates to:

###
    Result.Ok(
        {
            name: "Dess"
            values: [
                [[1, 2, 3], R: "Four"]
                [[1, 3, 4, 5], R: "Text"]
            ]
        }
    )
###
```

"Matching pipe" also works with this kind of structures.

```nano
let result = value ?|> { name: string , ...any }
```
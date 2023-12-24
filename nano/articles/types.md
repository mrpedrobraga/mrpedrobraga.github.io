## Types

Bindings have "types" which are constraints on where they can fit. Types are like the shape of apendages in puzzle pieces.

```nano
# A parameter passed to this function must be of type 'i32'.
fn double(x: i32) -> x * 2

let a = double(4)
# Works fine!

let b = double("Three")
# Error: Value of type 'string' can not be assigned to parameter of type 'i32'.
```
A type is a specific kind of value, which is generally only used in compile-time. Moreso because *type bindings* are much more useful.

Consider 'i32'. It is a binding created within the core library that represents a data layout.

```nano
from nnc import { DWORD }

let i32 = primitive DWORD   # i32 is a guaranteed 32-bits value!
```
You can then use this type to qualify bindings and restrict where it can be used (and the values that can be assigned to it).

```nano
let x: i32 = 4
# A value of type i32 can be assigned into a binding of type i32.
```

### Creating Types

You can put whatever you want in the type position when declaring a binding, as long as it can be safely executed at compile-time.

```nano
# You can use a literal as a type, and it'll be reinterpreted
# as a type which only accepts that specific constant.

let x: 4 = 5  # Error: Value of type '5' can not be assigned to parameter of type '4'.
```

But you might want to create your own brand new types as bindings, either by using a primitive data layout...

```nano
from nnc import { QUADWORD }

let BigNum = primitive QUADWORD

fn foo(x: BigNum) -> print x
```
...or by manipulating existent types into new ones.

```nano
# A new type declared as a linear structure with two members of type i32.
let IntPair = [ i32, i32 ]

let x: IntPair = [ 4, 4 ]

# A new type declared as a named-property structure.
let NamedInt = { name: string, value: i32 }
```

nano provides shorthand notation for creating bindings with structure types.

```nano
struct IntPair [
    i32
    i32
]

struct NamedInt {
    name: string
    value: i32
}
```

It is convention to name your type bindings in CamelCase, so they can be easily told apart from other bindings.

Structures are conjunctive types, which gather many values into a bigger one. But another useful tool is to be able to store one of many possible types of values in one space.

```nano
let foo: int | string = "Three"
let names = "Mary" | "June" | "Pedro"
```

Another way of achieving this is with the `variants` keyword.
It allows you to name each variant even.

```nano
# Declare 'NumOrText' as a type with two variants.
let Names = variants { Mary, June, Pedro }

# Shorthand syntax:

variants Names {
    Mary
    June
    Pedro
}
```

In a `variants` type, each variant can have an associated type, making the overall type a discriminated union.

```nano
variants NumOrText {
    Num(i32)
    Text(string)
}

# You can store either of the variants in a binding of that type.
# Naturally, the 
let foo: NumOrText = NumOrText.Num( 4 )

fn bar(x: NumOrText) -> (
    # Checking which of the variants is inside a binding
    # can be done with a concept we'll learn soon enough!
)

bar(Text("Some Text"))
# Since the parameter 'x' is biased towards the type "NumOrText"
# you can simply write 'Text' here instead of 'NumOrText.Text'
# and it knows what you're talking about.
```

### Templates

Let's say you have some common concept regarding a data layout you're creating, but it is "incomplete." For example, youcan imaine having a pair of values of... *some* type, but you don't wanna specify what type it has to be.

This can be done with a function and parameters, as usual.

```nano
fn Pair(T) -> [ T, T ]

let IntPair = Pair(i32)
# Evaluates to '[i32, i32]'.
```

```nano
let x: Pair(i32) = [ i32, i32 ]
let y: Pair(string) = [ "Hey", "There" ]
let z: Pair(...) = [ bool, bool ]
#            ^ Type is inferred as "bool".

# Also, since 'Pair' is just a normal function,
# you can do this.
let w = Pair(4)

# Evaluates to [4, 4]
# The type of 'w' is inferred as [ 4, 4 ] also.
```

### Inference

You might've seen the type information omitted in many cases. That doesn't mean the binding lacks a type, but that the type is inferred from usage.

```nano
let x = 3
#   ^ Type inferred as '3'.

let y = double(3)
#   ^ Type inferred as 'i32'.
```

Inference can be done partially, by using pattern-matching effectively (We'll learn about all kinds of pattern matching in an upcoming chapter).

```nano
let Pair(T) = [ T, T ]
let x: Pair(...) = [3, 8]
#   ^ Type inferred as 'Pair(i32)'.
```
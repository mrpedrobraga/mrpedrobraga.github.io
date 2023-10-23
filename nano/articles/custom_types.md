## Custom Types

You can create your own types based off the primitive types.

```nano
type MyInt = int

let a: MyInt = 3
```

The initializer expression will try to be converted to the target type.
But, after the fact, you can't assign an 'int' to a 'MyInt' implicitly.

```nano
let a: int = 3
let b: MyInt = a
#              ^ Can't assign value of type 'int' to receiver of type 'MyInt'.
```

This effectively lets you borrow a data format but give it a new meaning.

Types are the first source of meaning and safety for your codebase.
If you create a type for a Product and a Customer off the same base type, they are still not interchangeable.

```nano
type ProductID = i64
type UserID = i64

let prod: ProductID = 3232
let user: UserID = prod
#                   ^ Can't assign value of type 'ProductID' to receiver of type 'UserID'.
```

But, maybe, you don't want a "new type," only another way of referring to an existing type.

Use aliases for that.

```nano
alias number = int

let a: number = 3
let b: int = a
```

### Type combination

You probably want more than simply renaming existing types though.

You see, a type, in practice, is a "set": The set of all values that can fit on the type. So, int is really a set of "0, 1, 2, 3, 4, 5, 6..."

You can do operation of the types as sets, to create new types.

```nano
# Type Disjunction ("|" reads "or")
type number = int | float
let a : number = 3

# A type disjunction tells you that this value is "either an int or a float."
# It can't safely be assigned to either type...

let b: int = a
#            ^ Can't assign value of type 'number AKA int|float' to receiver of type 'int'
#              because can't assign value of type 'float' to receiver of type 'int'.
let c: float = a
#              ^ Can't assign value of type 'number AKA int|float' to receiver of type 'float'
#                because can't assign value of type 'int' to receiver of type 'float'.

# But it can be used whenever both int and float are accepted.
# For example, you can print the value.
print a  # 3

# Or do arithmetic with it.
print a + 7  # 10
```

```nano
# Type Conjunction ("&" reads "and")
type Strint = string | int

# A type conjunction creates a value that is "both a string and an int"
# at the same time.
```

This very type can't actually be created, in practice,
so it's only useful when used with [structured types](./structures) (as we'll see later).

```nano
let a: Strint = ### what would i even write here lmao ###

# But the takeaway is, 'Strint' CAN be assigned to variables of type 'string' or 'int'
let b: string = a
let c: int = a

# That is, since it IS a string and an int, it can be safely used as either one.
```

When you assign a value to a variable and it is implicitly converted,
the original type information is lost.

```nano
let a: Strint = ...
let b: string = a
let c: int = b
#            ^ can't assign string to int, etc

# Even though you can do this
c = a
```

### Custom data formats

You can tag together two or more types to create (your first) a linear structure that stores several values side by side on memory.

```nano
type NamedInt = [ string, int ]

let a = ["john", 20]
let b = a
let c = a[0]
```

If you come from another language, you might know these linear structures as "tuples."

You can optionally name each fragment of the linear structure.

```nano
type NamedInt = [ name: string, value: int ]

let a = ["john", 20]
let b = a
let c = a[0]
let d = a.name
```

Note that you do not have to create a new type to create a linear structure value.

```nano
let a = ["john", 20]
#   ^ type of 'a' gets inferred from the literal as [ string, int ]
```

A named-property structure variant also exists.

```nano
let a = { name: "john", value: 20 }

a.name
a[0]  # This doesn't work.
```

And here's how it looks when you give it a name:

```nano
type NamedValue = struct {
	name: string
	value: string
}

# Or the shorthand:
struct NamedValue {
	name: string
	value: string
}
```

If you've seen a language like 'Javascript', it is basically built on an implementation of 'hash map', a named-property structure it calls 'object'.

Unlike Javascript, most things in nano are NOT hash maps: behind the curtains, a struct not very different from a linear structure, storing all its values packed side by side in memory.

```nano
let a = {foo: 3, bar: 7}
a.foo = 10
a.baz = 30  # Won't work!
```

A struct doesn't store its properties' names either. They only exist during the compilation stage so you can refer to them\*.

(\* Nerds: Reflection is still possible, as we'll see way later...)

### Structs are awesome!

Structs are the building block of your custom data formats, and, as I've been hinting for a while now, they get their own section all about them.

Buckle up!

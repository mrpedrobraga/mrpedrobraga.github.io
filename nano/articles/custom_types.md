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

You can tag together two or more types to create (your first) a linear structure that stores several values side by side on memory. This kind of type is called an 'array'.

```nano
type NamedInt = [ string, int ]

let a = ["john", 20]
let b = a
let c = a[0]
#         ^ you can then access it by its offset
#           this reads "skip 0 elements of a"
```

The values of an array are expressions separated by "," or a newline.

```nano
let a = [2 + 2, 4 + 4, 8 + 8]
print a   # [ 4, 8, 16 ]

let b = [
	0
	1	# No commas, so clean!
	2
	3
]
```

(If you come from another language, you might know these arrays as "tuples.")

You can optionally name each fragment of the linear structure.

```nano
type NamedInt = [ name: string, value: int ]

let a = ["john", 20]
let b = a
let c = a[0]
let d = a.name
#          ^ you can then access it by name.
```

Note that you do not have to create a new type to create an array value.

```nano
let a = ["john", 20]
#   ^ type of 'a' gets inferred from the literal as [ string, int ]
```

A non-linear structure variant also exists. That is, you can't access its properties by their position. Those are called 'structs'.

```nano
let a = { name: "john", value: 20 }

a.name
a[0]  # This doesn't work.
```

And here's how it looks when you declare a struct type:

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

### Templates

You can create generic (incomplete) types, which require filling before the type can be used.

```nano
# A pair of two values... the two values will have the same type, but which type it is, it's unknown.
struct Pair<T> {
	a: T
	b: T
}

let m: Pair<int> = {a: 3, b: 7}
```

This is a very powerful concept that can be used to create reusable code without knowing how it'll be used.

### Accessing members is an art!

Consider the following structure:

```nano
let names_and_numbers = [
	["John", 3]
	["Gabriela", 4]
	["Patrick", 5]
]

# The type of names_and_numbers is
# array<[string, int]>
```

Let's say you want the items 0 and 2:

```nano
names_and_numbers[0, 2]
# [ ["John", 3], ["Patrick", 5] ]
```

Or a range of items:

```nano
names_and_numbers[0...1]
# [ ["John", 3], ["Gabriela", 4] ]
```

Let's say you want to quickly grab all of the names and none of the numbers.

```nano
let names = names_and_numbers.*.[0]

# This reads:
# names_and_numbers
# *   -  Get all items of that
# [0] -  From those, get the first member

# ["John", "Gabriela", "Patrick"]

# Notice that the names are all on a single array,
# and the original "nested array" structure was lost!

# Let's try to get the numbers and preserve the original structure:

let numbers = names_and_numbers.[*].[0]
# [ [3], [4], [5] ]

# The names are missing from there, but the resulting array has the same structure! Woohay!
```

You can also do some access on structs too, but they're more rigid.

```nano
let a = {
	name: "Pedro"
	cake: {
		frosting: "white"
		size: 11
	}
}

a.(name, cake.frosting)
# { name: "Pedro", cake: { frosting: "white" } }
```

You can "pattern-match" the content of a collection like this to local variables using this syntax called "destructuring assignment".

```nano
let a = ["john", 20]
let [ ..., age ] = a
#      ^ don't care about the name

print age   # 20

let b = { red: 10, green: 30, blue: 255 }
let { red, green } = b

print green   # 30
```

## Creating structures is also an art.

You can spill the content of collections onto others.

```nano
let a = [1, 2, 3]
let b = [0, ...a, 4, 5]
print b   # [0, 1, 2, 3, 4, 5]

let c = { name: "lilith" }
let d = { ...c, age: 140 }
print d   # { name: "lilith", age: 140 }
```

There's a shorthand for creating a property on a struct with the same name as a local variable:

```nano
let name = "Margareth"
let person = {
	&name
	age: 50
}

print person   # { name: "Margareth", age: 50 }

```

Types are too awesome for only one part.
Let's explore more on part II.

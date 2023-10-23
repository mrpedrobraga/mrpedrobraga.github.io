## More Type Magic

nano comes with more ways of typing your variables, especially when it comes to structs.

Struct types aren't assignable to each other even if they have the same name.

```nano
struct A {
	name: string
}

struct B {
	name: string
}

let a: A = {name: "Pedro"}
let b: B = a   # 'A' isn't assignable to 'B', etc.
```

Throw away the rigid requirement with 'Like&lt;T&gt;'

```nano
let a: A = {name: "Pedro"}
let b: Like<B> = a
```

This will create an anonymous trait that only cares about the properties being the right names and types.

Wait, an tr- what?

### Trait

A trait is a structure type with loose restrictions. It only cares about the property definitions.

```nano
trait Personhood {
	name: string
}
```

```nano
# But unlike Like<T>
# A needs needs to be declared as having the trait.
struct A {
	has Personhood
}

let a: A = {name: "Pedro"}
let b: has Personhood = a   # 'A' isn't assignable to 'B', etc.
```

### Traits & Entities

Consider the following traits:

```nano
trait Personhood {
	name: string
}

trait Position {
	shared position: fvec2
}

trait Velocity {
	shared position: fvec2
	shared velocity: fvec2
}
```

An entity is a structure made only of traits. You can enable or disable its traits.

```nano
let a = entity [ Personhood, Position, Velocity ]
```

A property with the same name and marked 'shared' in two traits will share the same value on the entity.

So the 'Velocity' trait there can affect the 'position' property in the 'Position' trait.

That's right, nano has builtin ECS! Although the 'System' part of ECS doesn't exist until we learn about functions... hmm...

### You can't know everything...

Typing your program is fun and makes everything so meaningful and safe. But sometimes you really don't know the type of a value, because it is foreign to nano.

For that cases, the auxiliary type 'unknown' exists.
It implies no data format, and is unsized.

Anything can be assigned to 'unknown' but 'unknown' can't be assigned to anything.

```nano
let a: unknown = 3
```

'unknown' is pretty rare to come across (you're more likely to see 'byte' or 'array&lt;byte&gt;'), but you might see it.

If you do, you can tell nano how to treat the value by reinterpreting it.

```nano
let a: unknown = 3
let b: int = a::convert_bytes<i32>(4)
```

This will copy and treat the bytes of unknown as an int. This operation has undefined behaviour by nature, so be careful.

To be honest, you can convert the bytes of anything.

But, pay attention.

```nano
let a = 999999999

# This will actually convert the value the best it can to a float.
let b1 = a as float
print b1
# 1000000000.0

# This will just carry the bit representation as is, producing an undefined result.
let b2 = a::convert_bytes<f32>(4)
print b2
# 0.004724
```

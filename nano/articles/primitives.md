## Nano Data & Types

Here is a non-exhaustive list of nano's basic data formats.
You'll be using them as building blocks to create your software's
data structures.

```nano
# The bottom data format is "byte".
byte 0xAA
```

For your convenience, the core and standard libraries define some
types that are more useful out of the box, as well as syntax for creating them.

```nano
# A compact sized collection of bytes is a 'string.'
# It's commonly used to represent text.
string "Hello world!"

# A group of bytes treated as digits on a number is called an integer!
# Nano has a few integer formats of different sizes:
i8, i16, i32, i64

# `standard` provides an abstract type, when you don't care about the size.
int
#^ `int` will resolve to one of the integer types listed above, depending on the compilation target. Usually it's to i32.

# Unsigned int variants also exist:
u8, u16, u32, u64
uint

# Floating point data formats can (kinda) represent fractional values.
f32, f64
float

# Data formats are also 'types'.

# A value's type tells it where it can go.
# For example, the following is allowed:
1 + 1

# But not this:
true + 1
```

Sometimes, you need to make operations on more than one datum at the same time.

```nano
# There are a few builtin tuple values for the types shown above.
# Boolean vectors
bvec2, bvec3, bvec4

# Integer vectors
uvec2, ivec2, uvec3, ivec3, ivec4

# Float vectors
fvec2, fvec3, fvec4

# Creation:
bvec3(false, true, false)
ivec2(1, 3)

ivec2(5, 7) + ivec2(3, 4)
```

There are only a few types shown here,
but, as we'll see on a couple of sections, you can create your own data structures.

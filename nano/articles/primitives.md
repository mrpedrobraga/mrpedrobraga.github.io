## Nano Data & Types

Expressions can be made by arranging different "values" and operations on those values. Those values are data, which can be stored in many formats.

Here is a non-exhaustive list of nano's basic data formats.
You'll be using them as building blocks to create your software's data structures.

The bottom data format is "byte". Deep down, everything in your computer program uses bytes.

```nano
byte 0xAA  # 8 bits of storage for whatever you want.
```

For your convenience, the core and standard libraries define some types that are more useful out of the box. Those types are "reinterpretation" of bytes -- the same binary data under the hood, but with different treatment.

```nano
# A compact sized collection of bytes is a 'string'.
# It's commonly used to represent text.
string "Hello world!"
string 'hi mom'

# A group of bytes treated as digits on a number is called an integer.
# nano has a few integer formats of different bit sizes:
i8, i16, i32, i64

# `standard` provides an abstract type, when you don't care about the size.
int
#^ `int` will resolve to one of the integer types
#  listed above, depending on the compilation target.
#  Usually it's to i32.

# Unsigned int variants also exist, which have no sign bit,
# and, therefore, can represent numbers twice as large.
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

It's really useful to aggregate different data in a single "wrapper". nano provides a few builtin structures that combines bools, ints, floats, into an vectors.

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

There are only a few vector types shown here,
but, as we'll see on a couple of sections, you can create your own data structures.

### Inspecting your values

To see what is in a value, you can use 'print'. It will write the value in a legible way to the standard output.

```nano
print 3 + 3
#  ^ prints '6' to the standard output
```

For now, take 'print' for granted (it is a 'function', which we'll only learn about in a few sections).

## Operations

A single value IS an expression... but more complex expressions require operations.

I already sneaked in some "operations" into the previous section. They are simple to understand.

```nano
2 + 2
# ^ look at the "+" operator
```

An operation can aggregate multiple values onto an expression. That expression will be computed when your program runs and generate a new value.

```nano
  2
# ^ value of type int 2

2 + 2
# ^ returns a value of type int, in this case '4'
```

Nano has a few builtin operators.

```nano
# Addition
2 + 2

# Subtraction
2 - 2

# Product
2 * 2

# Integer division
3 // 2

# Remainder
2 % 2

# Floating point division
3.0 / 2.0
```

But -- actually! -- what an operator does completely depends on the parameters you give it. So, really, there is no "addition operator," only a "+" operator. Later, we'll learn that operators are actually functions in disguise, but, for now, you can remember operators as handy magic symbols.

Speaking of useful builtin operators...

### Logic operators

The operators I showed were arithmetic operators -- they require numeric inputs and return a numeric value of the same type.

Logic operators, on the other hand, are operators that require boolean values, and return a new boolean value.

```nano
true
false
true and false
#     ^ evaluates to 'false'
```

nano has a few of them, and they behave the way you'd expect from boolean algebra.

```nano
# Assume two values 'a' and 'b' of type bool

not a		# ¬a
a and b 	# a ^ b
a but b		# a ^ b
a or b  	# a v b
a xor b 	# (a v b) ^ ¬(a ^ b)
a implies b	# a -> b

# Nerd note for 'a implies b':
# These operators are in 'question mode',
# that is, they'll check for something.
# This doesn't "force a to imply b."
```

### Comparison operators

Comparison operators take in values of _some_ type but return exclusively booleans.

```nano
# Numeric size comparison
2.0 > 5.0
1.0 < 10
1 >= 3   (> =)
3 <= 1   (< =)

# Equality comparison
3 == 2
true == false  (= =)
true != false  (! =)
```

nano has a lot of operators, and it would be too much to list all of them, but you'll learn them eventually as you go.

### Order of operations

You can use multiple operators in succession to create complex operations.

The order of the evaluation of the operations depends on the 'precedence' of the operators.

```nano
# The operator '*' has precedence over '+'
2 + 2 * 4

# Twitter moment: What does this compute???
# It computes to '10'.

(2 + 2) * 4
# ^ You can use a grouping (parenthesis) to force
# the addition to be computed first
```

For operators of equal precedence, they are calculated from left to right.

```nano
2 + 3 + 4 + 5

# will be computed in this order

(((2 + 3) + 4) + 5)

# This is not very meaninful here, since addition
# is commutative, but check this out:
# What does this evaluate to?
10 // 2 // 5

# 10 // 2 is 5.
# 5 // 5 is 1.
```

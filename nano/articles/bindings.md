## Bindings

Let's say you have some complex expression that you want to print twice to the standard output.

```nano
# Don't worry about this mysterious 'print' syntax yet.
print 2 + 2
print 2 + 2
```

Simply writing the expression works, but if your expression is computationally expensive or has side-effects you will run into problems.

If you were talking to a friend, you could say "calculate an expression... let's name its result 'a'... now print a, then print a again."

Well, the compiler is your friend, and your way of expressing this thought is through a binding.

```nano
let a = 2 + 2

print a
print a
```

A binding doesn't necessarily correlates to a memory allocation.
What it is: a semantic tool in your conversation with the compiler.

In fact, this code is likely to compile to the equivalent of:

```nano
print 4
print 4
```

This optimization can be made because the choice of bindings and values well informs the compiler of your 'intent' for the program.

If you create a binding that holds a structure value, its valid member accesses are themselves considered bindings.

```nano
let a = { name: "William", age: 20 }

a.name  # A binding!
```
In that way, the previous code is exactly equivalent to:
```nano
let a_name = "William"
let b_age = 20

a_name
```

nano has some great syntactic sugar regarding named-property structures and bindings:

#### Member with same name as binding

```nano
let foo = 3
let bar = 5

let a = { foo, bar }
# { foo: 3, bar: 5 }
```

#### Destructuring into local bindings

```nano
let a = { foo: 3, bar: 5 }

let { foo, bar } = a

print foo # 3
print bar # 5

# (Destructuring is a case of pattern matching, which you'll learn about later.)
```

#### Accessing while keeping structure

```nano
let a = {
    name: "Claire"
    age: 18
    items: [
        { name: "Cap", count: 5 }
        { name: "Dice", count: 10 }
        { name: "Glasses", count: 1 }
    ]
}

a.name
# "Claire"

a.(name)
# { name: "Claire" }

a.(name, age)
# { name: "Claire", age: 18 }

a.items.0
# { name: "Cap", count: 5 }

a.items.(0)
# [ { name: "Cap", count: 5 } ]

a.items.(0, 2)
# [ { name: "Cap", count: 5 }, { name: "Glasses", count: 1 } ]

a.items.0.name
# "Cap"

a.items.*.name
# [ "Cap", "Dice", "Glasses" ]

a.items.(*).name
# [ { name: "Cap" }, { name: "Dice" }, { name: "Glasses" } ]
```
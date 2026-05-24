Imagine a universe consisting of **all types**.

We can study this universe algebraically by equipping it with operations on these types that follow laws.

---

## Monoids
Within this universe, we can define **monoids** $M$ consisting of:

1. A binary operation $\star_M$ on types satisfying:
    1. **Closure**: $A \star_M B$ is itself a type in the monoid;
    2. **Associativity**:  $(A \star_M B) \star_M C \equiv A \star_M (B \star_M C)$,  which allows us to omit parentheses and write  $A \star_M B \star_M C$;
2. An **identity element** $M_0$ such that  $A \star_M M_0 \equiv A$.

Here's two examples of monoids:

1. $A \times B$ with identity $1$,
2. $A + B$ with identity $0$.

---

Some monoids $(M, \star_M, M_0)$ admit an operation  $\text{map}$ that takes a structure $A \star_M A$ and a function  $f : A \mapsto A'$ such that mapping **distributes over the monoid operation**:

$$
\text{map}(A \star_M A, f) = f(A) \star_M f(A),  
$$

Of course, $\text{map}$ only applies to homogeneous structures. If, moreover, $\star_M$ is injective, that is, for any value $c$ there is precisely one set of operands $a$ and $b$ such that $a \star_M b = c$ we can define a more general **heterogeneous** mapping operation:

$$
\begin{align}
\text{hmap}(A \star_M B,(f, g)) = f(A) \star_M g(B)\\
f : A \mapsto A'\\
g : B \mapsto B'\\
\end{align}
$$

#### Syntax for Heterogeneous Mapping

```rust
hmap <value> {
	// Here, we offer morphisms for each of the operands.
	left => ...,
	right => ...,
}

// Instead of standardizing an order for the morphisms to be given in, it makes sense to allow you to describe which of the operands (left or right) by name.
```

##### Product Types
```rust
// Named Fields
let person = Person { name: "Pedro", age: 10 };
let person =
    hmap person {
        name => name.as_uppercase(),
        age  => age * 2,
    };
// evaluates to `Person { name: "PEDRO", age: 20 }`

// Unnamed Fields
hmap [1, 2] {
    0: x => x * 2,
    1: x => x * 10,
}
// evaluates to `[2, 20]`
```

##### Sum types
```rust
// Named Fields
let project_name = Option::Some("An Adventure");
let project_name =
	hmap project_name {
		Some => Some.as_uppercase(),
		None => "Untitled Project",
	};
// evaluates to `Option::Some("AN ADVENTURE")`

// Unnamed Fields
hmap ::0(1) {
    0: x => x * 2,
    1: x => x * 10,
}
// evaluates to ::0(2)
```

#### Reinterpreting Syntax
Under this perspective:
- `for` is a special case of mapping over a **homogeneous type**, effectively the functorial map;

```rust
let functor = (1, 2, 3);
let functor =
    for item in functor {
        item * 2
    };

let functor = Option::Some(3);
let functor =
	for item in functor {
		item * 2
	}
```

- `if` is a special case of mapping over a **two-variant sum type** carrying unit data;
	- `if let` maps over a two-variant sum type where one variant might carry non-unit data.

```rust
enum Bool {
    True,
    False,
}

let value = Bool::True;

if value {
    // Bool::True
} else {
    // Bool::False
}

// This should also work for anonymous enums:
if ::0(()) { ... } else { ... }
```

---

## Semirings
If you equip the universe with two monoids

- a "product" monoid $(\times, \mathbf{1})$,  
- a "sum" monoid $(+, 0)$,

satisfying the additional laws:

1. **Distributivity**:  $A \times (B + C) = (A \times B) + (A \times C)$;
2. **Absorption**:  $A \times 0 = 0$;

you get a semiring!
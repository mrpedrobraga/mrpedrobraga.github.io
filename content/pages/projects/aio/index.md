---
title: aio (0.0.0)
description: Constraints-based programming language with a focus on expressiveness and correctness.
tags:
  - software
  - compiler
status: To Do
dependencies:
  - proj
---
<center><a href="https://github.com/mrpedrobraga/aio"><img src="https://img.shields.io/badge/github-repo-blue?logo=github"></a></center>

"Aio" is a word from latin which means "to assert."

In Aio, you write programs by asserting facts about them.

---
Aio is:
- **Declarative** — you make statements about the program's nature, and those are materialised by the compiler;

- **Functional** — you write programs by composing functions to transform values;

- **Constraints-based** — invariants are everything;

---
Everything starts with a file.

In Aio, a file is a collect of "statements," boolean expressions that render your world.

The simplest possible statement is one that defines a new "entity."

```rust
let p;
```

---
An entity has a  "value," about which we can make assertions.

```rust
let p;
p == false;
```

You can then probe the world by, for example, evaluating an expression.

```rust
@query p;  // false
```

---

> [!tip] An aside...
> It's important to note that these queries are evaluated at _build_ time, on the development environment.
> 
> It's recommended to make use of queries and statements to make sure your program has the properties you want it to have.
> 
> It's like the concept of a "type hints," and of "testing," but generalised.

---
By default, an entity's value "could be anything." A number, a truth value, an image, etc — it is undetermined. In a world with several entities, you could have any combination of values...

```rust
let p;
let q;
```

The more statements you make about the world, the more you constrain it.

```rust
if p then q;
not q;

@query p; // false
```

---
Each statement is like a "filter" that discards all possibilities that disagree with it. 

| `p`   | `q`   | `if p then q` | `not q` | `@query p` |
| ----- | ----- | ------------- | ------- | ---------- |
| false | false |               |         | false      |
| true  | false | 🚫            | -       | -          |
| false | true  |               | 🚫      | -          |
| true  | true  |               | 🚫      | -          |

It carves at reality, like a sculptor to marble, to reveal the artwork hiding inside.

---
### Aio is Functional

Procedural languages allow you to combine procedures to be run one after another, each reading from the environment and mutating state, as a basic building block of programming.

```c
void main () {
	do_this(); // mutates state — we call that an "Effect"
	do_that();
	do_something_else();
}
```

---
In functional programming, there are no implicit side effects, there is only evaluation of expression.

```rust
fn lerp(a, b, t) => (
	(1 - t) * a + t * b
)
```

All functions are "pure," all bindings are "immutable," etc.

Though it would be wrong to say the language has "no effects." Per the contrary.

---
In functional programming, thus in Aio, effects are promoted to values. They are "first-class."

```rust
fn main => (
	seq[ do_this, do_that, do_something_else ]
)
```

Values are cool, because being part of the language, they can be combined and manipulated and "reasoned about."

---

For example, it's easy to imagine combining these effects using `all`...

```rust
fn main => (
	all[ do_this, do_that, do_something_else ]
)
```

...in which case they can no longer depend on each other but can run concurrently.

---
Effects being values plays really nicely with the fact that queries are executed at build time.

No effects will be executed at compile time, since they are only values. But the compiler can look into these effects, understand what they do, and "store them for later."

---
For example, compiling a program to an executable is just a query. You query a value such as `main` and assuming it evaluates to an effect, it'll do it.

```rust
let main = print("Hello, World!")

@build main
```
```bash
aio build
```

Understand: `print(...)` is evaluated at build time. It doesn't print anything then though, it returns a value which is then serialised into an executable.

Running _that executable_ will print something.

---
It's like the difference between labels in assembly, and first-class functions. Labels are just an artifact of the syntax and can not be held or talked about in the program.

You could say Aio has "first-class effects."

> Later on we'll see more advanced... superpowers that first-class effects have.

---
## Aio is Expressive

Aio has incredibly powerful expressions, allowing you to constrain your world in powerful ways.

---
Naturally, it has basic syntax with operators and scopes and whatnot.

```rust
// Logic
a and b;
a or b;
not a;
if p then q [else r];
...

// Arithmetic
1 + 1
2 * 7
8 / 4
```

---
When defining entities, you can "organise" them by "associating" them into other entities.

```rust
let user;
user.name = "Pedro";
user.birth_year = 2003;
```

> This looks a lot like fields in a structs or objects, but unlike those cases, association implies nothing about data storage.
> 
> This is all about scoping and exists at build time!

---
And, of course, it has parametric declarations a.k.a. functions. These are declared with `fn` and take "parameters," which are undefined entities.

However, like this, parameters are mostly* useless.

```rust
fn foo(a, b) => (
	// ...
)
```

---

Because the function has to work for any entities passed as parameters, we can't see associated names, nor what constraints are applied to them. That's a problem...

Luckily, there's something that can help us!

---
### Patterns

Patterns are a special kind of constraint that can be asserted upon entities.

```rust
// Assuming a Pattern named "Pat".

// Declaration guard
let x: Pat;

// Input guard
fn foo(a: Pat);

// Output guard
fn foo(): Pat;
```

---
Patterns are Aio's analogue for "types" in other languages.

You can think of Patterns as being sets of values... where entities that satisfy the Pattern are in the set, and entities which don't are not.

```rust
// Assuming a pattern named "Dog"

let bandit: Dog;

// Semantically, we say "bandit is Dog".
@query x is Dog;
@query x isnt Dog;
```

---
There are three kinds of base patterns you can create, though they are all equally patterns.

- Predicates;
- Abstracts;
- Adhoc Patterns;

---
#### 1. Abstracts

Abstracts are bundles of declarations of associated names. So far, they've been omitted from all code samples, but, really, they are _everywhere_ in Aio code.

```rust
abstract Named {
	let name;
}

fn greet(who: Named) => (
	// we can access `.name`!
	print("Welcome, {who.name}!")
)

let a: Named;
a.name = "Soren";

let b: Named;
b.name = "Hildraed";

@query greet(a);
@query greet(b);
```

---
Abstracts are "semantic," that is to say, whenever a declaration guards using an Abstract, the compiler will only accept entities that implement that exact abstract, even if they have similar associated names within!

```rust
abstract MusicalPitch {
	let note;
	let duration;
}
let a;
for a impl MusicalPitch {
	let note = "F4";
	let duration = 3;
}

abstract Reminder {
	let note;
	let author;
}
let b;
for b impl Reminder {
	let note = "Do the dishes...";
	let author = "Pedro Braga";	
}

let c;
c.note = 4;

// `a.note` and `b.note` and `c.note` look similar
// but are semantically distinct!
```

---
Violating a guard prevents a program from compiling, so you can't do accidental mix-ups!

```rust
fn foo(x: Reminder) {
	print("Remember to do {x.note}.")
}

@query foo(a); // Will not even compile.
// You can not pass a `MusicalPitch`
// where a `Reminder` is required,
// even though the function looks for a name "note."

@query foo(c); // Also won't compile.
```

---
Abstracts can house any declarations, including functions.

```rust
abstract Animal {
	fn likes_food(food_name: Text): truth;
}

let dog;
for dog impl Animal {
	fn likes_food(food_name: _): _ => (
		food_name == "dog food"
	)
}

let cat;
for cat impl Animal {
	fn likes_food(food_name: _): _ => (
		food_name == "fish"
	)
}
```

---
#### 2. Predicates

Predicates are just functions that return true or false. Simple guards that can interact with top-level statements.

```rust
fn is_even(x);
fn is_divisible_by_four(x);

for all x => if is_divisible_by_four(x) then is_even(x)

fn foo(a: is_divisible_by_four) {
	@query is_even(a)  // true
}
```

They are most useful when combined with other kinds of Patterns, though.

---

Example.

```rust
let my_list = list [1, 2, 3]
let first = my_list.first_index(); // 0
let last = my_list.last_index();   // 2

let idx = if ... then first else last

my_list.at(idx) // <--- this
```

Usually, to make sure the access makes sense, `at` would perform "bounds-checking" and check at runtime that `idx` is within `0` and `.size - 1`, which has some performance cost.

---
But considering the indices came directly from the list, there shouldn't be any need for any bounds checking.

In Rust, the only ways to bypass this are by using `unsafe`, which is intimidating, especially if you refactor the code and the assumptions are no longer true.

Aio uses predicates here as a way to guard the list.

---
You can declare an `IndexOf` predicate...
```rust
fn IndexOf(container, x): truth;
```

You can imagine the index functions returning an `IndexOf(my_list)`...

```rust
let my_list = list [1, 2, 3]
let first: IndexOf(my_list) = my_list.first_index(); // 0
let last: IndexOf(my_list) = my_list.last_index();   // 2

let idx = if ... then first else last

my_list.at(idx) // no bounds checking :-)
```

The use  `IndexOf(self)` as a guard in `at`.

```rust
abstract List {
	fn at(idx: IndexOf(self)) { ... }
}
```

---
That guarantees safe access without any bounds checking.

If we don't have an `IndexOf(my_list)` we can create one ourselves by performing bounds checking manually!

```rust
fn get_nth_element(unknown_list, unknown_idx) => (
	if IndexOf(unknown_list, unknown_idx) then (
		// `unknown_idx` becomes `IndexOf(unknown_list)`
		// only within this scope
		unknown_list.at(unknown_idx)
	)
	
	unknown_list.at(unknown_idx) // won't compile
)
```

---
#### 3. Ad-hoc Patterns

And last, but not least, "ad-hoc" patterns. Those are your conjunctions, disjunctions, sequences, mappings, etc.

They are Aio's way of expressing structure!

---
There is `any` which matches anything. It is equivalent to having no guard.

```rust
let x: any = "Seventy"
let y: any = 32
```

---
Then there are the structural patterns.

These can be used as guards like all patterns, but also as values:

```rust
let x: Bird;
let y: Lizard;

// a sequence a.k.a. a tuple.
let sequence: [ Bird, Lizard ] = [ x, y ];
```

---
```rust
// a conjunction.
let con: all[ Bird, Lizard ] = [ x, y ];

// a disjunction.
let dis: either[ Bird, Lizard ] = .0[ x ];
```

---
There are a few interesting "fragments" of structural patterns, which are useful with another feature: pattern matching.

--
We have Optionals...

```rust
let foo: seq[ number?, text ] = [ 3, "Hi" ]
@query foo.0;  // Some(3)
@query foo.1;  // "Hi"

let bar: seq[ number?, text ] = [ "Hello" ]
@query bar.0   // None
@query bar.1   // "Hello"
```

--
Many...

```rust
// repetition in a sequence
// take note of pattern coercion
// a [ number, number, number, number, number, text ]
// fits into a [ number..., text ]
// but it gets "coerced" into the new pattern
let foo: seq[ number..., text ] = [ 1, 2, 3, 4, 5, "hi" ];

// the 0th element of `foo` isn't "1"
@query foo.0; // [ 1, 2, 3, 4, 5 ]
@query foo.1; // "hi"

let bar: seq[ text...{2}, truth ] =
	[ "Hello", "There", false ];
```

--
Captures.

```rust
let foo: [ let first, let rest: any...? ] = [ 0, 1, 2, 3, 4, 5, 6 ]
@query foo.first; // 0
@query foo.rest;  // [ 1, 2, 3, 4, 5, 6 ]
```

If you're only interested in the captures of a declaration, you can omit the name and "splat" the captures onto the current scope.

```rust
let [ let first, let rest: any...? ] = [ 0, 1, 2, 3, "Four", 5, 6 ]
@query first; // 0
@query rest;  // [ 1, 2, 3, "Four", 5, 6 ]
```

--
Captures can be referenced by later fragments in a sequence.

```rust
fn eq(y, x) => x == y;

let foo: [ let a, eq(a) ] = [ 42, 42 ];
```

But not in a conjunction.

```rust
let foo: all[ let a, eq(a) ];  // Won't compile, no `a` in scope.
```

---
Instead of failing with a hard error, you can match patterns (and have access to captures) in a "hypothetical context."

```rust
let foo = [1, 2, 3, 4, 5]

if let [let first: number, any...] = foo then (
	@query first; // 1
)

match foo [
	[ let first: text, any... ] =>
		@query first /* 1 */,
	[ any, any, let third: eq(3), any... ] =>
		@query foo.2 /* 3 */,
]
```

---
With patterns, pattern combinations and pattern matching you can do arbitrary control flow.

The funnest use case is for parsers. Aio has `standard.patterns`, as library with useful patterns to combine.

```rust
// a text is effectively a [ char...? ]
let input = "1 + 2 + 3";

use standard.patterns.(NUMERIC);

let LITERAL = NUMERIC;
let OPERATOR = "+";
let BIN_OP = seq[ EXPRESSION_L, OPERATOR, EXPRESSION ];
let EXPRESSION_L = LITERAL;
let EXPRESSION = either[ BIN_OP | EXPRESSION_L ];

if let expr: EXPRESSION = input then (
	// parsing was successful, do things
	match expr [
		.0(let lit) => ...
		.1(let bin_op) => ... 
	]
) else (
	// Some parsing error.
)
```

---
If you think of Patterns as sets, you can declare analogues for operations over sets.

You can combine patterns to make more complex ones.

```rust
not Foo;
Foo and Bar;
Foo or Bar;
Foo and not Bar;
```

---
You can match Patterns against each other.

```rust
// If "Foo" is a subset of "Bar", that is,
// for all x if x is Foo then x is Bar
Foo subs Bar;

// You may also read this as "Foo requires Bar"
// or "Foo implies Bar".
```
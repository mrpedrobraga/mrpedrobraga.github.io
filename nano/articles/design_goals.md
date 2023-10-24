## Design Goals

nano began as an art project, studying the psychology and the culture behind programming language design. It has a few goals, which interact with each other to create what 'nano' is.

### nano is expressive

Everything in nano is an expression. There are virtually no 'statements', i.e., everything evaluates to a value.

```nano
let a = true

let b = if a then 3 else 4

let c = for i in 0..10 select i * 2
```

nano is designed to be great to think in -- you may catch yourself prototyping things in nano before writing them in other languages.

```nano
if clicked_login but not online then (
    ...
    let is_even = fn x -> x % 2 == 0
    let a = list::from_range(0..10)
    let evens = select num for num in a when is_even(num)
    let sum_of_evens = for acc, num across evens select acc + num
    #                                 ^ reduces!

    let result = await some_function(view a)
    #                                 ^ immutable reference
)
```

### nano is simple... but not that much

nano's naming schemes are more verbose, but the names are chosen with great care, and in the end the keywords are more meaningful than symbols.

While I try to keep the language thin, I'm not afraid of adding new syntax if it helps make the language more expressive... so nano ends up having lots of keywords. Well, nano's not LISP;

```cpp
// C
int a = true
int c = &a
int d = (int)(*c || a)
```

```nano
# nano
let a = slot(true)
let c = a
int d = (c or a) as int
```

### nano is unique

On this day and age, it doesn't make sense to create a new programming language just to be another C/Rust clone.

While nano tries to borrow features and design choices from languages I admire, nano isn't afraid of reinventing the design wheel somewhat.

```nano
# An example of this is that your tuples, strings and structs
# are by default immutable (and passed by copy).
let a = [1, 2, 3]

a[0] = 4
# ^ Can't assign to a : [ int, int, int ] because it is immutable.

# The variables themselves are not immutable.
a = [3, 5, 6]

# This might make you frown if you came from Rust,
# where the need for the 'mut' is all the hype.

# nano isn't rust, and I went another way.
# If you keep your functions small and simple the way you should,
# it is easy to just 'not assign' to a variable.
# You can't mutate values across functions, after all.
```

### nano is sane

Look at all those beautiful non-highlighted words:

```nano
null
undefined
try
catch
throw
```

nano is throroughly strong-typed and it respects your data and control flow. You won't find functions that randomly return random values or that break the execution of your code.

Instead, it uses 'nothing', data enums and 'Error'.

```nano
let a : int = nothing
#               ^ Can't assign value of type 'nothing' to receiver of type 'int'.

let my_float = float::from_string("Banana")
#    ^ my_float is of type float!ParseError

if my_float is ParseError then (
    print "There was an error"
    return
    # Handle your error however you want
)

my_float   # The type of 'my_float' is narrowed to be 'float' here.
```

It also imposes helpful restrictions on your reference types regarding lifetime-based-ownership and cross-function-mutability to help you keep your code safe and reliable.

```nano
# Malicious function that asks for a mutable reference to an int.
fn give_me_an_int_to_look_at(a: slot<int>) -> (
    a = 10
    #   ^ mutates the value, then laughs maniacally
)

let num = slot(5)

give_me_an_int_to_look_at(num::view)
#                               ^ Passing value of type 'view<int>' but the function asked for 'slot<int>'.

# Semantically, if a function asks for a slot instead of a view,
# you can assume that it mutates the value.
```

#### nano has a thin output

nano is a compiled language, so, there's no such thing as a 'nano runtime'. It _can still be interpreted_, but even interpreted nano gets a benefit from nano being designed as a compiled language.

You see, Javascript bases its values and structures around an implementation of hash maps it calls 'objects'. "Everything is an object."

In nano, most things aren't hash maps, instead, they are data tightly packed side by side.

A struct containing two i32 properties like this:

```nano
{
    a: 45
    b: 90
}
```

Occupies exactly 2 \* sizeof(i32) bytes in memory.

There's also no garbage collector, memory is managed by the reference primitives, which impose rules to force you to use them in safe scenarios, and then free themselves accordingly.

```nano
(
    let a = slot(10)

    function_that_does_smth(a)

    print a  # 7

    # Scope ends, a gets freed.
)
```

```nano
(
    let a = slot(10)

    async_function_that_does_smth(a)

    print a
    #     ^ Symbol a : slot<int> was given to 'async_function_that_does_smth' and can't be reliably accessed from this scope.

    # (That's because if it wasn't given away, when
    # the scope ends here, the reference would become invalid.
    # Instead, it's freed when async_function_that_does_smth
    # finishes.)
)
```

```nano
(
    let a = slot(10)

    await async_function_that_does_smth(a)
    # ^ by awaiting the async function, you guarantee
    # that this scope outlives the called function's usage
    # of 'a', and, therefore, it's safe to not give 'a' away.
    # It is 'borrowed' instead.

    print a

    # a gets freed here
)
```

You can manage larger chunks of memory by preallocating it into buffers.

```nano
let buf = stack(10 * MEGABYTE)

on buf let a = 10
on buf let b = 20
```

### nano is technical

While choosing simpler naming for concepts and making the language easy to learn is important, nano respect's the programmer's intelligence, expertise and willingness to learn new things.

That is, it doesn't talk down with its terms, and will be transparent about what the compiler is doing.

```nano
let a: array<int> = [ 1, 2, 3, 4, 5 ]

a::push(6)
#   ^ method `array<int>::push` is declared but unusable
#     because array<int> doesn't satisfies the 'heap_allocated'
#     constraint (Use `list<int>` instead).

let b: list<int> = [ 1, 2, 3, 4, 5 ]

b::push(7)
```

Even though you didn't `malloc` yourself, you will be made aware when you're allocating things on the heap v.s. the stack.

nano makes it easy to write code that doesn't abuse your system's resources, so you can get used to preferring `slot` and `view` and sleep well knowing that they're never allocated on the heap without you knowing.

### nano is sufficient

`nnc`, the nano compiler, contains all it needs to understand and run nano... so it can do more than just compile.

Build system? Build with nano's annotations. They are similar to C's macros, but operate on AST instead of text, meaning you can even create your own.

```nano
%%is_target ASM_X86 (
    import 'library' as MyLib

    let c: i64
)

%%else (
    import 'otherlib' as MyLib

    let c: i32
)

c = 30

print c
%%run print "Compilation finished!"
#  ^ statement runs in compile time
```

Tester? Test with nano!

```nano
fn add (x, y) -> x + y

test $"Addition works" -> (
    assert add(3, 4) == 7
    assert add(10, -1) == 9
)

###
nnc test

-- Addition works : (2/2) --
PASSED   add(3, 4) == 7
PASSED   add(10, -1) == 9
###
```

Transform? Transform your code with macros (and LSP code actions).

```nano
# Macros can query and mutate your code.

let c: i8 = 10

##% (SYMBOL c) ever ASSIGNED ?       false
##% (SYMBOL c).type ?                i8
##% (SYMBOL c).type = (SYMBOL i32)
```

Linter? Lint with macros and tests.

```nano
test "No Local Variable Type Inference" -> (
    assert ###%
        (every SYMBOL x where x is LOCAL_VARIABLE)
            .declaration?.type != INFER
    %###
)
```

Deploy? Link stuff as if they were libraries and call functions to deploy.

```nano
from 'nnc' import { run_tests }
from 'git' import { repo, push }

fn build -> (
    let test_results = await run_tests()

    if not test_results.all_passed then err "Some tests failed."

    await deploy()
)

fn deploy -> (
    return await repo('./target/web-html').push_to('dev')
)
```

You can create custom importing protocols for nano, meaning that, potentially, you can import literally anything.

```nano
import 'image.png' as MyImage
from 'data.json' import { datum1, datum2 }
```

Package Manager? It's nano all the way down, buddy!

```nano
import 'http://weather.api'
# Fetch and cache packages over the network...

# Libraries can come with their own protocols for importing...
import 'npm'
import 'npm@datascript-nn'

# Create your own alias on a nano file to avoid repeating yourself.
global import alias 'npm@datascript-nn' as 'datascript'
```

### nano is FOSS

The silver lining of this project being created by some random guy on his free time is that this project isn't supported by mysterious corporation!

The code for the compiler (which is by the time of this writing incomplete and written in rust) is here.

My distribution of it uses [the MIT license](https://opensource.org/license/mit/) here at [git://mrpedrobraga/nnc](https://github.com/mrpedrobraga/nnc).

Me mentioning the MIT license here is not legal advice.

Futurely, the compiler will be written in nano, so I can brag about that.

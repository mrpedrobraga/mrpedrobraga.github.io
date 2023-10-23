## Reference Types

Consider this code snippet:

```nano
let a = 3
let b = a

a = 10
print b  # 3
```

It may come a time when you want not to 'copy' a value into another variable, but want to _share_ it and _keep sharing_ it.

As in, you can change that value later and all parties that share were given this value feel the change too.

For that, nano has 'reference types.' The first one you'll see is 'view&lt;T&gt;'.

```nano
let a = view<int> 3
##% ^ TYPE?    view<int>

# Notice that view<T> is an incomplete type!
# The "template parameter" T is needed to complete it.

let a = view 3

# You can omit the template parameter in this case, though,
# because it can be easily inferred from '3' t be 'int'.
```

When a view is passed to another value, the view itself is copied over...
But the underlying value the view is... uh... viewing, is NOT duplicated.

```nano
let a = view 3
let b = a

print b   # view( 3 )

a = 4

print b   # view( 4 )
```

If you used a language like C, you'll know this as a pointer.

```cpp
int a = 3
int* b = &a

printf("%d", *b)

a = 4

printf("%d", *b)

// But, of course, the C syntax looks like alien writing.
```

Note that you can't change the value of a from b, you can only 'view' it.
To allow b to change the value you can use a 'slot'.

A slot is something you can 'put things in'. Get it? Hahaha!

Anyways,

```nano
let a = slot 10
let b = a

a = 50
print b  # 50

b = 100
print a  # 100

# Both sides can 'put something in' the slot
# or look at what's in the slot.

# Since 'a' and 'b' are accessing the same 'slot',
# they will read and write to the same place in memory.
```

You may note a similarity between 'slot' and 'alias'.
They couldn't be more different though.

An alias is just a local nickname for a variable. A slot is a real thing that will exist on your program, and it is non-local, i.e., you can share slots and views across scopes.

```nano
let a: slot<int>

(
	let b = slot(3)
	a = b
)

print a   # slot( 3 )
```

## Section for nerds

Wait... what?

Isn't 'b' going to be erased when the scope ends?
How can you share its value with 'a'? Isn't 'a' going to have an invalid reference?

You're... uh, right with being concerned, since the following Rust would be illegal:

```rust
let mut a: *int;

{
	let b = 3;
	a = &b;
}
```

But notice that in this rust code, you're explicitly creating a variable inside the scope, then returning its address. In nano, you create a 'slot' and the value the slot holds is implicitly placed... somewhere...

As nano can clearly see that you want to export a slot to the outer scope, it choses to preallocate space for that value on the outer scope!!!

That would be equivalent to this rust code:

```rust
let mut a: *int;

let _b;
{
	_b = 3;
	let b = &_b;
	a = b;
}
```

which is perfectly legal.

This preallocation can only happen when the type and size of the slot's value are known before it is evaluated, as well as the lifetime of the scopes.

This all imposes some fun restrictions when using operations that read external input (see [Constraints](./?article=constraints)), or asynchronous operations that can create and destroy memory scopes arbitrarily (see [Asynchronicity](./?article=asynchronicity)).

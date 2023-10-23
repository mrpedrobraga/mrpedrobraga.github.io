## Errors

We are finally here! This is the finale for the first season of our learning.

On the previous section, you saw this code snippet here:

```nano
fn count_to_x(x: int): string -> (
	if x < 0 then return "Can't count to negative numbers."

	for i in x do (
		print i
	)

	return "Counted!"
)
```

In it, I used strings to represent the success states of the count_to_x function.

nano has a better way of doing this, though: Errors!

```nano
from standard import { RangeError }

fn count_to_x(x: int) -> (
	if x < 0 then err RangeError(x, 0, INFINITY)

	for i in x do (
		print i
	)
)
```

Notice that we use the 'err' syntax to 'return' an error. This is becaus errors can be treated differently than normal returns.

You see, a function that can err returns an error union.

```nano
let source_string = "1.07A5"
let my_float = float::from_string(source_string)
#     ^ TYPE?   float!ParseError
```

In order to use the value, you first have to narrow that union using constraint narrowing.

```nano
if my_float is float then (...)
```

But this can be cumbersome, having to manually check for errors every single time an operation happens.

For that there's some ways of handling errors more efficiently.

### Error pipes

```nano
# An error pipe will only evaluate the right side if the input is an error!

let source_string = "1.07A5"
let my_float = float::from_string(source_string) ||> return
#     ^ TYPE?   float

# Because of the early return, the value will only be
# stored on 'my_float' when it isn't an error so the type can be narrowed.
```

```nano
let source_string = "1.07A5"
let my_float = float::from_string(source_string) ||> nothing
#     ^ TYPE?   float|nothing

# You might want to throw away the error by switching it
# to a 'nothing' or something else.
```

### Error handlers

Error handlers are similar to the 'try-catch' block you will find in other languages.

By using 'with &lt;error_handler&gt;' on your scope, the errors will magically disappear.

```nano
with ParseErrorHandler

let source_string = "1.07A5"
let my_float = float::from_string(source_string)
#     ^ TYPE?   float
```

And here is the error handler (it is similar to a function!)

```nano
err_handler ParseErrorHandler (e: ParseError) -> (
	print "A parse error occurred: {e.parsing_input}"
	err e
	# ^ unlike functions, err_handlers can affect the control
	#   flow on the scope that uses it.
	#   So we can force the caller function to exit,
	#   magically narrowing the type there.

	# The reason why this is safe is:
	# You have to use 'with ParseErrorHandler' on your
	# calling scope, so you are giving permission for your
	# err_handler to control that scope.
)
```

An interesting tip I can give you is to use a "PanicErrorHandler" while prototyping.

```nano
err_handler PanicErrorHandler(e: unknown) -> (
	push_error "Oops, an error occurred: {e}."
	exit()
)
```

This will let you compile test code without having to worry deeply about how your fail cases are. When you're ready to make your program good and sane, remove the 'with PanicErrorHandler' line and start to treat those failure cases!

## End of season

Phew! It's over...

Now, I promised on the beggining of this season that there was a snippet of code you'd be able to understand by the season's end.

Here it is.

```nano
# sum.nn
from core import { args }

if args.size < 2 then err SimpleError("Not enough arguments supplied!")

with err_handler (p: ParseError) -> (
	err 'Parameter {p.parse_input} is invalid.'
)

let result: float =
	for acc=0, arg across args select
		acc + float::from_string(arg)
```

With this knowledge, you can already write the majority of programs out there.

So, go write some nano!

## Asynchronicity

In nano, expressions are evaluated one after another, in what's assumed to be 'instantaneously.'

```nano
print "Hello world."
print "Hello world."
```

But you may find yourself with an action that _takes time_ to execute... perhaps a complex task parsing a big file, or a request over the network. Either way, that will be an 'asynchronous' operation -- your code won't wait for it.

```nano
fn get_file_content () -> (
	# Asynchronous shenanigans...
	print "File import finished!"
)

print "Hello world"
let file_content = get_file_content('my_huge_file.txt')
print "Hello world 2"
```

This will print something like this...

```
"Hello world"
"Hello world 2"
"File import finished!"
```

...probably.

Asynchronous operations are unreliable in the order that they'll run. You can imagine that when you call 'get_file_content' a new parallel execution context is created.

```
"Hello world"
(spawned) .........> ...loading file
"Hello world2"   |   "File import finished"
(ended)          |   (ended)
```

To do things across execution contexts, you can send messages via a reference type.

The first building block of async communication is the 'signal'.

You can use 'await' on stop an execution flow until the signal's emission. The signal can then be emitted by the other side.

```nano
let my_signal = signal()

fn get_file_content () -> (
	# Asynchronous shenanigans...
	print "File import finished!"
	my_signal()

	return content
)

print "Hello world"
let file_content = get_file_content('my_huge_file.txt')
await my_signal
print "Hello world 2"
```

The control flow looks like this:

```
"Hello world"
(spawned) .........> ...loading_file
                 |   "File import finished"
(resume) <.......|.. Signal emitted!
"Hello world 2"  |   (ended)
(ended)
```

Not only that, but at the moment where the emission happens, both flows are synchronized, too.

If this signal is emitted right when the asynchronous function's execution flow ends, that also means the caller scope outlives it.

This has very powerful implications when we're talking about reference types (as we'll see in a bit).

### async functions

Creating a signal for every asynchronous function is cumbersome. But check this out:

```nano
fn get_file_content () -> (
	...
)

print "Hello world"
let file_content = await get_file_content('my_huge_file.txt')
print "Hello world 2"
```

You can directly call 'await' on a function like this if the function returns an awaitable type, such as 'signal.'

```nano
fn awaitable -> (
	return signal()
)

await awaitable()  # Infinite waiting
```

But that's not very useful, since as the function returns the signal, it can't then emit it.

We can't emit the signal before we return it either.

```nano
fn awaitable -> (
	let s = signal()
	s()
	return s
)

await awaitable()  # Still infinite waiting
```

What we would really like is to do this:

```nano
fn awaitable -> (
	let s = signal()
	return s         # return a signal
	...
	s()              # then emit it
)

await awaitable()
#       ^^^ waits for the returned signal... then resumes when it emits
```

While this is not possible, consider this:

What makes a function be asynchronous and need awaiting in the first place?

Well, an asynchronous function is just a function that, within itself, calls 'await' on some other thing.

And something like we wanted to do with the 'internal signal' is exactly what happens when you call 'await' on something inside a function.

```nano
fn get_file_content() -> (
	await os_give_me_the_file_please
	# ^^^ When you call await, the function will actually 'return'
	#     from the call site's POV... but it'll keep working.
	#     It returns a signal, which the call site can wait on.

	print "File import finished!"

	return 3
	# ^^^ The function will emit that signal and the call site's
	#     flow will resume.
)

let result = await awaitable()
#                    ^^^ waits for the returned signal... then resumes when it emits
```

The implicit signal that a function returns has a special name, 'Expectation&lt;T&gt;' and it holds only one boolean telling you whether it was already fulfilled or not.

```nano
struct Expectation<T> extends signal<T> {
	fulfilled: bool = false
}

# You can see the expectation in practice if you don't
# 'await' the asynchronous function that you called.

fn my_func -> (
	await ...
	return 3
)

print my_func()
#       ^^^ 'Unfulfilled Expectation<int>'
```

What if you want to return something more complex than a simple 'Expectation?'

Let's say you want to encapsulate the idea of a task that is continuously made. This construct will need to have progress, signals for when progress changes or it concludes, and, of course, be awaitable.

```nano
from 'nano' import { ASYNC_OP_FINISHED }
#                     ^ This is a special builtin identifier.

struct Task<T> {
	progress: float = 0.0
	is_finished: bool = false

	signal<float> progress_changed
	signal finished

	fn [ ASYNC_OP_FINISHED ] -> finished.emit()
	#      ^ nano will look for a function with this name to call
	#        when the async function finishes.

	fn set_progress(new_progress: float) -> (
		progress = new_progress
		progress_changed(new_progress)
	)

	# Forward the awaiting to the internal finished signal.
	async get -> await finished
}
```

You can, then, override the construct a function uses from Expectation to Task by using the 'with' syntax, same as you saw with the error handler.

```nano
fn delayed_integer(): int -> (
	with Task

	await ...
	return 10
)
```

This object you just created is a synchronization handler. Like error handlers, they'll only be affecting the scope where you use the 'with Task'.

```nano
with Task # Affects all functions in this file!

fn delayed_integer(): int -> (
	await ...
	return 10
)
```

But... if the scope that receives the task is only awaiting it, the progress is hardly useful.

First, you need a way to access the task from the function to update its content.

```nano
fn delayed_integer(): int -> (
	with Task
	let ongoing_task = selffn.sync_handler
	#                     ^ If your function is asynchronous,
	#                       'selffn' will contain a reference to
	#                       the synchronization handler.

	let result = 10

	for i in 0..10 do (
		result -= 1
		ongoing_task.set_progress(i / 10)
	)

	return result
)
```

But how do you consume this progress?

Say hello to the actually useful but histerically named... meanwhile block!

### Meanwhile at the call site...

```nano
await delayed_integer() meanwhile (task: Task<int>) -> (
	task.progress_changed.connect fn -> (
		print "Progress: {100 * task}%"
	)
)
```

The meanwhile block is a way of accessing the awaited sync handler when it is awaited directly.

While in the case I just showed, you could just do this:

```nano
let task = delayed_integer()
task.progress_changed.connect fn -> (
	print "Progress: {100 * task}%"
)
await task
```

A meanwhile block can loop indefinitely... it can then stop mid-execution when the awaited operation finishes.

```nano
await delayed_integer() meanwhile (t) -> (
	loop (
		print 'waiting...'
		sleep 10

		yield
		# ^ the meanwhile block will be broken here
		#   if the operation finished...
	)
)
```

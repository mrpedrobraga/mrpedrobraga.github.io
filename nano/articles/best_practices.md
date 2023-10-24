## Best Practices

nano's design allows you to chose your own way of organizing your code. Many criticisms from experienced programmers of programming languages is that they allow other programmers to do things in ways they don't like.

Knowing that, nano still refuses to enforce stiff project structures, but you can go a long way with 'Project Rules.'

So, here's a basic idea of how you might wanna organize your files for different projects.

## Small Project

```nano
## main.nn

print "Hello world!"
```

When you call 'nnc compile main.nano', it will start from the file, executing the code from top to bottom.

To use multiple files, you use `import`, which transfer execution to the other file.

```nano
## main.nn
print "Hello"

import 'file2.nn'

print "World"
```

```nano
##file2.nn

print "Good"
```

These two files will print:

```
Hello
Good
World
```

Note that each file has their own _lexical scope_: variables declared in one file can't be accessed by another file. You need to explicitly export and import symbols between files.

```nano
## double.nn
fn double x -> x * 2
```

```nano
## main.nano
from 'double.nn' import { a, b }

print double(3)
```

With that, you can organize your code in small modules, where each file has a conceptual responsibility, and maybe one or a few files for utilities. But a small CLI program can be written in a single file no problem.

```nano
lib/
	utils.nn
main.nn
LICENSE
README
.gitignore
```

## Big Project

From big videogame projects, REST APIs, or GUI desktop software, sometimes you need a bit more scalability.

Let's say you're making a videogame.

First, an 'index' file that sets up libraries, library aliases, a few key project global constants and 'Project Rules' before you access your 'main' file.

Consider the following project structure for a small game where you can choose an item and a weapon.

```nano
lib/
	utils.nn
res/
	weapons/
		sword.nn
		bow.nn
	items/
		apple.nn
		pineapple.nn
src/
	usables/
		Item.nn
		Weapon.nn
	main.nn
tests.nano
index.nano
LICENSE
README
```

The index.nano file is the file you'll call 'nnc compile' on. Notice the '.nano' -- it's treated the same as any other nano file, but conveys the meaning of this file being a "meta" file.

```nano
## index.nano
global import './src/usables/Item.nn' as Item
global import './src/usables/Weapon.nn' as Weapon
import './src/main.nn' as Entry

# Call the main function in main.nn
await Entry.main()
```

'utils.nn' is also a normal file, but it consists mainly of symbols that it exports for other files to use. The symbols that it exports are its 'interface' with other files.

```nano
## utils.nn
export (Action)

## A function that does something and returns its success.
type Action = function<unknown, bool>
```

'Item.nn' is a type file. Don't be scared, they're still just a normal nano file in every way, but its variables are bundled and accessible from the outside as a single type -- in this case, a struct.

```nano
with self as struct Item

## @self An item that can be used by the player.

## Emitted when the item is used.
signal was_used

## Name of the item.
let name: string

## How many times the item can use used
let durability: u8

## The action that's called when the item is used.
let usage_action: Action

## Use the item.
fn use -> (
	let usage_successful = usage_action()
	if usage_successful then (
		durability -= 1
	)
	was_used()
)
```

The 'apple.nn' file is similar to a type file, it is a 'resource file'. Again, they're just normal nano files in every single way, but they are importable as if they were an instance of a struct.

```nano
with self as Item

name = "Apple"
durability = 1
usage_action = fn -> "Ate apple!"
```

Finally 'main.nn' is a standard nano file which can use all the types and resources declared.

```nano
## main.nn
import Item
import './res/items/*' as ItemDatabase

fn main -> (
	print("What item would you like to use?")
	let item_name = prompt('> ')

	if not '{item_name}.nn' in ItemDatabase then return

	let item: Item = ItemDatabase['{item_name}.nn']::copy()
	item.use()
)
```

## Project Rules

If you want to define how things should look and feel on your project, binding all team members to the same style rules, you can create test suites with queries inside.

'Macros/Queries' are powerful compile-time introspectional tools nano gives you to ask questions about your codebase. They work on an AST/IR level, so you can ask questions about symbol declarations, symbol usage, references, even doc comments.

```nano
## formatting.nano

## Variables must be named in snake_case.
test "Local Variables In Snake Case" -> (
	assert ##% (every SYMBOL x where x is DECLARATION and x.kind == LET) |>
		match _.name with r/[a-z_]+/
)

## Types must be named in TitleCase.
test "Local Variables In Snake Case" -> (
	assert ##% (every SYMBOL x where x is DECLARATION and x.kind == TYPE) |>
		match _.name with r/[A-Z][a-zA-Z]*/
)
```

When these test suites run they'll give a detailed (and commented) explanation of everything that's wrong with your styles.

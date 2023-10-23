## What is nano?

nano is a compiled, strong-typed, multiparadigm programming language.

It began as an art project, trying to design a language that is both fast and safe as well as clear and elegant-looking.

Check out this tiny hello world program!

```nano
print "Hello world!"
```

Another code sample, a theoretical implementation of an ordered map in nano:

```nano
## Map struct that stores the entries.
struct Map {
	entries: list<[string, string]>
}

## Simple struct for failure case of finding a string.
struct KeyNotFoundError : Error { map: Map, key: string }

fn store (self: Map, key: string, value: string) -> (
	for [k, v] in self.entries do (
		if key == k then (
			v = value
			return
		)
	)

	entries::push [key, value]
)

fn fetch (self: Map, key: string) -> (
	for [k, v] in self.entries do (
		if key == k then (
			return v
		)
	)

	err KeyNotFoundError(self, string)
)
```

Maybe those examples inspire you to learn some nano?

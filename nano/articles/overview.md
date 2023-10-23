## What is nano?

nano is a compiled, strong-typed, multiparadigm programming language.

It began as an art project, trying to design a language that is both fast and safe as well as clear and elegant-looking.

Check out this tiny hello world program!

```nano
print "Hello world!"
```

Another code sample, a theoretical implementation of an ordered map in nano:

```nano
from collections import { KeyNotFoundError, keyof }

## Simple struct that stores entries relating "keys" to values.
## @generic_param K The type of the keys of this map.
## @generic_param V The type of the values of this map.
struct OrderedMap<K, V> {
	entries: list<[key: K, value: V]>

	has (key: K |- keyof self): bool implies key |- keyof(self) -> (
		some k == key for [k, _] in entries
	)

	get (key: K |- keyof self): V!KeyNotFoundError<MyMap, K> -> (
		for [k, v] in entries do (
				if k == key then return v
		)
		err KeyNotFoundError(self, key)
	)

	set (key: K, value: V) -> (
		for [k, v] in entries do (
			if k == key then v = value
		)
		entries.push [key, value]
	)
}

test "Map is Sane", (
	let map = OrderedMap<string, int|bool>()

	map["Key 1"] = 10
	let randomkey = "Key {randi()}"

	let result: int|bool!KeyNotFoundError = map[randomkey]

	assert (
		if randomkey == "Key 1" then
			(result is int|bool)
		else
			(result is KeyNotFoundError)
	)
)
```

Maybe those examples inspire you to learn some nano?

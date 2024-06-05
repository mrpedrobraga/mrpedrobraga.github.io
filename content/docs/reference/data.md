## Data

Every `.sol` file is an "asset."

The first way you can use them is for standalone assets. These can contain arbitrary data in the form of key value pairs.

```sol
-- DIR TREE:
-- assets/
--   game_info.sol

name = "The Legend Of Brad"
version = "0.0.1"
author = "You"
```

Each one of these key-value pairs is called a "field" and contains some unchangeable data. In this case, the field `version` has a datum of type `Text` and content `"0.0.1"`.

For this use case, a sol asset isn't that different from something written in TOML. But the fun hasn't begun yet.

### `using` other assets

You can statically refer to other assets in the sol workspace. This guarantees an asset reference is always valid.

```sol
-- DIR TREE:
-- assets/
--   rooms/
--     HeroHouse.sol
--   game_info.sol

using rooms

name = "..."
version = "..."
author = "..."
starting_room = rooms.HeroHouse
```

We will get in depth how the `using` syntax works in another chapter.

### `model` our woes away

We just imported a room from the `game.rooms` asset. Perhaps we modeled `HeroHouse.sol` as standalone asset. But it would be good to enforce on us some kind of standardization in how rooms are defined, so that the sol compiler will help us keep everything in order.

For that, we create a `model`.

```sol
-- assets/rooms/mod.sol

model Room
  field display_name: Text
  field player_spawn_position: vec2
end
```

> Don't stress about what `Text` and `vec2` are yet.
> We'll see about field "types" in a future chapter.

Now, when we create new rooms, we will `impl` that `model`.

```sol
-- assets/rooms/HeroHouse.sol
impl Room
  display_name = "Hero's House"
  player_spawn_position = vec2(10, 10)
end
```

And the compiler will warn us if we make a mistake.

```sol
-- assets/rooms/HeroHouse.sol
impl Room
  display_nmae = "Hero's House"
  --   ^^^ Field 'display_nmae' not found in model `Room`.
  --       hint: Did you mean `display_name`?
  player_spawn_position = vec2(10, 10)
end
```

To avoid indentation we can turn the entire file into the `impl` block.

```sol
-- assets/rooms/HeroHouse.sol
...impl Room

display_name = "Hero's House"
player_spawn_position = vec2(10, 10)
```

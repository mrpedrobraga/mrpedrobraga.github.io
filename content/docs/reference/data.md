## Data

Sol is a programming language for assets. With it, you can create your assets in expressive and powerful ways. In the end, assets are just "data" your application reads, but while in `.sol` form, they are much more.

If you want to create an asset, a `.sol` file is the first step. Every "asset" "implements" a "model."

### Models

A model is a contract that many assets will be beholden to. Imagine you are making a game, which contains several items definitions (a "kind" of item the player can have). Something like "Iron Sword" or "Leather Boots."

Each item definition should be well-formed and provide information of how the item will be displayed in game and what it does. A `model` describes that.

```sol
-- We have many items in the game,
-- each kind of item is an ItemKind

model ItemKind
  label: text
  durability: nat
  icon: Image
  on_use: Action(Character)
end
```

A model contains several "fields," each one with a name and a contract (we call those "types"). The first field is named "label" and is of type `text`.

The second field is of type `Option(nat)`: `nat` is just a natural number (0, 1, 2, 3).

Image is an image, another asset. You can refer to other assets, even if they are not written in `.sol` given that your backend has an extension to handle the file type. In this case you can imagine the "image" is a PNG file.

`Character` is another model on the same file. A model you created is a valid type!

Lastly, `Action(Character)` is an `action` with a parameter of type. We'll see how to provide a value of this type soon.

### Asset

A model can be implemented by an asset (on another `.sol` file) with the `impl` keyword.

```sol
using game.models.(Character, ItemKind)
using game.sprites.icons.iron_sword

impl ItemKind
  label = "Iron Sword"
  durability = 4
  icon = iron_sword

  action on_use(target: Character)
    -- Give the weapon for the player to equip.
    target.try_equip_weapon(weapon: self)
  end
end
```

> Note that every `.sol` file _is_ its own asset.
> Even files that only define models and don't implement anything count as assets. You can think of those as "modules."

> Also, notice the `using` keyword.
> Files must **explicitly** refer to each other, letting you (and the sol toolchain) understand the relationships between files easily.

Here we have provided data to be displayed in the inventory of the game. Now, our item definition can, itself, be used in another model.

```sol
-- A player might have, like, [3x Iron Sword] in their inventory.
model ItemStack
  field count: nat
  field kind: ItemKind
end
```

We also provided a value for the `on_use` field with a proper action. Inside of it we do some behaviour, but we're not going to get into that here.

But, wait, we need to attack with the sword, too, right? We call `target.try_equip_weapon(weapon: self)` -- it would be great to have something that describes what it "means" to be equipped.

That would be another model, which we could also implement for the same asset.

```sol
using game.models.(
  Character,
  ItemKind,
  EquipmentKind,
  EquipmentCategory
)
using game.sprites.icons.iron_sword

impl ItemKind
  label = "Iron Sword"
  durability = 4
  icon = iron_sword

  action on_use(target: Character)
    target.try_equip_weapon(weapon: self)
  end
end

-- You can implement multiple models for the same asset.
impl EquipmentKind
  category = EquipmentCategory.WEAPON

  action on_attack_with(actor: Character, target: Character, battle: Battle)
    target.stats.apply_damage(10HP)
  end
end
```

> Each `.sol` asset file might actually represent several assets (one for each `impl`) if your backend doesn't support the concept of models.

### Groups

An asset might not itself implement models, but might contain things inside.

One of those cases is functions, for computations:

```sol
-- We won't get in the detail about functions
-- right now so enjoy this simple function.

-- Returns the sum of two numbers.
fn add(x: nat, y: nat): nat
  return x + y
end
```

And actions (which are almost identical to functions), which you can use to make scenes.

```sol
using game.interaction.Chest

-- Open chest
action open_chest(actor: Character, chest: Chest)
  * You opened the chest.
  * There was {chest.item.count}x {chest.item.kind.display_name} inside.
  actor.inventory.give_stack(chest.item)
  chest.opened = yes
end
```

And also `def`s, which creates an asset implementing a model and puts it "inside" your asset, whatever that might mean.

You can use this to make game layouts, for example.

```sol
-- my_room.sol
using game.models.(Room, ItemStack, Npc)
using game.items.iron_sword

-- To avoid indentation, you can turn your whole file
-- into the impl of some model.
...impl Room

name = "Room 3"

def spawn : PlayerSpawn
  position = vec2(10, 12)
end

def chest_1 : Chest
  position = vec2(0, 0)
  item = ItemStack(count: 1, kind: iron_sword)
end

def "Martin" : Npc
  position = vec3(10, 16)
end
```

Inside a `def` block, you have a new asset of that type. If allowed by the def block's type (it's also a layout), you can put `def` blocks inside `def` blocks.

You can use them to make complex compositions.

```sol
using game.ui.(UIElement, VListContainer)
...impl UIElement

-- Here's an example with game UI.
def "Title Screen" : VListContainer
  def : Label
    label = "Game"
  end
  def : Button
    action on_press
      go_to_scene(...)
    end
  end
end
```

For you to be able to do `def`s, the model you're implementing must have a field named `@children`.

```sol
model UIElement
  field @children : UIElement
  -- (other fields hidden)
end
```
```sol
model Room
  field @children : RoomElement
  -- (other fields hidden)
end
```

Since the fields are typed, you wouldn't be able to `def` a `RoomElement` inside a `UIElement` or vice-versa.

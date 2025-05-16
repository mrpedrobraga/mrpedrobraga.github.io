# Sol

Sol is an implementation-agnostic markup language for creating arbitrary game data. Think items, enemies, rooms, etc.

By defining your assets in Sol, you can use them in any language or engine of your choice.

## The format

A `.sol` file is a module. Inside a module, you can declare _fields_ of many types. That's it.

```lua
name = "Jayce"
age = 20
favourite_color = "blue"
```

The precise types of fields you can create are:
- `Text`;
- `Nat`;
- `Float`;
- `Truth`;
- Tuples `[A, B]` where `A` and `B` are types;
- `List(T)`;
- `Set(T)`;
- `Map(TKey, TVal)`;
- `Table(T)`, which can be built from a folder of homogeneous items (think `weapons/*.sol`);
- `Fn`, `Action` and `Scene` are procedure types;
- User defined types like `model`s, `struct`s and `either`s.
- `Type` (as in a field can be a type definition other fields can refer to);
- Types from extensions such as `ImagePNG`, `AudioOgg`, etc;

## What kind of data can you create?

Standalone assets;

```lua
name = "Jayce"
age = 20
favourite_color = "blue"
```

Model definitions;

```lua
model Item
  field display_name : Text
  field unbreakable : Truth = no
  field icon : Image
  if not unbreakable then -- dependent types!
    field durability : Nat
  end
  on_use : Action(Character)
end
```

Type-safe assets;

```lua
@model ItemKind

-- Dependencies on other modules!
using Icons

-- Static (Constant) fields
display_name = "Iron Sword"
icon = Icons.iron_sword
durabilty = 4
-- ^ No field 'durabilty' in the model. Did you mean 'durability'?

-- Fields can be procedures!
action on_use(target: Character)
    target.try_equip_weapon(weapon: self)
end
```

Scenes & Procedures, inside which you can call procedures, functions, dialogue...

```lua
use Items.(IronSword)

scene get_sword
    [Jude smiling]
    - Hello, {PLAYER_NAME}!
    - It is dangerous to go alone... take this!

    Inventory.acquire_item(kind: IronSword, amount: 1)
end
```

Level layouts... which can _themselves_ follow models.

```lua
model Room
    display_name: Text
    @container objects: impl Object 
end
```
```
@model Room

use game.(Marker, Chest)

display_name = "My room!"

def player_spawn as Marker
  position = [-10, -3]
  facing = .DOWN
end

def chest1 as Chest
  position = [0, 0]
end
```

And whatever more you'd like...

```lua
use UI.(HBoxContainer, TextureRect, Label)

-- User Interfaces?
def hbox1 as HBoxContainer
    def t1 as TextureRect
        texture = ...
    end
    def l1 as Label
        text = "My Label"
    end
end
```

My favourite use case is to define the save data (flags) of my game in a type safe way.

```lua
-- Save Data definition...
model SaveData
    story_point: either
        ActOne: either
            Tutorial
            FirstArea
            SecondArea
            Miniboss: struct
                fail_count: Nat
            end
            AfterMiniboss
        end
        ActTwo: either
            BeforeDungeon: struct
                keys: Nat
                found_map: Truth = no
                found_compass: Truth = no
            end
            BossBattle
            AfterBoss
        end
        ActThree;
    end
end

-- Then a save file looks like this:
-- I love it because it's readable yet it doesn't spoil what comes after...
def s as SaveData
    story_point: ActTwo.BeforeDungeon { keys: 3 }
end
```
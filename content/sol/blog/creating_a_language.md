## A tale of assets

> It's a miracle that any game gets finished.

This is a typical saying in the video game development community. It refers to the fact that video games look silly and simple to make, but have **so much** behind them, especially in tech. It's a funny thing to consider that every game we play is held together by duck tape and spaghetti code. It's funny... until you start making your own games.

Some people make a point of how some popular games we love, under the hood, are little chaotic messes themselves. So you don't really need to care that much.

<img alt="Survivorship Bias Plane" class="self-center" width="200px" src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/b2/Survivorship-bias.svg/1280px-Survivorship-bias.svg.png" />

But for every shipped game out there, there are dozens of projects that won't see the light of day. You could board at least three planes-with-red-dots-on-them with the broken dreams of starting developers who, after a month of coding, no longer understand what they themselves wrote and can not make any new mechanics. And it's not their fault: the discipline of game programming is very new, and there's so little resources in technical project management for games.

The quality of the tech behind a game isn't just an aesthetic measurement. Good tech and good tools can make implementing a feature (and making games) change from being "impossible" to being only "very difficult."

And I've been delighted with the efforts around the community to make game development easier, more affordable and more rewarding for your efforts. Projects like LDTK, Godot Engine, Aseprite have greatly helped _me_ go further than I could've gone (and I'm a full time software developer as my day job).

I did identify, however, a big area of game dev that does not yet have _great_ tools for. So, as my contribution to the awesome open-source community that raised me I started working on "Sol."

Here's the tale of its creation.

### Once upon a time...

In the 2016 I started working on the game [Inner Voices](https://mrpedrobraga.com/inner_voices). The premise of the game is pretty good: "an RPG about the difference amongst people's minds." You explore the minds of charmingly written chararcters, solving puzzles and fighting battles, and saving the day.

<p>
<img src="../../public/images/invo_vs_chaos_knight.png" width="480px" />
</p>

Back then, when I started making RPGs, I used RPG Maker Vx Ace, which was **so cool.** I could just place tiles, and make my dialogues in a cute GUI. I had no idea how people could make games "from scratch." Talking to the screen directly? Reading keyboard inputs? Writing C? No thank you.

It was heaven -- I could focus on the (already difficult) task of figuring out my story, the character arcs, story points, and gameplay balancing.

But RPG Maker is a closed piece of software. It does what it was made to do and as soon as you want to do something its creators didn't already think of, you're out of luck. I could have never implemented the battle system (as pictured in the image) in RPGMaker with my skillset of then.

That's why eventually I moved out of RPG Maker, and into game engines to make my games.

### Moving out

As a piece of software, a game like Inner Voices is not that complicated. "Move this sprite across the screen" some and "click on these menus" some more. Getting this part done is pretty straightforward.

The **vast majority** of the game's work, actually, is **"content."** Writing quality dialogues, balancing attacks, writing patterns to dodge in the battle system, drawing sprites (sigh... so... many... sprites...).

![Claire, acting](../../public/images/claire_psychologist-export.png)

For some of the content has pretty much "canon" tools in the community. Want to make pixelart? Aseprite, Photoshop, Pixelorama, are right there for you. Tilemaps? Tiled, LDTK, etc. Lots of engines have plugins to support them as if they were native. Aseprite and Godot are part of my personal toolset, and they are pretty hackable (I made some [addons for them](https://mrpedrobraga.itch.io/aseprite-qw) to aid in my process).

But what if you want to make something like an RPG that's heavy on _custom data_: dialogues, rooms, attacks, items, equipment, profiles; with formats of your own, you're out of luck.

### JSON

Most engines nowadays support JSON parsing, so a lot of developers use it to write their custom assets. JSON is rather straightfoward to learn... until it isn't.

```json
// Iron Sword for beating bad guys.
{
  "name": "Iron Sword",
  "type": "weapon",
  "durability": "10",
  "effect": {
    "damage": 5,
  }
}
```

This is how one could define an item. This example, however, is not valid JSON. Can you find the mistakes?

- Line `0` has a comment, which is not allowed in JSON;
- Line `4` has `durability` set to `"10"`, which is not a number, it's the characters "10", text;
- Line `5` has a "trailing comma," a comma not followed by anything. This can happen if you deleted something under this line and forgot to remove it, but it's not allowed by the JSON spec;
- Line `2` has the invalid content `"weapon"`. What? You remember creating it on your parser? Nope, you wrote `"Weapon"`. Good luck catching that while print-debugging.

Furthermore, if you want to change something or add a new property to this item, what properties will it accept? There's no way to know without looking at the code you wrote to parse this JSON. It's also not trivial to crate a good parser for yourself. You might end with a lot of errors caused by something being `null` or `undefined`.

I quickly started missing the simplicty of using RPG Maker. I can barely remember the time where I would try changing a property of an item, or a delay in a cutscene and it _just worked_.

Before, I could have an idea at school, come back home, and iterate over dozens of variations of it. Now, I found myself backing away from doing things that were "too ambitious."

Say you want to make a cutscene system in JSON, how would *you* go about it? Something like this, perhaps:

```json
[
  { "action": "move", "who": "Hero", "to": [0, 1] },
  { "action": "move", "who": "Hero", "to": [0, 2] },
  { "action": "dialogue", "who": "Hero", "content": "Hello there!" },
]
```

This is _awful_ to write and maintain. If you have a team where the writer is not a programmer, they will probably not understand this at a glance (or even after several glances), and, GOD DAMN IT, I left a trailing comma again.

> Admitedly this is, yes, a "skill issue." You can *just become better at writing JSON* and I did! But mom, I don't wanna go to school I wanna be a cowboy!!!

Listen.

Organization, composability, expressiveness, are well discussed topics in the world of non-game software development. We know that your code and tech is the foundation upon which your product is built. We have techniques and talks about creating codebases that can be maintained for not months, but _decades_.

So, why are games lagging behind? I don't want to fight against commas and brackets, I just want to sit and make my game, man... that's already difficulty enough.

### Go Godot!

I moved into using Godot's [Resources](https://docs.godotengine.org/en/stable/tutorials/scripting/resources.html) as my definitions of assets. `Resource` is INCREDIBLE. I can define a `class` with a model for assets of some type and BOOM! I get type safe assets that I can even edit in the Godot editor.

People in the community _love_ Resources (they are still SEVERELY underutilized).

<iframe class="yt-video" src="https://www.youtube.com/embed/vzRZjM9MTGw?si=C8EseoXNULbzd6NI" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

`Resource` is a class you can `extend`, and for the fields you define, you can create a `.res` instance of that class. So, you create an `Item` class, and can define several `my_item.res` that implement `Item`.

> This is similar to Unity's [ScriptableObject](https://docs.unity3d.com/Manual/class-ScriptableObject.html), yes, but I'm a Godot user so yeah.

Let's take a look at `.tres`, Godot's text resource format (paths were redacted).

```ini
[gd_resource type="Resource" script_class="InnerVoicesAttack" load_steps=8 format=3 uid="uid://e0wgeu842dhq"]

[ext_resource type="Script" path="res://packs/.../InnerVoicesAttack.gd" id="1_6qt24"]
[ext_resource type="Script" path="res://packs/.../atk_music_fireworks.gd" id="1_ah170"]
[ext_resource type="Script" path="res://packs/.../InnerVoicesDBSAttackChallenge.gd" id="2_4a637"]
[ext_resource type="Script" path="res://_engine/scripts/.../CharacterStatsAlteration.gd" id="4_ivihd"]
[ext_resource type="Resource" uid="uid://w36e0qfjpgvr" path="res://packs/.../char_claire.tres" id="4_yxide"]

[sub_resource type="Resource" id="Resource_beis3"]
script = ExtResource("2_4a637")
pattern = ExtResource("1_ah170")
top_left = Vector2(-2, -2)
bottom_right = Vector2(2, 2)
animation = 1
initial_position = Vector2(-20, 0)
```

Yikes. But I'll take it. It's clearly not made to be authored by text (although you can still edit it if you _need_ to I guess). But something about how it is type safe... and can refer to other resources (`ext_resource`), and can use constructors like `Vector2` make it so good to _use_ at least.

After discovering this workflow I started everything to do everything super integrated with Godot. I started writing my cutscenes in Godot's scripting language: GDScript.

```gd
move("Hero", Vector2(0, 1))
move("Hero", Vector2(0, 2))
dialogue("Hero", "Hello there!")
```

This gives me (somewhat) type safety and better error messages. I have full access to my game state and GDScript's programming capabilities.

```gd
if something:
    do_something_in_response()
```

The problem is, of course, it's a programming language, with its own technical looking syntax. It's also _not built_ for the use case of making cutscenes and has many, many unpleasant quirks when you try to use it like this.

There has not been a shortage of [issues](https://github.com/godotengine/godot-proposals/issues/56) and [proposals](https://github.com/godotengine/godot-proposals/issues/6307) for Godot about the topic (you can find me on them hehehe) but there's been little focus on it. The first issue I linked is number "56."

**This is NOT a fault of the Godot team:** they have _a lot_ in their hands making a game engine. It's natural for them to focus on more technical matters of game development. The features proposed are non-trivial to implement and don't necessarily mesh well with GDScript and Godot's internals.

I just with I had a language built for the purpose of making assets and cutscenes... Something that is its _own thing_, its own project detached from a game engine, but still easy to integrate.

### Introducing... not Sol!

My solution for that at the time was to create a language called "Starscript." Call it Sol's predecessor.

![Simple dialogues in Starscript](../../public/images/starscript-2.webp)

It wasn't perfect, but was very straightforward. It's trivial to write new dialogue and it made my productivity skyrocket. Really.

It has a dirty implementation in GDScript, but it does hooks into Godot's "Resource" system so that every time you save a Starscript file, Godot automatically recompiles it.

<iframe width="640px" height="360px" class="yt-video" src="https://www.youtube.com/embed/V8Mtf1jZG9I?si=QIbFv_f2Az8u-mJ5" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

Within the game editor, as you can see on `00:30`, you can drag your starscript into the correct location and select the scene from a dropdown. No more "key not found" errors.

You're probably expecting a "but", but nah, this _was_ great. I could finally see myself finishing that game, all because I built a strong foundation for myself to work on top of. In fact, I often sketched new scenes _in starscript_ before even making them in game, since it was so easy. Writing cutscenes became joyous and fun again! And also fast!

![](../../public/images/erica-spin.gif)

It even had basic programming capabilities, despite its clunky minecraft-commands-inspired syntax.

![if else expression](../../public/images/conditionals.webp)

It did not solve _every_ problem, though. While I could write cutscenes easily, other custom assets, the ones using `Resource` had some limitations:

- **Bulk editing of resources:**
There's no straightforward way of editing several resources in bulk (although there are [addons](https://godotengine.org/asset-library/asset/1479)).

- **Resources with both data AND scripting:**
I have attacks that have data (the display name, icon, etc) and also run a script in GDScript to decide their effect. For that I have to create a `.tres` and a separate `.gd` file.

> Although I did _try_ to make scripting capabilities with Starscript, it never looked good.
> ![More starscript code, this time showing how functions work.](../../public/images/starscript-functions.webp)

- **Type safe scenes:** There's no way to create [scenes that feel like builtin nodes](https://github.com/godotengine/godot-proposals/issues/1935), or that [only accept certain kinds of children](https://github.com/godotengine/godot-proposals/issues/5475), without a lot of annoying configuration.

You also lock yourself into using only your game engine. If you develop a game in Godot and want to migrate to Unity (or the contrary, most likely) you'd have to migrate all those assets by hand. This is also true if you'd like to migrate to a new version of Godot.

I migrated Inner Voices from 3.x to 4.x because I'm a madman, but plenty of my game dev friends are stuck on the 3.x experience. Recently I've been flirting with Bevy, since my most productive language is Rust, but I'm not migrating a whole game again.

### Sister tales

[Mother Encore](https://motherencore.com/) is a reimagining of Mother with reworked graphics, story, game design, etc. They are a team of very passionate developers powering through the esoteric world of game programming.

I'm tangentially affiliated with the team (they're one of the "game dev friends" I mentioned) by occasionally supplying development tools that integrate with Godot but are distinct from it.

For example, recently I made a tool for editing their battle background shader without touching Godot editor or GDShader:

<iframe class="yt-video" src="https://www.youtube.com/embed/0OEWLVnX30A?si=JNiu9w8y28R27cw7" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

There's was a great demand by them (and other indie game dev teams) for tools like these because opening the entire game in an editor only to edit something consumes a lot of processing, and people who want to make battle backgrounds might not know how to navigate a game engine editor.

At some point I was also developing this beaut, a scene editor for [Modot](https://benichi.neocities.org/MODOT), the framework that Mother Encore uses on top of Godot.

![Garden, a gorgeous tile editor](../../public/images/screenshot_garden.png)

It didn't get completed but was a joy to use. It was a hell of a project to approach back then. There was also the most recent unfinished project, a cutscene editor...

![Dialoginator](../../public/images/dialoginator.webp)

...which was the project that really showed me there was a demand for good tooling for assets and dialogues.

I set out to put an end to the madness for good, and create a solution that game devs everywhere can use. The best tooling, so that we can _just focus on making our games_.

> Before I show what _I_ came up with, here are some honourable mentions.
>
> - [Dialogic](https://github.com/dialogic-godot/dialogic?tab=readme-ov-file), by Emilio Coppola and Jowan-Spooner, has made all the waves in the Godot community (has a visual editor, too, if you're into that). If I didn't already have Starscript by the time of its creation, I'd probably be using it!
> - [Dialog Manager](https://github.com/nathanhoad/godot_dialogue_manager), by Nathan Hoad, which I only did not use because it happens to be incompatible with my needs, but is pretty great (syntax is pythonic, too if you're into that).
> - I mean, just all of [these](https://godotengine.org/asset-library/asset?filter=dialog&category=&godot_version=&cost=&sort=updated) I guess, hahaha.

## Here comes the sun!

The long cold lonely winter is gone.

I decided to put my knowledge of programming language compilation to good use and solve this problem generally. You see, there's been a great number of attempts to create open formats for assets in games, but they are designed for rather specific use cases. I wish to solve the problem of custom data in a way where it can fit **any** use case.

By making it engine-agnostic, too, it brings us closer to a world where you can specialize in "game development" instead of "Godot" or "Unity". If you wrote your assets in `.sol`, it's effortless to migrate them to another or a new engine.

> Does that even make sense as a goal? Is it possible at all to achieve?

Surprisingly, it is. Any game you make is ultimately written in a computer in a programming language (like C, Rust, etc). So, a programming language necessarily has sufficient capability to represent any asset we want. The challenge is, then, to create a programming language with the right syntax and quirks for the job.

*\[drum rolls\]* 🥁

Sol is a markup/programming language for assets. It has a toolchain you can run from the command line called `sol`.

You can create a new sol project on a folder with this command:

```bash
$ sol init
```

Which creates a minimal `sol.toml` file. This is what makes a directory count as a sol directory.

Now, in this directory, we can create our first asset. Let's name it `IronSword.sol`.

```sol
-- Iron Sword for beating bad guys.
name = "Iron Sword"
type = "weapon"
durability = 10
effect_damage = 5
```

Look at it. So small, so simple, so not JSON.

Right now, though, this file has some of the same problems as JSON (no guarantees of structure). But in some other file (`item.sol`) you might write something like this:

```sol
model Item
  field name: Text
  field type: ItemType
  field durability: nat
  field effect: ItemEffect
end

enum ItemType
  FOOD
  WEAPON
  ARMOR
end

model ItemEffect
  field damage: nat
end
```

And now, you can use this in your Iron Sword to guarantee that it is a proper `Item`.

```sol
...impl item.Item

name = "Iron Sword"
type = "weapon"
durability = 10
effect_damage = 5
```

And you're done. Let's run `sol build` and see what happens.

```sol
-- Could not compile.

...impl item.Item

name = "Iron Sword"
type = "weapon"
--      ^ Expected value of type `ItemType`, not `Text`.
durability = 10
effect_damage = 5
-- ^ No field named 'effect_damage'.
--   hint: Did you mean `effect`?
```

Oh yeah we forgot to do the changes to the new models. Oops. But, yeah, you can see how `sol` will, ahead of time, not let us forget the contracts we create. Here we made a small mistake, but these error reports can eliminate _so many hours_ of debugging -- especially if you use a code editor like VSCode or Zed (the errors will appear as red squiggles before you even hit `build`).

```sol
using item.(ItemType, ItemEffect)
...impl item.ItemType

name = "Iron Sword"
type = ItemType.WEAPON
durability = 10
effect_damage = ItemEffect (damage: 5)
```

Awesome.

Okay, let's add a new field for determining what happens when an item is used.

```sol
using Character

model Item
  field name: Text
  field type: ItemType
  field durability: nat
  field effect: ItemEffect
  --     vvvv
  field on_use: Action(user: Character, target: Character)
  --     ^^^^
end
```

It is an `Action`, which can contain arbitrary code. It takes a single parameter of type `Character` (another model in our workspace, let's say).

```sol
-- (Hidden usings)
...impl item.Item

name = "Iron Sword"
type = ItemType.WEAPON
durability = 10
effect_damage = ItemEffect (damage: 5)

-- And here we provide the action
action on_use(user: Character, target: Character)
  * {user.name} slashed with force.
  target.stats.damage(10 HP)
  * {target.name} took 10 damage!
end
```

Now let's add icons. Surely we couldn't create an image using `.sol`, right? Well, yeah, we can't -- but we can import it into our asset.

```sol
using Character
using Texture2D

model Item
  field name: Text
  field icon: Texture2D
  field type: ItemType
  field durability: nat
  field effect: ItemEffect
  field on_use: Action(user: Character, target: Character)
end
```

And now we have all we need to have items in our inventory! There are a lot of details I glossed over, but it does _feel_ simple, right?

![Inner Voices inventory, with lots of items inside!](../../public/images/invo-items.webp)

Here's how a dialogue section looks like.

```sol
-- They look like screenplays!

def "Explaining Dialogues"
  * Pedro gets close to the camera.

  [Pedro]
  - Hello, there!
  - This is how dialogues are like.

  [Claire]
  - You can change the speaker with the square brackets.
  - They can also have these speaker tag thingies...
  - You can use them for expressions, right?

  [Pedro smiling]
  - That's true!
  - It's so cool.
  [& sad]
  - You can change only the expression and not the speaker.

  [Claire surprised]
  - Oh... nice.

  wait(for: 1 second)

  [Jeff]
  - hi guys

  [Claire]
  - Shut up Jeff.
end
```

It's somehow even cleaner than Starscript.

> Here is a video of me rendering "expressions" in my dialogue system.
>
> <iframe class="yt-video" src="https://www.youtube.com/embed/91vPdVKfkx4?si=UwMpS8UMUG4anZh4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

Dialogue and code are seamless so you can do checks, loops, etc, to call dialogue.

```sol
if finished then
  - Congratulations! You did it!
else
  - Go do the thing.
end
```

There's a syntax for "putting things inside others" that you can use for making rooms.

```sol
...impl Room

name = "Hero's Room"
spawn_marker = player_spawn

def chest_1 : Chest
  position = vec2(3, 4)
  item = IronSword
end

def player_spawn : Marker
  position = vec2(3, 7)
  direction = UP
end
```

This is similar to Godot's `.tscn`, except that the room here implements `Room`, a model, which can restrict which children can be put in it.

There's so much more that sol brings to the table, and not a lot of time to mention everything in this article, so here are some honorable mentions.

- **Game Engine integration:** You can supply Sol with functions it can call to do things on your game with `extern fn`s. That's how Sol can modify your game's world.
- **Non .sol assets:** With extensions, Sol should be able to `using` assets that are not themselves `.sol` files. Think setting an `Item`'s icon as a PNG image.
- **Todo:** There's a syntax for todo comments (`--todo`) which will emit warnings when you call `sol build`. With it, you can partially sketch assets, and Sol will remind you to finish them.
- **Dependencies:** Install assets with `sol add <asset>` for installing models, assets, etc as a dependency. This might be assets like images, weapons, etc, but also models! So you could install a pack with an "Item" model pre-made and only create instances of it.
- **Migrations:** With `sol migration` you should be able to specify how different versions of assets get upgraded into new versions as you release patches to your game. If a `Character` model previously had `name` and `surname`, but will now only have `full_name` you can write a function that explains and applies the change across your entire workspace (and the migration gets saved in case a player with an older save file boots a new version of your game).

### Only text?

Text is incredibly versatile, and text editors are _super_ light. That's why I chose to start by thinking about text assets.

Sol _can_ build assets into binary files, so no worries about shipping gigabytes of text files. I'm also studying how SQL databases are used in video games to ensure Sol is compatible too.

But, more deeply, Sol, as a programming language, is not necessarily idea of `.sol` files. Its model of data and behaviour can serve as a "protocol" for other tools and engines. It has a public Rust library (`named sol-lang`) which can do anything you want with `.sol` files.

### Sol as a protocol

The same way that Godot has a builtin editor for `.tres`, it's possible to have GUI editors for sol, using it as a "language" for talking about assets, with the extra benefit that it's not tied to a specific engine.

I want to try my hand at reviving Dialoginator in the future, with the same hackability and extensibility as Sol. Also with a new name.

But the best part of Sol being an open source "protocol" is that _I_ don't necessarily have to do it. Community members can build libraries for sol for math, geometry, sprites, etc, and also tools that save into `.sol`. And as soon as they'd target `.sol`, it means their asset can be used in any engine that also speaks `.sol`.

Futurely, you might see tile map editors that edit tiles and save map metadata as `.sol`, or music editors that save BGM and loop data as `.sol`... What an exciting idea!

### Integration

One of arguments for using tools built into engines (like `.tres` or `GDScript`) is that they are very tightly integrated into the engine. But there's no need why Sol can't also be integrated.

My first two integration targets will be `Godot` and `Bevy`, because I use them and they speak Rust. `Bevy` has always been a "bring your own stuff" kind of engine. `Godot` is incredibly welcoming to external tech in its design.

As an example, I want Sol assets to be editable in the editor and to be able to use Godot only features like `@export`, `@tool`, `signal`s, etc. So, I'm making sure Sol has directives and annotations that extensions can use. Then, I can create a `Godot` library with Godot-specific functionality. After that, with GDExtension, I can generate the bindings and make an addon on the asset library.

I want to make Sol feel right at home with Godot and Bevy, while having the least amount of setup possible.

And if Sol catches on, engines might start trying to be compatible with Sol on purpose, the same way they do with PNG, OGG, OBJ, GLSL, glTF...

### Community

There's a lot to be play-tested and battle-tested about Sol before I can tell you to start making games with it. I want to make sure it is a fun language to use (or at least a boring language, but just not an insane language).

I want to make Sol simple to learn, which means making it have THE LEAST number of features I can. So I need to make them _good_, _versatile_ features.

I'm specially proud of the `def` syntax for "putting things inside each other" as I said. Because you don't necessarily have to use it for making game scenes! You could also make UI, or animations, or behaviour!

```sol
...impl UIElement

def : HListContainer
  def label : Label
    text = "..."
  end

  def button : Button
    text = "Click me!"
    action on_click
      label.text = "Thanks for clicking me"
    end
  end
end
```

I'm very excited to listen to the use cases of everyone and to develop a strong presence in the game development ecosystem. If you are a game developer and think you'd be interested in using Sol (if it had been released when you started your game) you can [DM me at Twitter](https://twitter.com/messages/compose?recipient_id=&ref_src=twsrc%5Etfw) or send me an email at `mrhenribraga@gmail.com`.

I ran out of things to say.
Until next week!

Pedro Braga.

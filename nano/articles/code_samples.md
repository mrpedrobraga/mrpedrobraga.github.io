## Theoretical Code Samples

nano is designed to keep looking clean even when you use it for very specialized purposes -- like a faux DSL.

### REST API

Consider the following controller (NestJS).

```ts
import { Controller, Get } from "@nestjs/common";
import { UserService } from "./user.service.ts";

@Controller("user")
class MyController {
  constructor(private userService: UserService) {}

  @Get("")
  async getAllUsers() {
    return this.userService.getAllUsers();
  }

  @Get(":id")
  async getOneUser(@Param("id") id: string) {
    return this.userService.getUser(id);
  }
}
```

nano's support for annotations (AST-level decorators), resource files (almost like singletons) allows us to have a theoretical REST API library like this:

```nano
from 'REST' import *
import './user.service.nn' as userService

#%Controller

## Returns all users.
%%ApiEndpoint (GET, "user") -> (
	userService.get_all_users()
)

## Returns one user given its [param id].
## @param id The id of the user.
%%ApiEndpoint (GET, "user/(id)") -> (
	userService.user(id: params.id)
)
```

Remember that nano is compiled and strong typed -- the above example is fully typed from inference alone, and you should be able to see the types if you're using an LSP. (I do recommend you to explicitly type your things wherever you can).

Furthermore, nano is capable of deep reflection, so the parameters, types and even the comments from the routes can be served to a documentation.

## Foreign interfaces

Now consider this connection to a database:

```nano
from '../database.nn' import database

fn get_all_users -> (
	database.query %%sql SELECT * FROM user u WHERE u.deletedAt IS NOT NULL
)
```

nano's annotations allow embedding temporary grammars by borrowing as long as it uses nano-compatible tokens. nano serves as a host to the embedded grammar, feeding it symbols and types. The result must be also fully typed.

nano can also import libraries over a network. Someone who's serving an API on nano will be able to share a strong-typed interface into their code...

```nano
## entry.nano

alias MyApi = 'https://weather.api'

## other_file.nano

import MyApi

let weather = await MyApi.current_weather()
#     ^ TYPE?    MyApi::GlobalWeatherInfo
```

This isn't impossible with other programming languages, but it is effortless with nano.

### Godot Engine Class

Consider this GDScript snippet I took from the Star Engine Core repository.

```gdscript
@tool
extends TextureRect
class_name SubViewportRect

## Renders all the children through a [SubViewport].

func _ready():
	clip_contents = true
	if Engine.is_editor_hint():
		return
	setup()

func setup():
	var children := get_children().duplicate()
	var viewport := SubViewport.new()
	viewport.canvas_item_default_texture_filter = Viewport.DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS
	viewport.transparent_bg = true
	viewport.size = size
	add_child(viewport, false, Node.INTERNAL_MODE_BACK)
	for child in children:
		remove_child(child)
		viewport.add_child(child)
	texture = viewport.get_texture()

func _draw():
	if Engine.is_editor_hint():
		draw_rect(Rect2(Vector2(), get_size()), Color.GOLDENROD, false, -1.0)
```

Here's how it would look in nano:

```nano
from 'godot' import *

#%tool
with self as class SubViewportRect : TextureRect

## Renders all the children through a [SubViewport].

fn _ready -> (
	clip_contents = true
	if Engine.is_editor_hint() then return

	setup()
)

fn setup() -> (
	let children = get_children() as array
	let viewport = SubViewport.new()

	viewport.canvas_item_default_texture_filter = DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS
	viewport.transparent_bg = true
	viewport.size = size
	add_child(viewport, false, INTERNAL_MODE_BACK)

	for child in children do (
		remove_child(child)
		viewport::add_child(child)
	)

	texture = viewport::get_texture()
)

fn _draw -> (
	if Engine.is_editor_hint() then (
		draw_rect (
			Rect2 (fvec2(), get_size(), GOLDENROD, false, -1.0)
		)
	)
)
```

The nano code is a little bit taller formatted, but after the `from godot import *` declaration, everything feels right at home!

Whatever syntax an engine has that nano 100% doesn't allow for can be made up with annotations.

```gdscript
@export var: float = 1.0
@onready var button = $Control/Button
```

```nano
%%export var: float = 1.0
%%onready var button = %%node("Control/Button")
```

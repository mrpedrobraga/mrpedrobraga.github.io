---
date: 0001-01-01
tags:
  - graphics
  - ui-composer
title: An Interactive Collage
---
In the [[article-01|previous article]] I introduced the reason why I started working on UI Composer. Today, I am excited to bring you nerds more information about how exactly it works.

Let us start with "what is a Graphical User Interface."

> I don't know, Pedro, what _is_ a graphical user interface?

Oh, hey, Reader!

A user interface is something that allows the user to interact with a running program. It serves two purposes: allow the user to change the program's state and allow the user to read the program's state;

A _graphical_ user interface allows the user to interface with the program, through graphical elements that are rendered to the screen. Like this one:

<button onclick="alert('Howdy partner!')">Clickety click me!</button>

Let us see how we would make a component like this from scratch!

## The Interactive

Here is the button without its visuals but with its interactivity.

<button onclick="alert('Howdy partner!')" style="opacity:0;">Clickety click me!</button>

Really, the button _is_ there. Try clicking on it!

If you were to implement this behaviour, you can imagine these steps:
- Add a listener for a mouse button down event;
- Check if the click's position was within the button's bounds;
- Call the button's `onclick` callback;

Again, there _are_ more nuances than that, like the darkening of the button when you hover it, the mouse cursor, the Tab navigation, etc. But baby steps!

> No, yeah, I get it.

The Operating System will gladly notifies us of when any event happens, so we can check if it was a button event

```rust
// Dramatisation of what UI Composer does
loop {
	let rect = Rect::new(0.0, 0.0, 100.0, 20.0)

	match new_event().await {
		WindowEvent::MouseButton(position, press) => {
			if rectContainsPosition(rect, position) {
				// Do whatever you want.
			}
		}
	}
}
```

Because there are so many different kinds of events, and many more nuances regarding how to handle them, UI Composer handles them for you. Just create a new "Interaction" object with a given size and shape and give it to UI Composer.

```rust
let tap = Tap::new(rect, || alert("Howdy partner!"));

App::run (
	Window (
		tap,
	)
)
```

> That was simple.

Yeah!
## The Collage

Here are the button's visuals. A gray rectangle and text saying "Clickety click me!" overlayed on top. Here is the button with _only_ its visuals:

<button disabled>Clickety click me!</button>

If you were to render this graphic, it would be pretty easy to imagine the steps.
- Render a gray rectangle;
- Render text;

If it sounds weird to think of UI as being "just a bunch of primitives arranged together," ask your UI designer how they make their components in Figma...

> No, I get it. Although, what do you _exactly_ mean by _"render"_?

The word "render" means "to represent." Painters use the word in the context of them painting something, representing its form through the paint in the canvas.

The digital canvas of your screen, however, is a big rectangular "fabric" made of tiny little parts that can change colour individually. These picture elements (short: pixels) are what we will ultimately render all our graphics onto.

> Are you really gonna tell me what a "pixel" is?

Yes!

Pixels are easily modifiable by changing some memory sitting somewhere. They would be layed down in the one-dimensional memory left to right, then top to bottom, the same way letters flow in a paragraph of text.

<blockquote style="display:flex;flex-direction:row;gap:1em;">
<style>
	img {
		image-rendering:pixelated;
		min-width: 64px;
	}
</style>
<img src="./assets/8by8img.png"/>
	<div>[ Red, Yellow, Cyan, Green, Pink, Purple, Blue, Navy, Dark Gray, Light Gray, Darkish Light Gray, Lime, Pink, ... ]</div>
</blockquote>

Drawing something like, say, a gray rectangle, would just be a matter of addressing each pixel one by one and changing its value.

```rust
// Something like this

let screen: MemoryBuffer = ...;
let rect = Rectangle { x: 0, y: 0, width: 100, height: 20 };

for y in (rect.y..(rect.y + rect.height)) {
	for x in (rect.x..(rect.x + rect.width)) {
		image[x + y * image.width] = Color::gray();
	}
}
```

> I see... do I have to write this code every time I write a rectangle?

No, no, you can make a function to reuse.

```javascript
let screen: MemoryBuffer = ...;

fn Rectangle(screen, x, y, width, height) {
  ...
}

Rectangle(screen, 0, 0, 100, 20);
Rectangle(screen, 10, 10, 30, 30);
Rectangle(screen, 0, 40, 10, 80);
```

> Oh, yeah, that makes sense.

Now there is a problem with this approach.

Your computer's _Central Processing Unit (CPU)_ is very powerful, yes.

[This Rust code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=840c89a3b505da2d0f4915692483cdba) runs pretty takes a tenth of a second to write some colour-like data to a buffer. That is impressive. But if we want to draw an app in 60 FPS, we have short of sixteen milliseconds to not only _draw_ to the screen, but calculate the colours, layout, state, everything.

Its flaw is that it is to execute instructions one after the other. Even with multi-core CPUs, the filling work would be done _mostly_ sequentially. And that is a shame, because the work to paint one pixel does not really _depend_ on the work done for the previous pixels. It would be better if we could find the equivalent of holding a big bucket of paint and throwing it at the canvas.

Fear not, for people have already asked themselves "Well, what if we had LESS powerful computation cores but had thousands of them?"

So, while we still use CPU rendering for lots of tasks, for more beefy graphical needs we use the _Graphics Processing Unit (GPU)._

GPUs have the capability to interact with each pixel in parallel*, cutting rendering time by orders of magnitude. Instead of writing the pixel filling code ourselves, we talk to the GPU through a graphics library (_OpenGL_, _Vulkan_, _DirectX_, _Metal_) and tell it to do that work for us.

And it looks _kind of_ like this:

```rust
let shader = "#version 300 void main() {drawMyStuffPleaseThanks();}";
let rect = Rectangle = { x: 0, y: 0, width: 100, height: 20 };
let params = Params { color: Color::gray() };
let screen: MemoryBuffer = ...;

let geometry_gpu: GPUMemoryBuffer = ...;
let screen_gpu: GPUMemoryBuffer = ...;

vkMakePipeline(rect, shader);
...
vkCopyBuffer(rect, geometry_gpu);
vkYouWillRenderThisGeometry(geometry_gpu);
vkYouWillRenderItWithTheseParameters(params);
vkRenderMyPrimitives();
vkCopyBuffer(screen_gpu, screen);
...
```

> Um, Okay, What? What is this?

Yeah, okay, this might look a slightly more verbose, but it is just some pseudo-code. I can assure you that, in practice, it is much, much, **much** more verbose.

> And all this work makes it faster, somehow?

Yes! Because the GPU's hardware is built for a very specific purpose, you can not do _whatever_ you want. Graphic libraries like _Vulkan_ try to give you as much control as they can by letting you change state, rendering modes, allocate buffers, configuring the rendering pipeline to your need.

The cost of all that freedom is having to be explicit about every step of the way. But the reward is that now things that would take seconds take milliseconds.

Also, a lot of it only needs to be done once. For example, once you have set up a GPU program, allocated the memory buffers, etc, you can reuse a lot of that work for the next frame. One of my favourite things is _Instanced Drawing_, which allows you to specify a geometry once and draw it like, a million by just saying:

```rust
vkCmdDrawIndexed(like, a million times)
```

> Really? Interesting...

Yeah! With this, you can draw not just rectangles, but _entire worlds_ interactively. All starting from the primitive idea of specifying geometries and filling them.

<iframe width="640" height="360" frameborder="0" src="https://www.shadertoy.com/embed/McV3R3?gui=true&t=10&paused=true&muted=false" allowfullscreen></iframe>

The reason why I explained all of this, is to give you a sense of from where comes the incredible computational cost of rendering UI...

...or, more specifically, the lack thereof.

> I was gonna say! It seems to me like GPUs would kick ass at rendering UI?

Language. But yes it really makes you wonder how modern apps, with their hipster clean minimalist UIs still stutter and lag so much.

The answer is _bloat_. _Overhead_. Whether you are using a browser or a game engine to build apps with, you eventually you run into it.

> Why?

Because they do _way_ too much more than necessary for any given app. That makes sense, right? A game engine is built for the average game. A web browser's purpose is to be able to render any page it could encounter in the web, it needs to have code that works well with every scenario.

This all shows how even though there is Big Money given to engineers to make the web fast, its still _so_ much performant to roll out your own graphics that _do less_; are perfectly tuned to achieve your specific wants an not an inch more.

> Still, I don't want to write all that code just to render a few elements.

That is where UI Composer comes in.

Here is how you would specify you want a rectangle to be drawn in UI Composer.

```rust
Quad::new(Rect::new(0.0, 0.0, 100.0, 20.0), Rgb::gray())
```

> Really? That's it?

Yes.

Then, you can create components, by, aggregating primitives.

```rust
fn Button (text, action) -> LayoutItem (
	Resizable::new(|container_hints| {
		Quad::new(container_hints.rect, Rgb::gray()),
		Text(container_hints.rect, text),
		/* Interaction Code Goes Here */
	})
)

// Then, to use it:

Button("Clickety click me!", || alert("Hello there, partner!"))
```

> Oh. This does look really nice. You describe _how_ to draw your component geometrically and then the library knows how to render them.
> 
> But all this abstraction brings some overhead sooner or later, does it not?

There is such a thing called "Zero-Cost abstractions," which allow you to add language features and useful APIs that get read during compilation time, but produce the same binary that you would get from writing everything by hand.

> Huh? Really?

Yes!

Now, still in this topic of getting rid of "the bloat."

I am obviously not the first one to consider this. For example, the famous [**ImGui**](https://github.com/ocornut/imgui) prides itself in being a **bloat-free graphical user interface for C++**.

> If these bloat-free UI libraries exist then why are you making your own? Are you Rewriting It In Rust™?

No, [I have been beaten to it](https://github.com/emilk/egui). The thing is, these libraries are _simple_. They have very focused goals of allowing you to iterate fast and interact with your application (especially games) easily.

> That's good, right?

It _is_ good! But it does not satiate my hunger.

I want a library that is: Native, Light and Simple, yes...

...but is also capable of doing everything a more powerful library can do. And then do it faster.

I am talking about digital audio workstations, code editors, painting programs... so that requires a slightly different approach.

> Ah...

What makes ImGui so simple is how it manages reacting state. Or, more specifically, how it kind of does not.

If you have a button in a [**retained** mode](https://en.wikipedia.org/wiki/Retained_mode) UI, like in a browser with _HTML_, or in _GTK_ or _QT_, and you want to create elements, you need to... well, _create_ elements, and then "configure" them so they work together.

Consider the case of having a button making an element appear or disappear.

<blockquote style="display:flex; gap:1em;">
	<input id="btn" type="button" value="Show"/>
	<div id="element" style="visibility:hidden;">Hello there!</div>
	<script>
		let visible = false;
		let element = document.querySelector("#element");
		let btn = document.querySelector("#btn");
		btn.onclick= function() {
			visible = !visible;
			element.style.visibility = visible ? "visible" : "hidden";
			btn.value = visible ? "Hide" : "Show";
		};
	</script>
</blockquote>

You need to not only create the elements, but weave them together in how their states alter their visuals. 

```javascript
//Pseudo-code
let body = ...;
let visible = false;
let element = document.createElement("div");
let button = document.createElement("button");

body.appendChild(element);
body.appendChild(button);

element.style.visibility = "hidden";
button.value = "Show";

button.onClick = () => {
    visible = !visible;
	element.style.visibility = visible ? "visible" : "hidden";
	button.value = visible ? "Hide" : "Show";
}
```

> This looks complicated.

Yes, this is cumbersome, prone to error, and requires you to _create_ elements that exist in memory...

Now check out the **immediate** mode equivalent.

```rust
//Pseudo-code
static bool visible = false;

fn RenderUI() {
    if (Button(if visible { "Hide" } else { "Show" })) {
        visible = !visible;
    }

    if (visible) {
        Text("Hello there!");
    }
}

```

> Woah. This is _much_ more simple. It is also intuitive!

Yes, immediate mode rules. Also notice that you never do _create_ elements.

> No? What does `Text` do?

Well, it does _not_ create an element. There is no "object" that sits in memory can be manipulated by you and whatnot. Basically, if the `Text()` function gets called in a frame, it means a little text exists in that frame. If it is not called, it does not exist. Same for the button.

> And if the button was pressed in that frame, `Button(...)`, will return true?

Yes.

> Okay, what is the problem with this?

The biggest problem is, the screen is re-rendered every frame... kind of. There are some optimisations, some good cache, but it still not suited for huge UI loads.

So, for UI Composer, I wanted to have fine grained awareness of what can change what, when and how, so I can choose to re-render optimally.

> How do you know exactly what to re-render?

It is up to the user to determine.

> So... like the cumbersome example you showed???

Uh- um- well, yeah but, like, with a different API, but this complexity can be managed nicely through an elegant API, by using Functional Programming!!!

> Oh. Oh no, Did I misclick into a Haskell article?

Relax!

What I mean by functional programming is... you do not just call a function to add a component... you _return_ it.

```rust
//Pseudo Code

fn ShowAndHide() -> LayoutItem {
	let is_visible = false;
	// Since the component returns UI now,
	// we pass the result of clicking the button as a callback.
	let toggle_visibility = || is_visible = !is_visible 

	(
		Button(
			if is_visible { "Hide" } else { "Show" },
			handle_clicked
		),
		is_visible.then(|| Text("Hello there!"))
	)
}
```

> Okay... I understand. But even like this, would not this still require redrawing the entire screen just in case the variable changes?

Patience!
Here is how we make the UI know what reacts to what.

```rust
//Pseudo Code

fn Element() -> LayoutItem {
	let is_visible_st = Editable::new(false);
	let toggle_visiblity = || is_visible_st.set(!is_visible_st.get());

	is_visible_st.signal().map(|is_visible| (
		Button(
			if is_visible { "Hide" } else { "Show" },
			toggle_visibility
		),
		is_visible.then(|| Text("Hello there!"))
	))
}
```

> Let's see... you changed `is_visible` to `is_visible_st` which is now of type `Editable<bool>`... whatever that is.
> 
> And you wrapped the UI part inside `is_visible_st.signal().map(...)`.

Yep, `Editable`, is a monad that adds reactivity to a value.

> Monad? What? I don't... understand how this works.

That is fair.

Consider this JavaScript code involving Promises:

```javascript
function getSomeNumber(): number {
  ...
}

let num = getSomeNumber();
let string = num.toString();
```

> Okay, so you have a number then you convert it into a string.

Now suppose that this function _takes_ a while to complete.

> That would make it `async`!

```javascript
async function getSomeNumber(): Promise<number> {
  ...
}
  ```

More specifically, it will no longer return `number`, but `Promise<number>`. And now we can not call `toString()` on it to get our stringified number anymore.

> Which makes sense, yeah, because the number isn't necessarily "here" yet.

Yes. But what if we want to specify what to do when the number comes, in advance?

> We'd use `.then` with a callback that tells you what to do with the number that will arrive.

```javascript
let num_promise = getSomeNumber();
let string_promise = num_promise.then( num => num.toString() );
```

> Yeah, like that.

Then, we can pass `string_promise` forth for doing more transformations on it until eventually something `await`s it.

Now imagine this. What if this number that _will_ arrive... could arrive _more than once_?

> Oh.

That is what a _Signal_ is.

```javascript
function getSomeNumberState(): Signal<number> {
  ...
}

let num_signal = getSomeNumberSignal();
let string_signal = num_signal.map( num => num.toString() );
```

> Oh my.

Likewise, `string_state` is passed forth until eventually something `await`s it.

> But... UI...

By mapping on the `Signal<String>` there you can eventually get a `Signal<LayoutItem>`. If you give UI Composer a `Signal<LayoutItem>` it will "await" it. Every time this Signal resolves with new UI, UI Composer will re-render it.

```javascript
string_state.map( string => Text( string ) )
```

And it would re-render _only_ the UI the signal resolved with, not the whole screen.

> I see!
> 
> So this allows you to tell exactly what reacts to what, so you can have efficient updates, like retained mode... but you can write it in terms of "existence" and avoid creating objects, like immediate mode!!!

Yes.

And, by the way, since "state" is a first-class value now, we can do things like returning it from a function and have potentially thousands of UI items reacting to it each on their own, without manually specifying every interaction.

> Oh!!!

And, the implementation of Signals I use is highly efficient, and Zero-Cost most of the time. That is, [they are often equivalent of you having written all the updating and reacting code yourself](https://docs.rs/futures-signals/0.3.34/futures_signals/tutorial/index.html#signal-1).

> OH!
> 
> Wait a minute...

Epiphany time!

> I get to write beautiful, expressive, declarative code like React or Swift UI, write and share components, do animations and tie things to states, all while having millions of elements on the screen, all updating efficiently, without lagging or stuttering! This is wonderful!

:-)
---
date: 2024-12-28
tags:
  - graphics
  - ui-composer
title: An Interactive Collage
description: An overview of what UIComposer does as a GUI library.
---
To explain how we can make UI from scratch, we have to learn the basics.

Starting with "what is a user interface?"

> I don't know, Pedro, what _is_ a user interface?

Oh, hello, Reader!

A user interface is something that allows the user to interact with a running program. It serves two purposes: allow the user to change the program's state and allow the user to read the program's state;

A _graphical_ user interface allows the user to interface with the program, through graphical elements that are rendered to the screen. Like this button:

<button onclick="alert('Howdy partner!')">Clickety click me!</button>

You are able to trigger an action by clicking on this button, right?

> Yea.

WRONG! Shame on you!

You do not actually "click" on this button, you click on your mouse's button. Your mouse detects that and sends a voltage to the computer through the USB port. Your operating systems has a "driver" that understands this signal and relays it to the application!

That thing that looks like a button, if you look very closely, you will notice it is merely a gray rectangle with text written on it.

> ...

Objects are just a social construct. UI Composer embraces this angle, it's built like that _all_ the way down.
## The Interactive

Remember this button?

<button onclick="alert('Howdy partner!')">Clickety click me!</button>

> How could I forget? The clickety click button.

Now here is the same button, without its visuals but with its interactivity.

<button onclick="alert('Howdy partner!')" style="opacity:0;">Clickety click me!</button>

Try clicking on it!

> Huh! It worked!

If _you_ were to implement this behaviour from scratch, you would follow these steps:
- Register a listener for events the Operating System receives;
- Check if the event was a mouse button click;
- Check if the click's position is geometrically within the button's bounds;
- If it is, perform the action you want to be performed;

Here's some dramatised code.

```typescript
// Dramatisation of event handling.
const buttonArea = new Rect(0.0, 0.0, 100.0, 20.0);

while true {
	const nextEvent = await OS.getNextEvent();

	if (nextEvent.type instanceof MouseButtonEvent && nextEvent.isPressed) {
		if (buttonArea.includesPoint(nextEvent.position)) {
			alert('Howdy Partner!');
		}
	}
}
```

> I understand. But do I have to do this for every button I need clicked?

In practice, no, because I am doing it for you 😛. UI Composer handles the event pooling and whatnot for you. There's so many ifs and buts and nuances, so it only makes sense, right?

As the user of the library, you can compose "Interaction Primitives." For example, to achieve the behaviour of a button, you only need to write this:

```typescript
// UI Composer is a Rust library
// but I will be using Typescript for educational purposes.

const buttonArea = new Rect(0.0, 0.0, 100.0, 20.0);
const tap = new Tap (buttonArea, () => alert('Howdy partner!'));

App.run (
	tap
);
```

And you get a new window, with a little clickable area inside.

<div class="window">
	<input type="button" value="Button" onclick="alert('Howdy partner!')" style="width:100%; height: 100%; opacity: 0.0;"/>
</div>

> Oh, that's much simpler!

Pay attention to how to add the `tap` to the window, you pass it _as a value_, instead of calling `appendChild`.

This is important. It's what allows you to create "Components" in this manner:

```typescript
// This is a component! It's just a function that returns UI!
function Button(rect, callback) {
	return new Tap(rect, callback)
}

App.run(
	Button (
		new Rect(0.0, 0.0, 100.0, 20.0), // rect
		() => alert("Howdy partner!")    // callback
	),
)
```

> Oh!!!

But, hey, let's pretend for a second that you are the one writing this library. For `App.run`, it will receive our UI as the parameter.

```typescript
class App {
	static run(ui: UI) {
		while(true) {
			const nextEvent = await OS.getNextEvent();
			// Handle events here.
		}
	}
}

interface UI {}
```

> How does it know what to do when an event arrives?

It does not, knowing how to handle the event is actually a responsibility of the UI.

```typescript
class App {
	static run(ui: UI) {
		while(true) {
			const nextEvent = await OS.getNextEvent();
			ui.handleUIEvent(nextEvent);
		}
	}
}

interface UI {
	handleUIEvent(event: UIEvent): void;
}
```

We can create `Tap` as a `UI` that has an implementation of `handle_ui_event`.

```typescript
class Tap implements UI {
	rect: Rect;
	action: () => void;

	constructor(rect: Rect, action: () => void) {
		this.rect = rect;
		this.action = action;
	}

	handleUIEvent(event: UIEvent) {
		if (event.type instanceof MouseButtonEvent && event.isPressed) {
			if (this.rect.includesPoint(event.position)) {
				self.action.call();
			}
		}
	}
}
```

> Oh, so the code for handling a click gets self-contained entirely within `Tap`, and I don't have to look at it ever again.

Yep. Now on to visuals!
## The Collage

Here are the button's visuals. A gray rectangle and text saying "Clickety click me!" overlayed on top:

<button disabled>Clickety click me!</button>

> Let me guess the steps...
>
> Render a gray rectangle... then just draw text on top.

"Just draw text" would make a graphics developer foam at the mouth...

> I mean, it's just a bunch of small shapes, can't be that hard!

Let's just focus on everything that isn't text for now.

The digital canvas of your screen, is a big rectangular "fabric" made of tiny little parts that can change colour individually. These picture elements (short: pixels) are what we will ultimately render all our graphics onto.

> Are you really gonna tell me what a "pixel" is?

Yes!

Pixels are easily modifiable by changing some memory sitting somewhere. They are layed down in the one-dimensional memory left to right, then top to bottom, the same way letters flow in a paragraph of text.

<blockquote class="showcase" style="display:flex;flex-direction:row;gap:1em;">
<style>
	img {
		image-rendering:pixelated;
		min-width: 64px;
	}
</style>
<img src="/blog/assets/8by8img.png"/>
	<div>[ Red, Yellow, Cyan, Green, Pink, Purple, Blue, Navy, Dark Gray, Light Gray, Darkish Light Gray, Lime, Pink, ... ]</div>
</blockquote>

> Yes Pedro do tell me what a pixel is...

_Patience..._

Drawing something like, say, a gray rectangle, would just be a matter of addressing each pixel one by one and changing its value.

```typescript
let image: MemoryBuffer = ...;
let rect = Rectangle { x: 0, y: 0, width: 100, height: 20 };

for y in (rect.y..(rect.y + rect.height)) {
	for x in (rect.x..(rect.x + rect.width)) {
		image[x + y * image.width] = Color::gray();
	}
}
```

> I see...

Now... your computer's _Central Processing Unit (CPU)_ is very powerful, yes. [It can modify millions of pixels in a tenth of a second](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=840c89a3b505da2d0f4915692483cdba).

But if we want to draw an app in 60 FPS, we have short of sixteen milliseconds to not only _draw_ to the screen, but calculate the colours, layout, state, everything.

The flaw of the CPU is that it executes instructions one after the other. And that is a shame, because the code to paint one pixel is independent from the code to paint other pixels.

> What if we use multi-threading? Like, with a quad-core processor.

Then you get to be four times faster.

> Oh. Uh... what if we could get a big bucket of paint and throw at the screen...

Sure!

> What?

Consider this: "what if we had LESS powerful computation cores but had thousands of them?"

That would be what we call a _Graphics Processing Unit._

Like I said, GPUs are less powerful than CPUs per computing core, but they have the capability to interact with every pixel all at once, cutting rendering time by orders of magnitude. Like throwing a bucket of paint at the screen.

Instead of writing the pixel filling code ourselves, we talk to the GPU through a graphics library (_OpenGL_, _Vulkan_, _DirectX_, _Metal_) and tell it to do that work for us.

And it looks _kind of_ like this:

```typescript
// Highly paraphrased
const shader = "#version 300 void main() {drawMyStuffPleaseThanks();}";
const rect = new Rect(0, 0, 100, 20);
const params = { color: new Color(0.5, 0.5, 0.5) };
const screen: MemoryBuffer = ...;

// The GPU has its own memory;
// we need to copy our geometry data
// to a buffer in the GPU.
const geometry_gpu: GPUMemoryBuffer = ...;
const screen_gpu: GPUMemoryBuffer = ...;

vkMakePipeline(rect, shader);
...
vkCopyBuffer(rect, geometry_gpu);
vkYouWillRenderThisGeometry(geometry_gpu);
vkYouWillRenderItWithTheseParameters(params);
vkRenderMyRects();
vkCopyBuffer(screen_gpu, screen);
...
```

> Um, Okay, What? What is this?

Yeah, okay, this might look a slightly more verbose, but it is just some pseudo-code. I can assure you that, in practice, it is much, much, **much** more verbose.

Graphic libraries like _Vulkan_ try to give you as much control of the GPU as they can by letting you change state, rendering modes, allocate buffers, configuring the rendering pipeline to your need. The cost of all that freedom is having to be explicit about every step of the way. But the reward is that now things that would take seconds take milliseconds.

Silver lining is, a lot of it only needs to be done once. For example, once you have set up a GPU program, allocated the memory buffers, etc, you can reuse a lot of that work for the next frame.

> So, you would only need to create the buffers once.... And the pipeline, too... and then for the rest of the program you just keep calling `vkRenderMyRects()`...

Yes. And of course, send some new data whenever anything changes.

> Wait a second!

Hm?

> You just told me that copying data from place to place with the CPU is slow... but to render things with the GPU you need to copy the data to it every frame.

Well-

> If you have one rectangle, sure, that's just a few bytes... but if you have a million...

Very perceptive. The first good thing is, you don't have to send new geometry for _every_ rectangle. They all have the same geometry.

One of my favourite things GPUs can do is _Instanced Drawing_, where you specify a geometry once and draw it _like, a million times_ by just saying:

```typescript
vkCmdDrawIndexed(like, a million times);
```

> Oh!

Yeah! With this, you can draw not just rectangles, but _entire worlds_ interactively.

> Like Minecraft!

Yeah! Like... Minecraft, sure. Imagine how many rectangles a Minecraft world has to render! It is a lot! In the case of UI, we have an advantage though. Because UI is 2D and elements sit in nested boxes, if something changes, we need not re-render the entire screen, but only the area that changed... in which case we only send over the positions and colours of the rectangles that changed.

> Woah! That's great! I sure hope I never have to write any of that!

Haha! Fair.

Here is how you would specify you want a rectangle to be drawn in UI Composer.

```typescript
let rect = new Rect(0.0, 0.0, 100.0, 20.0).with_color(Color.GRAY);
//   ^? Graphic

App.run(
	rect
)
```

> Oh. Just like the "tap."

Yes.

> Wait, I can understand that! I guess the `UI` interface that we made earlier would have a method for handling the GPU interaction...

```typescript
class App {
	static run(ui: UI) {
		while(true) {
			const nextEvent = await OS.getNextEvent();
			ui.handleUIEvent(nextEvent);
			ui.redraw();
		}
	}
}

interface UI {
	handleUIEvent(event: UIEvent): void;
	redraw(): void;
}
```

> Yeah, that makes sense! It looks very simple.

It is actually nothing like that, but let's not worry about that for now!

> What??

Really, we will get there!

The important part is that you can create components by aggregating primitives.

```typescript
function Button (rect, text, action): UI {
	return [
		rect.with_color(Color.GRAY),
		new Tap(rect, action),
	]
}

// Then, to use it:

App.run(
	Button(
		new Rect(0, 0, 100, 20),
		() => alert('Hello, there!'),
	)
)
```

> Oh, the title of this post makes sense now! Yeah this looks rad- WAIT.

Hm?

> This makes no sense at all! Shouldn't the function return `Array<UI>`?
> I thought it was a typo at first, but you did not even rewrite the `App.run` code to handle multiple elements?

Oh, the return type is correct. We just need to make sure that any `Array<UI>` also implements `UI`.

> You can't??? implement an interface for Array??? That's a type from the standard library!

Not in typescript, no, but UI Composer is written in _Rust_. In Rust, we can implement interfaces for anything. Kinda like this:

```typescript
impl interface for Array<UI> {
	handleUIEvent(event): void {
		for (const item of this) {
			item.handleUIEvent(event);
		}
	}

	redraw(): void {
		// Likewise...
	}
}
```

> A- ah...

If that's too weird, for now, you can pretend I added special handling in `App.run` for arrays. But the distinction between JS and Rust will become important in later blog posts.

> I do like how you can achieve some good rendering with not that many layers of abstraction. It feels... lightweight.

And still capable of doing everything a more powerful library can do. And then do it faster.

I am talking about digital audio workstations, code editors, painting programs...
## Adding State
If you have a button in a [**retained** mode](https://en.wikipedia.org/wiki/Retained_mode) UI, like in a browser with _HTML_, or in _GTK_ or _QT_, and you want to create elements, you need to... well, _create_ elements, and then "configure" them so they work together.

Consider the case of having a button making an element appear or disappear.

<blockquote class="showcase" style="display:flex; gap:1em;">
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

```typescript
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

> Now that you mentioned "creating big apps," I can see that writing all the updating code yourself can get cumbersome.

Cumbersome, prone to error, and requires you to _create_ elements that exist in memory...

Now check out the **immediate** mode equivalent.

```typescript
const visible = false;

function RenderUI() {
	Begin();
    if (Button(visible ? "Hide" : "Show")) {
        visible = !visible;
    }
    if (visible) {
        Text("Hello there!");
    }
	End();
}

```

> WHOA.

I know, right? Notice that you also never do _create_ elements.

> No? What does `Text` do?

Well, it does _not_ create an element. There is no "object" that sits in memory can be manipulated by you and whatnot. Basically, if the `Text()` function gets called in a frame, it means a little text exists in that frame. If it is not called, it does not exist. Same for the button.

> Oh, let me guess: if the button was pressed in that frame, the call to `Button(...)`, will return true?

Yes.

> It's so much less code!

Yeah! Because there is no code here that tells the UI how to update. When a change happens, it simply redraws the entire window from scratch. Possibly every frame.
<span style="opacity:0.2;">There are some optimisations, some good cache, but it still not suited for huge UI loads.</span>

For UI Composer, however, I wanted to have fine grained awareness of what can change what, when and how, so I can choose to re-render optimally.

> How do you know exactly what to re-render?

Well, the user tells me.

> So... like the cumbersome example you showed???

Uh- um- well, yeah but, like, with a different API, this complexity can be managed nicely by using Functional Programming!!!

> I don't think I want to be writing some esoteric math-thingy. I've tried looking at Haskell code before and my brain just could not understand it.

Funny you say that, since all the UI Composer user code I showed so far is functional.

> Huh?

What I mean by functional programming is... you do not just call a function to add a component... you _return_ it.

```typescript
function ShowAndHide(): UI {
	const rect = ...;
	const isVisible = false;

	return [
		Button(
			rect,
			isVisible ? "Hide" : "Show",
			() => isVisible = !isVisible,
		),
		isVisible ? Text(rect.translated(...), "Hello, there!") : []
	];
}
```

> As if "functional programming" is just "returning things instead of calling functions that do things."

Yes! That's exactly what it is.

> Really?

Yes!

> Have I been scared of this the whole time?

😛

> But even if you do return the UI, would this not still require redrawing the entire screen every time the variable changes?

Patience!
Here is how we make the UI know what reacts to what.

```typescript
function ShowAndHide(): UI {
	const rect = ...;
	const isVisibleState = new Editable(false);
	//          ^? Editable<bool>

	return isVisibleState.map((isVisible) => [
		Button(
			rect,
			isVisible ? "Hide" : "Show",
			() => isVisibleState.set(!isVisible),
		),
		isVisible ? Text(rect, "Hello, there!") : []
	]);
}
```

> Of course it was gonna get more complicated!

Haha.

> Let's see... you changed `isVisible` to `isVisibleState` which is now of type `Editable<bool>`... whatever that is.
>
> And you wrapped the UI part inside `isVisibleState.map(...)`.

Yep, `Editable`, is a monad that adds reactivity to a value.

> Monad? What? I don't... You lied to me! What is this!

You _also_ know what monads are! Consider this typescript code where you get a number and convert it to a string:

```typescript
function getSomeNumber(): number {
  ...
}

let num = getSomeNumber();
//   ^? number
let string = num.toString();
//   ^? string
```

> Okay, so you have a number then you convert it into a string.

Now suppose that this function _takes_ a while to complete. Maybe because it reads a file or does a `fetch` call to some API.

> That would make it `async`!

```typescript
async function getSomeNumber(): Promise<number> {
  ...
}
  ```

More specifically, it will no longer return `number`, but `Promise<number>`. But now we can not call `toString()` on the result to get our stringified number anymore.

> Which makes semantic sense, because the number isn't necessarily "here" yet.

Yes. But what if we want to specify what to do when the number comes in advance?

> We'd use `.then` with a callback that tells you what to do with the number that will arrive.

```typescript
let num_promise = getSomeNumber();
//     ^? Promise<number>
let string_promise = num_promise.then( num => num.toString() );
//     ^? Promise<string>
```

> Yeah, like that.

Then, we can pass `string_promise` forth for doing more transformations on it until eventually something `await`s it.

Now imagine this. What if this number that _will_ arrive... could arrive _more than once_?

> Oh.

Such construct is what we call a "Signal."

```typescript
function getSomeNumberState(): Signal<number> {
  ...
}

let numSignal = getSomeNumberSignal();
let stringSignal = numSignal.map( num => num.toString() );
```

> Oh my 😮.

Likewise, `string_state` is passed forth until eventually something `await`s it.

By mapping on the `Signal<String>` there you can eventually get a `Signal<LayoutItem>`. If you give UI Composer a `Signal<LayoutItem>` it will "await" it. Every time this Signal "resolves" with new UI, UI Composer will re-render it.

```typescript
let label = stringSignal.map( string => Text( string ) )
```

And it would re-render _only_ the UI the signal resolved with, not the whole screen.

> !!!
>
> So this allows you to tell exactly what part of the UI changed.

Yes.

> Bring me the reactive ShowAndHide component again.

```typescript
function ShowAndHide(): UI {
	const rect = ...;
	const isVisibleState = new Editable(false);
	//          ^? Editable<bool>

	return isVisibleState.map((isVisible) => [
		Button(
			rect,
			isVisible ? "Hide" : "Show",
			() => isVisibleState.set(!isVisible),
		),
		isVisible ? Text(rect.translated(...), "Hello, there!") : []
	]);
}
```

> Okay, let's see.
> So Editable is a `Signal`. This code returns a `Signal<UI>`, which, I assume also implements `UI`.

Yes.

> Because it is a Signal, it can notify the App, whenever it resolves, that it has to redraw part of the screen, that is, with the UI it resolved with.

Yep.

> Oh, and whenever you call `.set` on it, it triggers a new "resolve" of the state!!!

Exactly!

And, by the way, since "state" is a first-class value now, we can do things like returning it from a function and have potentially thousands of UI items reacting to it each on their own, without manually writing every interaction.

> There's one thing I don't understand though...

*sigh...*

> Hey, don't get mad at me! You're literally the one writing my dialogue lines!

> So far, whenever you've drawn an element, like a Button, you've passed a variable named `rect` to it. Do I have to manually specify the exact pixel coordinates of where I want my button to be?

Yeah, no, in UI Composer you just pass the Button without a rect.

```typescript
App.run(
	Button(() => alert('Hello, there!'))
)
```

The UI you pass to `App.run` is layed out into a "container," and when you put a Button on it, it makes the button fill the whole window.

<div class="window">
	<input type="button" value="Button" onclick="alert('Hello there!')" style="width:100%; height: 100%;"/>
</div>

> Wait... like you said, there's no "Button" element created...

Yes...

> So, how can the window resize the button? Doesn't the call to `Button` just return a `UI`? The library can't even introspect inside of the button to change it!

I've been waiting for us to get here.

Hear this... instead of returning UI straight up... the Button will return a _closure_ that returns UI.

> A closure?

A function.

> A function returning a function???

Yep! That's called a _Higher Order Function_, and is very common in functional programming.

```typescript
function Button(action): (rect: Rect) => UI {
	return (rect) => [
		rect.with_color(Color.GRAY),
		new Tap(rect, action),
	]
}
```

> WOAH.

In the library, actually, the inner closure can receive more than just `rect` from is parents, for example: theme (light or dark mode), gaps, locale, layout direction, user handedness...

```typescript
function Button(action): (hx: ParentHints) => UI {
	return (hx: ParentHints) => [
		hx.rect.with_color(Color.GRAY),
		new Tap(hx.rect, action),
	]
}
```

When you call `Button(...)`, what you are passing as a parameter to `App.run` is that inner closure. Every time the window resizes itself, it calls the closure to get new UI.

> This is great! What if I _don't_ want my Button to be Window-sized, though.

You can add containers between it and the window. A component, of course, is also just a higher order function.

In Standard UI, one of the design systems built on top of UI Composer, you have `Center`.

```typescript
App.run(
	Center (
		Button(() => alert('Hello, there!'))
	)
)
```

<div class="window" style="display: flex; align-items: center; justify-content: center;">
	<input type="button" value="Button" onclick="alert('Hello there!')"/>
</div>

> How does the window know that it now needs to centre the button?

It does not. In fact, it can not even "see" the button.

> Huh?

The window makes the `Center` component fill the entire window. Then, `Center`, centres the button by calling `lay` with a new rect.

```typescript
// A Container is a component that takes
// a component (which is a higher order function).
// It is a Higher Order Component.
function Center(item: LayoutItem): LayoutItem {
	return (hints: ParentHints) =>
		item.call({
			...hints,
			rect: hints.rect
					.center()
					.asRect()
					.withSizeCentered(/* The button size */)
		})
}
```

> Do the other hints cascade downwards, like with CSS?

Yes! A Container will usually modify one of the hints and pass the rest of them to its children unchanged.

> What if I want to lay out several buttons side to side?

You use an other container. Like `Row`.

```typescript
App.run(
	Row (
		Button(() => alert('Hello, there!')),
		Button(() => alert('Hello, there!')),
	)
)
```

Fun fact! Like all the components in Standard UI, `Row` is semantic by default! Instead of laying down elements "left to right," it lays them down in "Writing Order."

Because of `ParentHints`, it is aware of the current layout direction, and can lay the elements down right to left when you inform the `App` of a new locale.

> Interesting!

<div class="window" style="display: flex;">
	<input type="button" value="Button" onclick="alert('Hello there!')" style=" height: 100%;"/>
	<input type="button" value="Button" onclick="alert('Hello there!')" style=" height: 100%;"/>
</div>

> Wait! How does the `Row` know to put one button besides the other? It would need to know the size that the first button occupies... but the `Button(...)` call only returns a function.

If only we could add metadata to functions...

> Woah! Can we do that???

No. <span style="opacity: 0.2;">Thankfully.</span>

We will stop returning a closure and start returning an object instead. This object will contain two fields:

- The closure that returns UI;
- Some metadata;

> Child hints!

Yes!

```typescript
interface LayoutItem {
	hints: ChildHints;
	lay(hx: ParentHints): UI;
}

interface ChildHints {
	minimumSize: Size;
	naturalSize: Size;
	// ...
}

function Button (action): LayoutItem (
	return {
		hints: { minimumSize: ..., naturalSize: ... },
		lay: (hx: ParentHints) => [
			hx.rect.withColor(Color.GRAY),
			new Tap(hx.rect.translated(...), action),
		]
	}
)
```

> Oh... so `Row` receives two `LayoutItem`s. It reads the size of one, then it takes that into account when calling `lay` for the next one.

Hm. Interestingly, you did not question if child hints cascade...

> They do?

Yes! They cascade _upwards_. A container like `Center` is a `LayoutItem`, right? It has child hints of its own. It defines its minimum size as being the minimum size of its child.

> And Row defines its minimum size as being... the sum of the minimum sizes of the children!

Yes! Plus the gap, if there is one. And when laying items out, a container will _never_ force an element past its minimum size.

> And a container will never _be_ layed out past _its_ minimum size?

Yes! Even the window can never shrink past the minimum size of its child.

> So overflow is impossible!!!

Yes!

As long as you implement `Button` to respect the bounds of its `ParentHints`, overflow is impossible.

If you do want a component to hold more items than it should, you can opt _in_ to some specific way of achieving that.

```typescript
App.run (
	Scroll (
		Row (
			// Add as many buttons as you want!
		)
	)
)
```

This is what I meant by _soundness_ in my previous post. The behaviour of the UI is predictable down to the pixel.

> CSS could never!!!

Finally, let's replicate the "ShowAndHide" example with all that we learned.

```typescript
function ShowAndHide(): LayoutItem {
	const rect = ...;
	const isVisibleState = new Editable(false);

	return isVisibleState.map((isVisible) => Row(
		Button(
			() => isVisibleState.set(!isVisible),
		),
		isVisible
			? Text("Hello, there!")
			: []
	));
}

function Button(action): LayoutItem (
	return {
		hints: { minimumSize: ..., naturalSize: ... },
		lay: (hx: ParentHints) => [
			hx.rect.withColor(Color.GRAY),
			new Tap(hx.rect, action),
		]
	}
)
```

<div class="window" style="display: flex;">
	<input type="button" value="Button" onclick="this.parentElement.querySelector('span').classList.toggle('hidden')" style=" height: 100%;"/>
	<span class="hidden" style="user-select: none;">Hello there!</span>
	<style>.hidden { display: none; }</style>
</div>

Want to know the best part? Because UI Composer is written in Rust, all of these closures, objects and functions that would make a JavaScript program slow, incur no cost -- they are Zero-Cost abstractions.

So, in the end, the UI you write gets compiled to, essentially, the equivalent of you having written the OpenGL/Vulkan/Metal commands yourself.
## Conclusion
So, now you know the basic premise behind the UI Composer API.

Next week, I'll enter in more detail regarding _how exactly_ to use UI Composer concepts to make GUIs that scale. I'll introduce _Resources_, the _Editor Pattern_, _Meta-Components_ and Animation maths.

Until then!

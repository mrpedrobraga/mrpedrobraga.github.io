---
date: 2024-12-21
tags:
  - graphics
  - ui-composer
title: A Manifesto for Better UI
description: A pitch of a new UI library!
---
I _love_ software.

I'm a Software Maker™ myself, and I certainly use a lot of it for everything I do professionally and creatively. Whether it is taking research notes, painting masterpieces, engraving music sheets, they have my back :D

Part of my job as a game developer is [making one-purpose tools for artists](https://mrpedrobraga.itch.io/modot-distortionator) to _do their thing_. For this reason, I have been researching the world of GUI app development, comparing different ways of creating apps.

**Native Libraries** tends to be more performant: faster processing, quicker input responses, lower install sizes... but the innovation brewing in **the Web** development community is unparalleled.

Alas, none of the approaches excel in the case of rendering arbitrary graphics that superimpose or interact with the layout. Custom colour pickers, visualisers, connection cables, the kind of graphs you use in art tools.

At last, I decided to take these matters into my own hands and develop a new UI Library that combines the strengths of each paradigm into a beautiful little package.

We have learned a lot since the 2000s, after all. By starting from scratch, with the benefit of hindsight, we can get the best of all worlds.

## UI Composer!
Getting started is pretty simple!

Create a new project with `cargo init` and add `ui-composer` as a dependency. Then, add a window to your app.

```rust
use ui_composer::prelude::*;

pub fn main() {
    App::run(Window(()));
}
```

And... you are done.

You can run your app with `cargo run` to validate it works, and then go right to building!

Here is an obligatory reactive counter example.

```rust
use ui_composer::prelude::*;

pub fn main() {
	App::run(
		MyApp()
	)
}

pub fn MyApp() -> impl LayoutItem {
	let count_state = Editable::new(0.0);

	Window(
		Center(
			Row((
				Button("-", || count_state.set(count_state.get() - 1)),
				Button("+", || count_state.set(count_state.get() + 1)),
				count_state.signal()
					.map(|count| Label!("Count: {count}"))
					.collect_ui()
			))
		)
	).with_title("Counter")
}
```

Cool, right?
## Project Goals
- **Iteration:** Creating a new app should be quick. The API should be declarative and expressive. Importing and sharing components should be seamless. Users should be able to _play_ with ideas without committing to them, and truly enjoy their creative process;
- **Speed:** UI should run smoothly and interactively, applications should start nearly immediately, elements should update quickly, without lagging or stuttering;
- **Simplicity:** The framework should be lightweight and simple, focusing on delivering a small set of well-maintained features;
- **Capability:** By building onto the building blocks of the library, you should be able to create very complex applications, with up to millions of signals wiring every part of the application together elegantly in bigger codebases;
- **Soundness:** If a user can describe something, they can make it, without having to fight a legacy technology's idiosyncrasies. There should not be any random layout switches, flashes, clipping, and unexpected behaviours. The library should _make sense_;
- **Polish:**  The library should have first-class support for reactive animations, from good kinetic scrolling to the tiniest micro-interactions. All the visual effects that UX developers dream of, front-end developers should not fear implementing them;
- **Accessibility:** Elements in the UI should be _semantic_ by default. They should be highly structured, interface with screen readers and react to the final user's specific needs: theme, locale, screen orientation, motion sensitivity, handedness;
- **Extensibility:** Users should be able to author new looks and behaviours for the UI without needing to "_hack_" it. That is, they should be able to create their own functions and components as high-level as they can get away with, but as low-level as they may need;
## How can we achieve them?
The choice of *Rust* as the language was very important: it is a language that is very fast and cares a lot about memory safety, but, most importantly, **conceptual safety**. By encoding semantics into the types in the language, [strictly enforcing invariants](https://www.youtube.com/watch?v=z-0-bbc80JM), it is possible to create resilient UI components that do not clip through each other unexpectedly or do any other unpredictable behaviours.

The API is inspired by *functional programming*. State synchronisation (reactivity AND responsiveness) are achieved through monads. Likewise for procedural processes and animations. Since FP is not as mainstream, there is a slightly steeper learning curve to learning UI Composer, but the results are you not having to guess what value makes what react when. Rust is very friendly to the functional paradigm.

It is also very easy to install, and has one of the best official package managers, `cargo`.

```
// Rust being very easy to install:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The library is designed as a wrapper around the local graphics library (OpenGL, Vulkan, WebGL, Metal). It does the work of creating windows and talking to the GPU efficiently in all platforms one your behalf, and nothing more.

The best part is, it is possible for the library to be both fast and have a good API due to the existence of Zero-Cost abstractions.

You do not pay a performance fee for organisation (the way you do with an interpreted language). In many cases, your high-level code is the equivalent of you having carefully written the low-level graphics commands yourself.
## Considerations
As of the moment, UI Composer is still experimental. Graphics development is hard!

I have much to research about myriad topics: order-independent transparency, deferred anti-aliasing, multi-threading, hot-reloading, texture allocation, repainting, embedded systems, over-the-network reactivity and so much more.

I have so much to talk about!

If you are as excited about it as I am, you will enjoy following this journey through the next blog updates.

Until then!

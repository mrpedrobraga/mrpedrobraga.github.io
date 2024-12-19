---
date: 2024-12-17
tags:
  - graphics
  - ui-composer
---
I _love_ software.

I certainly use a lot of it for everything I do professionally and creatively. Whether it is taking research notes, painting masterpieces, engraving music sheets, they have my back :D

Yet, I must not be the only one who noticed that using the computer has felt slower and slower over time. As time passes and computer hardware evolves, you would think things would be getting faster, right? Sure, we now have more ambitious features, smooth animations, transparencies, but computers also run games, entire worlds, 60 times a second (that is under 16 milliseconds per render), so what is the excuse?

As a Software Maker™ myself, so I have high interest in the _development_ side of the thing. Part of my job in game dev is to create tools for asset editing and visualisation, enabling artists to interface with the game in novel ways.

> My most recent public tool is [Distortionator](https://www.youtube.com/watch?v=0OEWLVnX30A), which lets artists in the [Mother Encore](https://motherencore.com/) team do battle backgrounds easily.

I have been researching the world of GUI app development, comparing different ways of creating apps of their ecosystems.

Each approach has its own strengths and weaknesses:

Native tends to be most performant, but is not _nearly_ as expressive and rich in ecosystem as web is. The innovation and creativity that brews in web front end development is unparalleled. Native has not quite caught up to the concept of Functional components, reactivity and pleasant development environments.

On the other hand, the Browser really was not made to make apps, and it shows. Building for browsers introduces unnecessary hurdles. Besides, if a web app is to be used in a desktop environment, you would have to bundle a whole web browser embedded with each app you make, which is a silly idea that no one would seriously consider.

And all of the approaches fail at the task of rendering arbitrary graphics (shapes, 3D Models) easily and seamlessly integrated within the UI layout. Complex apps usually have to resort to using OpenGL directly.

At last, I decided to take these matters into my own hands and develop a new UI Library that combines the strengths of each paradigm into a beautiful little package.
## And that would be UI Composer!

 > Why have yet another way of making UIs?

Because we have learned a lot since the 2000s. Software development has never been so cozy, but UI libraries have not quite caught up. By starting from scratch, we can get the best of all worlds while avoiding the bloat.

The specific goals of this project are:
## Goals
- **Performance:** UI should run smoothly and interactively, applications should start nearly immediately, elements should give snappy feedback;
- **Simplicity:** The framework should be thin and simple, focusing on delivering a small set of well-maintained features;
- **Capability:** By building onto the building blocks of the library, you should be able to create very complex applications, with up to millions of signals wiring every part of the application together elegantly in bigger codebases;
- **Soundness:** If a user can describe something, they can make it, without having to fight a legacy technology's idiosyncrasies. There should not be any random layout switches, flashes, clipping, and unexpected behaviours. The library should _make sense_;
- **Iteration:** Creating a new app should be quick. The API should be declarative and expressive. Importing and sharing components should be seamless. Users should be able to _play_ with ideas without committing to them, and truly enjoy their creative process;
- **Polish:** From good scrolling in every platform to the slightest button interactions. The library should have first-class support for reactive animations. All the visual effects and micro-interactions that UX developers dream of: front-end developers should not fear implementing them;
- **Accessibility:** Elements in the UI should be _semantic_ by default. They should be highly structured, interface with screen readers and react to the user's specific needs: theme, locale, screen orientation, motion sensitivity, handedness;
- **Extensibility:** Users should be able to author new looks and behaviours for the UI without needing to "_hack_" it. That is, they should be able to create their own functions and components as high-level as they can get away with, but as low-level as they may need;

Sounds too good to be true? Let us explore this.
## How can we achieve them?

The choice of *Rust* as the language was very important: it is a language that is very fast and cares a lot about memory safety, but, most importantly, **conceptual safety**. By encoding semantics into the types in the language, [strictly enforcing invariants](https://www.youtube.com/watch?v=z-0-bbc80JM), it is possible to create resilient UI components that do not clip through each other unexpectedly or do any other unpredictable behaviours.

Rust is also very easy to install, and one of the best official package managers, `cargo`.

```
// Rust being very easy to install:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The library is designed as a wrapper around the local graphics library (OpenGL, Vulkan, WebGL, Metal). It does the work of creating windows and talking to the GPU efficiently in all platforms in your behalf, and nothing more.

> You understand this if you know [Raylib](https://www.raylib.com/) and how good it is for video game programming.

Unlike other alternatives, the API is inspired by *functional programming*. State synchronisation (reactivity AND responsiveness) are achieved through monads. Likewise for procedural processes and animations. Since FP is not as mainstream, there is a slightly steeper learning curve to learning UI Composer, but the results are you not having to guess what value makes what react when.

The library makes heavy use of use Zero-Cost abstractions that get dissolved during compilation stage into highly efficient code, meaning that you do not pay a performance fee for organisation (the way you do with JavaScript). In many cases, your high-level code is the equivalent of you having carefully written the low-level graphics commands yourself.
## Okay, I'm digging that! Let's see some code though...

To get started, you will create a new Rust project with an `ui-composer` dependency and write the code to run a window.

```rust
use ui_composer::prelude::*;

pub fn main() {
    App::run(Window(()));
}
```

This is it.
You can run your app with `cargo run` to validate it, and then go right to building!

Here is an obligatory reactive Counter example.

```rust
use ui_composer::prelude::*;

pub fn main() {
	App::run(
		MyApp()
	)
}

pub fn MyApp() -> impl NodeDescriptor {
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

Great! And you pretty much learned 50% of the library's basics. There is no HTML/JS/CSS separation: there is only _code_.
## Considerations
As of the moment, UI Composer is experimental. Graphics development is hard!

For example, it is not a thing to understate how slow rust is to compile. The slowness is a consequence of it being compiled and all of the checks it does for you... but it does end up hurting iteration. This is a problem everyone in the Rust ecosystem is aware of, so there is a lot of good research on this to be done.

I still have a lot to research about many aspects: order-independent transparency, deferred anti-aliasing, multi-threading, hot-reloading, texture allocation, repainting, embedded systems, over-the-network reactivity and so much more. I have so much to talk about!

If you are as excited about it as I am, you will enjoy following this journey through the next blog updates.

Until then!
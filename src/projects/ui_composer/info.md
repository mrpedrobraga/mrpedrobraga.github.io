The ui-composer project is a functional-reactive UI rendering library written in Rust. It uses function composition patterns to statically build a type safe UI that can rerender optimally as it listens to changes in the model state.

Please check the /docs directory for API documentation and technical explanations.

It is currently in early development, but a prototype has been made in the TypeScript library `femto-ts`. Here is the proposed API in Rust.

```rust
use ui_composer::prelude::*;
use ui_composer::std::prelude::*;

fn App(cx: AppContext) -> impl UIFragment {
    let count = cx.state(0);

    Row(
        Button("-", || count.incr(1)),
        count.map(|x| Label(format!("Counter: {x}")),
        Button("+", || count.incr(-1)),
    )
}

fn main() {
    let app = UIComposer::new()
        .with_window_title("My Counter")
        .with_window_size(640, 360)
        .build(App);
}
```

The internal build API mimicks the APIs and behaviours of standard Rust allocation. Every thing is "stack-allocated" (batched into one instance buffer and draw call) by default, but can be "dynamic-allocated" (separated into its own buffer and draw call) when you need. You opt-in to dynamic allocation behaviours using UI smart pointers.

For example, if you had to render a container with contents that can change in number at runtime, you'll need to use the UI analog of a 'Vec'.

```rust
fn App(cx: AppContext) {
    let todo_list = cx.state_vec(vec!["Do dishes", "Make wishes", "Catch fishes"]);

    ColumnVec(todo_list.map(|text| Label(text)))
}
```

Through this system, only the exact resources needed for the desired UI behaviour are used, and without any "guessing" magic nor active "optimization." The performance of this library comes from the sheer preliminary simplicity of the building blocks.

This also means the UI can be geometrically type safe: no clipping or overflow can happen, since the behaviour and interplay between components is explicitly defined a priori in your code.

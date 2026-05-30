---
title: cables (0.1.0)
description: A DAG compiler library for Rust.
tags:
  - software
  - library
link: https://github.com/mrpedrobraga/overtone/tree/main/crates/cables-core
---
<center><a href="https://github.com/mrpedrobraga/overtone/tree/main/crates/cables-core"><img src="https://img.shields.io/badge/github-repo-blue?logo=github"></a></center>

This is a tool that will only make sense if you're a developer.

Cables is library for creating directed acyclic graphs that will be executed thousands of times a second.
It's inspired by Blender's geometry nodes in which nodes may have multiple input and output sockets.

I created this implementation of `Graph` to be used for real-time audio processing. At 44100Hz, a standard
sample rate for audio, an entire graph like this needs to run under 22 microseconds.

Even though this graph was designed for audio, you can use it for anything else.
High sample rate never hertz. Hurts. Whatever.

```rust
fn main() {
    let mut graph = Graph::new();
    
    let a = graph.insert(Num::new(40.0));
    let b = graph.insert(Num::new(60.0));
    let ab = graph.insert(Sum);
    
    graph.connect(a, 0, ab, 0);
    graph.connect(b, 0, ab, 1);
   
    // Compile the graph into an efficient pipeline.
    // Compiling the graph, traversing it, checking the types
    // is slow (in the order of microseconds).
    //
    // You need to call this every time the graph changes.
    let pipeline = graph.compile();
    
    // Running a compiled graph
    // is fast (in the order of nanoseconds).
    let result = pipeline.run();
}
```

You _will_ need to create your own nodes. This library doesn't come with any.

Nodes are effectively just functions with a lot of reflection. [`cables-macro`](https://github.com/mrpedrobraga/overtone/tree/main/crates/cables-macro) can make creating these nodes easier, since they involve using `unsafe`.

```rust
struct Num(pub f32);

#[node_impl(fields(num = 0))]
impl Node for Num {
    fn process(out: &mut f32) {
        *out = num
    }
}

struct Sum;

#[node_impl]
impl Node for Sum {
    fn process(&self, a: &f32, b: &f32, out: &mut f32) {
        *out = *a + *b;
    }
}
```

Under the hood, a node has a "compile" function that takes an iterator of byte pointers, transmutes those pointers into references, then returns a closure that captures those references. The result is a packed array of closures in topological order—it can be executed from top to bottom with a simple for loop!

Take a look at [the file responsible for this](https://github.com/mrpedrobraga/overtone/blob/main/crates/cables-core/src/graph.rs), you'll find that `GraphPipeline::from_graph` is very complex, but `GraphPipeline::run` is exactly the code below. It doesn't get faster than this without a compiler to machine code.

```rust
pub fn run(&mut self) {
	for vertex in self.vertices.iter_mut() {
		vertex()
	}
}
```
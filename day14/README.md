# aoc2022

<img width="167" alt="image" src="https://user-images.githubusercontent.com/127909/205458140-e6064fd0-caae-4282-9be9-54a3ca8ac303.png">

Advent of Code 2022 in Rust.

This is Day 14 of AOC 2022 implemented as a [Leptop](https://leptos.dev/) wasm (web assembly) simulation.

The repo holds Part 2. Part 1 is in the commit history. Part 1 used the [isPointInPath](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath) canvas API call for collision detection, but Part 2 required many more checks and became prohibitively slow. Firefox's profiler indicated almost all of the time was spent in `isPointInPath`, so I refactored significantly to use a [bit vector](https://docs.rs/bitvec/latest/bitvec/index.html) for collision detection. A `Vec<bool>` would have been faster and used <2MB memory, but I took the opportunity to learn the `bitvec` crate.

I had several other things I'd like to have played with during this exercise:
- decoupling simulation loop from `requestAnimationFrame`, which would enable
  - simulating acceleration from gravity on falling sand in Part 1
  - enabling user controls to increase/decrease simulation speed
  - not tying simulation speed to screen's refresh rate
- playing with shading of rock/sand
- implementing some of the optimizations in [this MDN tutorial](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas)

However, this exercise already took me a long time due to learning several new concepts/frameworks/libraries at the same time, so I'll leave it here.

![image](https://github.com/djanderson/aoc2022/assets/127909/5ffcc6fa-5133-4e01-a62c-c2f3a448cd75)

`cargo leptos serve`

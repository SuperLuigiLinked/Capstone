# Preface

This repository is a copy of my Capstone Project from my Senior Year of College.

The contents are left more-or-less as they were when the project was submitted.

As is currently, this project should compile and run on Windows.
The source code for the Linux/MacOS implementations is not up-to-date.

To see some interesting demos, go to `rge` and try out some of the example programs, such as:
* `rge/examples/fireflies.rs`
* `rge/tests/game-of-life.rs`
* `rge/tests/space.rs`

For the programs in the `tests` folder, check the documentation at the top of each program for instructions on how to use them.

---

# Rust Game Engine + Windowing Library

My Capstone Project was to develop a cross-platform Windowing Library using native OS libraries as the backend, and then use that library to build a Game Engine/Framework that renders using the Vulkan Graphics API.

The code is written in Rust, with tests being run via `cargo test` and documentation generated via `cargo doc`.

## Features

The windowing library (titled **Wyn**) allows users to open/manipulate windows on the desktop, receive user-input (such as mouse, keyboard, and controller), and query information such as window positions and monitor sizes.

The game engine/framework (titled **RGE**) supports rendering simple 2D Graphics, such as polygonal shapes of many colors, as well as texture-mapped shapes.

## Users

The primary intended users for these libraries are Game Developers, but the libraries are general enough (the windowing library in particular) that even non-game Application Developers could find use for them, too.

# Rust Game Engine + Windowing Library

My Capstone Project will be to develop a cross-platform Windowing Library using native OS libraries as the backend, and then use that library to build a Game Engine/Framework that renders using the Vulkan Graphics API.

The code will be written in Rust, with tests being run via `cargo test` and documentation generated via `cargo doc`.

## Features

The windowing library (titled **Wyn**) will allow users to open/manipulate windows on the desktop, receive user-input (such as mouse, keyboard, and controller), and query information such as window positions and monitor sizes.

The game engine/framework (titled **RGE**) will, at minimum, support rendering simple 2D Graphics, such as polygonal shapes of many colors, as well as texture-mapped shapes.
The game engine/framework will also allow for the loading and playback of audio files.

## Users

The primary intended users for these libraries are Game Developers, but the libraries are general enough (the windowing library in particular) that even non-game Application Developers could find use for them, too.

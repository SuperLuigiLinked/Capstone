# Dependencies

The libraries developed in this project will target the 3 major families of Desktop Operating Systems: Windows, Linux, and MacOS.[^1]

## Rust

To build the libraries, you must install the Rust Compiler and associated tools.

These can be downloaded from the official website:

https://www.rust-lang.org/tools/install

Any external libraries used in this project are listed in the `Cargo.toml` files in the [Wyn](../wyn/Cargo.toml) and [RGE](../rge/Cargo.toml) directories.

When you compile the code, `cargo`, the Rust package manager, will automatically download these dependencies over the Internet.

## Vulkan

In order to run Vulkan code, the proper Drivers must be installed.

In order to build programs that use Vulkan, the SDK must also be installed.

These can be downloaded from the official website:

https://www.vulkan.org/tools

> **Note:** Vulkan is only needed for the Rust Game Engine part of the project. The Wyn windowing library does not depend on Vulkan.

If you try to build an RGE Program without first installing the Vulkan SDK, you may receive an error message like the following:

![image](https://user-images.githubusercontent.com/65352263/227596923-dec63ea2-c853-442d-9e87-094018a9bca6.png)


## OS

The native OS libraries used should come bundled with the operating system itself, and should not need to be downloaded/installed manually.

For reference, here are the underlying APIs used:

### Windows
* **Windowing:** Win32
* **Inputs:** Win32 + Xinput
* **Audio:** XAudio2

## VS Code

In order to integrate Rust into Visual Studio Code, you'll need to download the official "Rust Analyzer" extension:

https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

In addition, though not strictly necessary, it is recommended that you download the following Rust Extension Pack:

https://marketplace.visualstudio.com/items?itemName=Zerotaskx.rust-extension-pack

If you would like to view in-depth line-by-line code coverage results from within VS Code, then you must download this extension, too:

https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters

[^1]: Only the Windows part of this project will be counted towards my hours and tested/demonstrated in-class.

## Rainmeter Rust Plugin Template
This template allows for the creation of Rainmeter plugins using Rust.  
It serves as a starting point by providing a simple counter example, which can be found in the `Example.ini` file.

### Usage
This template provides API bindings and exported functions that closely resemble the ones found in the C# and C++ versions of Rainmeter plugins. For more information, please refer to the [Plugin Anatomy](https://docs.rainmeter.net/developers/plugin/plugin-anatomy) documentation.

### Building the Plugin
1. Install Rust (`winget install rustlang.rustup`).
2. Clone this repository.
3. Run `build.bat`.

It's important to note that `build.bat` has configuration options, make sure to check them out!

Additionally, plugin name, version and copyright can be found in `Cargo.toml`.
# MyCLI
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.63+-lightgray.svg)](https://github.com/markhj/rust-config-reader)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

> **Important!**
> This project is not yet ready for use.

Customizable CLI written in Rust

# Installing

## Optional: Add PATH in OS

# Getting started
## Without no previous packages
> **Notice**
> If you have used MyCLI before and have existing packages, you can skip to the next chapter.

The first step is to create a package. A package contains one or more commands.
Packages would typically be grouped by organization or sometimes by project.
But that's really up to yourself how you want to organize the packages.

Start by launching the CLI, and writing:
````
make:package name
````
Where ``name`` is replaced with something of your choice.

Now, you have to exit the CLI and add the following line in your ``config`` file:

````
[packages]
name = "./packages/name"
````
Again, replace ``name`` with the one you chose earlier.

Now, you can launch the CLI once again. To work with the package, enter:
````
use name
````

Now you have access to the commands (scripts) in the package, which - for a new package - is none.
To do something about that, we can create a command name ``helloworld``.

````
make:command helloworld
````

Commands created with this method have a single "Hello world" line in them. To execute it, simply write:

````
helloworld
````

Now the CLI will load up the _helloworld_ file and interpret its contents.
See the chapter about scripting language to learn how to make more interesting commands.

## With previous packages

# Scripting language


# Todo
- Validate package name when creating
- Validate command name when creating
- Global commands
- Automatically add new packages in the config file
- Make it possible to pass arguments from CLI-level (``my-cli.exe packagename some command``)
- 
## Scripting language
- System calls
- Match pattern

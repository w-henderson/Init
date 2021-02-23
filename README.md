![init Banner](images/banner.png)

# `init`: Initialise any project with one command

With the vast number of languages and tools used today, it's easy to end up with many different initialisation tools, each with their own unique syntax to remember. This, along with the fact that many languages don't have an initialisation tool at all, can really slow down development right at the start of a project. With `init`, you only need to remember one command which works for numerous languages and tech stacks, and which also provides an easy way to add more.

## Quick Start
Start by downloading a binary from the [Releases page](https://github.com/w-henderson/init) and adding it to your PATH. Then, the command `init` is used as follows:

| Command | Explanation |
| --- | --- |
| `init list` | Lists supported languages |
| `init <language>` | Initialises a project with the given language in the current directory, inherits name from current directory |
| `init <language> [name]` | Initialises a project with the given language in a new directory with the specified name |

### Arguments:
 - `-a, --author [author name]`
 - `-d, --description [short description]`
 - `-e, --extras [extra name] [another extra name]...`
 - `-l, --list-extras`
 - `--no-git`

## Adding a Language
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for more information about how to add a new language.

## Building
To build `init`, you'll need Rust and Cargo installed. Then, you can just run `cargo build --release`.
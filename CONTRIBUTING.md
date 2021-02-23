# Adding a Language

Firstly, thank you for considering helping out on this project! `init` will only be worthwhile if it supports most languages that one could want to use, so expanding our support is vital to this project's success. In this file, I'll outline the basics of how to put together an `init.json` file as well as adding extras and running commands.

## The Basics
Every language has its own subdirectory in the `data/` directory, and the name of each directory is the name users will reference the language by. It must be lowercase, have no spaces, and no special symbols. Inside the subdirectory should be an `init.json` file as well as all the files for the project and any optional extras. These will be sorted out in the initialisation file.

## The `init.json` File
The `init.json` file is the heart of the language. `init` will read and parse this file to figure out how to correctly initialise the project. The most basic possible `init.json` file looks like this:

```json
{
    "language": "Language Name",
    "files": [
        "{{projectNameLower}}.extension"
    ]
}
```
You might notice the special phrase `{{projectNameLower}}`, which represents the lowercase form of the project name. You can use special phrases anywhere, including in regular files, but we'll cover this in more detail later.

Alternatively, you can use a command or series of commands to initialise the project. Here's an example of this, taken straight from the Rust `init.json` file:
```json
{
  "language": "Rust",
  "commands": [
    "cargo init {{folderPath}} --name {{projectNameLower}} --vcs none"
  ]
}
```
You can see more special phrases being used here, as well as version control being turned off. This is important, as `init` handles creating version control itself so you *must* disable it if possible in your commands. You can also use files and commands together, with files being copied before commands are run.

## Special Phrases
| Phrase | Meaning |
| --- | --- |
| {{projectName}} | The project name as written with no changes, e.g. `My Project` |
| {{projectNameLower}} | The project name, lowercase and with no spaces, e.g. `my_project` |
| {{author}} | The specified author name |
| {{projectDescription}} | The specified project description |
| {{folderPath}} | The relative path to where the project is being created, e.g. `my_project` or `.` |

## Optional Extras
You can include optional extras for users to activate with the `-e` or `--extras` parameter. These at their most basic can contain additional files, and at a more advanced level can explicitly include or exclude sections within files depending on which extras are enabled. To define an extra in the `init.json` file, you need to add it to the `extras` array like so:
```json
{
    "language": "Language Name",
    "commands": ["init command"],
    "extras": [
        {
            "name": "extraName",
            "description": "A short description of the extra goes here.",
            "files": [
                "someExtraFile.extension"
            ]
        }
    ]
}
```
All extras must have a name and description, but specifying files is optional. The specified files will only be copied if the extra is enabled, and must not appear in the default `files` parameter, but may occur in other extras' files.

## Optional Sections of Files
If your extra requires a small change to an existing file, you can use some special syntax in the file to only allow certain parts to be copied if an extra is enabled, or only if it's not enabled. For example, here is an extract from the Jamstack project's `index.html` file.
```html
//!startExtra "typescript"
<script src="compiled/scripts/{{projectNameLower}}.js"></script>
//!endExtra
//!startExtra "!typescript"
<script src="scripts/{{projectNameLower}}.js"></script>
//!endExtra
```
You can see that to start an extra section, you use the syntax `//!startExtra "extraName"`, and to end an extra, use `//!endExtra`. You can also use `#!startExtra "extraName"` and `#!endExtra` if you prefer. You can also see in the lower part, the second script tag will only be included if the extra `typescript` is *NOT* included.

**IMPORTANT:** if you use this syntax in any file, you must declare the specified file in the array `filesContainingExtras` for optimisation purposes.

## More Information
For more information, take a look at some examples already in the `data/` folder.
# Diecast
#### A CLI template creation and management program in Rust

## Disclaimer
Use at your own risk.  Misusing this program can cause you to delete the entire contents of a folder with no way to get it back.  You are warned before any action that would cause an overwrite, but it's up to you to ensure you're in a directory that should have it's contents overwritten.

I offer no warranty on this software and I take no responsibility for any data loss.

**You have been warned**

## Purpose
Diecast provides an easy way to store, recall, and manage project templates.  This is for people who hate wasting time re-writing boilerplate just to get your project started.

## Installation
For now, the only installation method is to compile from source.  

1. Ensure Rust is installed and you're using the latest nightly.
2. Clone this repo and run `cargo install`.

If there's enough interest, I may start including releases and/or release a Homebrew formula.

This might work on Windows, but it's untested.  I'd stay away to avoid potential data loss.

## Usage
There are four main functions within Diecast: `new`, `load`, `delete`, and `list`.  

Templates are stored in `~/.diecast`.  They have no Special Sauce™ associated with them, so you can create, edit, and delete a template directly from this directory if you so choose.

You can get this info by running `dt` without any args, or with `-h` <sup>(dc was already taken)</sup>.

### New
`dt new <language> <name>`

`new` creates a new template and saves the contents of the current directory to it.

### Load
`dt load <language> <name>`

`load` replaces the contents of the current directory with the specified template.  Again, *this will remove the entire contents of the current directory.  This cannot be undone.*

### Delete
`dt delete <language> <name>`

`delete` removes a template.  *This will destroy a template, removing its files.  This cannot be undone.*

### List
`dt list <language (optional)>`

`list` displays existing templates for a given language.  If no language is specified, all templates for all languages will be shown.

## Contributing
If you want to contribute, don't hesitate to create an issue or PR! Since this project is in it's infancy, your input could help shape the project as a whole.

Bug reports and pull requests are welcome on GitHub at https://github.com/kieraneglin/diecast/. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the Contributor Covenant code of conduct.

## License
### MIT
Copyright 2017

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

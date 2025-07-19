# Teal Extension for Zed

A Zed extension that provides language support for [Teal](https://github.com/teal-language/tl).

## Features

- [x] File Association: Automatically recognizes `.tl` files as Teal
- [ ] File Icons
- [x] Syntax Highlighting based on the [VSCode Teal Extension](https://github.com/teal-language/vscode-teal)
- [x] Tree-sitter Integration using [tree-sitter-teal](https://github.com/euclidianAce/tree-sitter-teal)
- [ ] LSP support via [Teal Language Server](https://github.com/teal-language/teal-language-server)
- [ ] Auto Installation of the `teal-language-server`
- [ ] Formatting?

## Language Server Support

The LSP support isn't perfect yet, the original VSCode extension didn't have it, so this part has to be done from scratch.
Currently, there is instances where it just crashes while not saying why.
The diagnostics are also not properly formatted, but still useful.

Currently the `teal-language-server` has to be installed separately, the easiest way is via luarocks:
```sh
luarocks install teal-language-server
```
I will look at the Lua extention to see how to install it automatically soon.

## Formatting

There is literally no formatter that supports Teal out there and i don't see myself writing one by hand right now.
If i feel like it at some point, i will look at stylua to check if teal support is somewhat easy and will probably just tell claude to do it.
If it works. i will try to also make it work with helix, for those who are interested.

The [tealfmt](https://github.com/joe-p/tealfmt) project is a formatter for a different Teal, not this one.

## Snippets

While Zed extensions don't currently support custom snippets, you can add create custom Snippets separately. Check the [docs](https://zed.dev/docs/snippets) for more information.

## Configuration

There are no configurations because the LSP doesn't have any.

## Sources

- [Teal Language](https://github.com/teal-language/tl) - The Teal compiler and language
- [tree-sitter-teal](https://github.com/euclidianAce/tree-sitter-teal) - Tree-sitter grammar for Teal
- [VSCode Teal Extension](https://github.com/teal-language/vscode-teal) - The original VSCode extension

## Thoughts

This extension is not a passion fuelled project like my other ones, i needed syntax highlighting so i quickly created an extension mostly using ai. Then i thought adding LSP wouldn't be so hard...
The tooling for teal is pretty poor, as mentioned earlier there's not even a formatter.
I will keep an eye on it, but don't expect any big changes or innovations here.

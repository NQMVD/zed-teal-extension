# Teal Extension for Zed

A Zed extension that provides language support for [Teal](https://github.com/teal-language/tl), a typed dialect of Lua.

## Features

- **Syntax Highlighting**: Full syntax highlighting for Teal language constructs including:
  - Keywords (`local`, `global`, `record`, `interface`, `enum`, etc.)
  - Types and type annotations
  - Functions and methods
  - Comments and strings
  - Operators and punctuation

- **Tree-sitter Integration**: Uses the official [tree-sitter-teal](https://github.com/euclidianAce/tree-sitter-teal) grammar for accurate parsing

- **Code Structure**: 
  - Bracket matching for `()`, `[]`, `{}`, and quotes
  - Auto-indentation for functions, control structures, and blocks
  - Code outline showing functions, records, interfaces, and enums

- **File Association**: Automatically recognizes `.tl` files as Teal

## Installation

### From Zed Extensions

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "Extensions" and select "Extensions: Install Extensions"
4. Search for "Teal" and install

### Development Installation

1. Clone this repository
2. Open Zed
3. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
4. Type "Extensions" and select "Extensions: Install Dev Extension"
5. Select the cloned directory

## Requirements

For full Teal development experience, install the Teal compiler:

```bash
luarocks install tl
```

## Language Server Support

This extension includes full Language Server Protocol (LSP) integration with the [Teal Language Server](https://github.com/teal-language/teal-language-server), providing:

- **IntelliSense**: Auto-completion for variables, functions, and types
- **Diagnostics**: Real-time error checking and type validation
- **Go-to-Definition**: Navigate to function and type definitions
- **Hover Information**: Type information on hover
- **Signature Help**: Function parameter hints

### Language Server Installation

The extension will automatically detect and use the Teal language server if available. Install it via:

```bash
# Via LuaRocks (recommended)
luarocks install teal-language-server

# Or build from source
git clone https://github.com/teal-language/teal-language-server
cd teal-language-server
./scripts/setup_local_luarocks.sh
./scripts/generate_lua.sh
```

The extension will look for the language server in the following order:
1. `teal-language-server` in PATH
2. Local project installation at `teal-language-server/bin/teal-language-server`
3. Local lua_modules installation
4. Falls back to Lua language server if available

## Snippets

While Zed extensions don't currently support custom snippets, you can add Teal snippets to your global Zed configuration. Add these to your `snippets.json`:

```json
{
  "teal": {
    "req": {
      "prefix": "req",
      "body": "local ${1:name} = require(\"${2:module}\")",
      "description": "Local require"
    },
    "lrec": {
      "prefix": "lrec", 
      "body": "local record ${1:name}\n\t$0\nend",
      "description": "Local record definition"
    },
    "grec": {
      "prefix": "grec",
      "body": "global record ${1:name}\n\t$0\nend", 
      "description": "Global record definition"
    },
    "lenu": {
      "prefix": "lenu",
      "body": "local enum ${1:name}\n\t\"$0\"\nend",
      "description": "Local enum definition"
    },
    "genu": {
      "prefix": "genu",
      "body": "global enum ${1:name}\n\t\"$0\"\nend",
      "description": "Global enum definition"
    }
  }
}
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This extension is released under the MIT License.

## Related Projects

- [Teal Language](https://github.com/teal-language/tl) - The Teal compiler and language
- [tree-sitter-teal](https://github.com/euclidianAce/tree-sitter-teal) - Tree-sitter grammar for Teal
- [VSCode Teal Extension](https://github.com/teal-language/vscode-teal) - The original VSCode extension
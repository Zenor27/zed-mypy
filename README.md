# Zed ♥️ Mypy

This extension provides `mypy`, a Python typechecker, for the Zed editor.

It uses [pygls](https://github.com/openlawlibrary/pygls/) as a simple embedded LSP server.

## Enable

To enable the extension for Python files, add the following to your `settings.json`

```json
{
  "languages": {
    "Python": {
      "language_servers": ["mypy"]
    }
  }
}
```

## Configure

This extension will look for `mypy` in your path, but you may configure a custom path in the Zed `settings.json`.

You may also pass arguments to the `mypy` command.

```json
{
  "lsp": {
    "mypy": {
      "settings": {
        "path": "/foo/bar/.venv/bin/mypy",
        "args": ["--config-file", "mypy.ini", ...]
      },
    },
  },
}
```

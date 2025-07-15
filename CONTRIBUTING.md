# Contributing to mdsf

Thank you for considering contributing to mdsf. It is greatly appreciated! ❤️

## Table of contents

<!-- START_SECTION:toc -->

- [Table of contents](#table-of-contents)
- [Adding support for a new tool](#adding-support-for-a-new-tool)
  - [Tool metadata](#tool-metadata)

<!-- END_SECTION:toc -->

## Adding support for a new tool

Adding support for new tools is extremely easy, and does not require any knowledge of Rust programming.

To add support for a new tool start by creating a folder in [`tools`](tools/) with the name of the tool.

Inside the folder you create a new file called `plugin.json`.

The file must include _at least_ 2 keys, `binary`, which, as the key suggest, is the name of the binary, and `commands`, which is a nested map of all the commands for the tool.

If we wanted to add support for mdsf (`mdsf format PATH_TO_FILE`) that would look like the following:

```json
{
  "$schema": "../tool.schema.json",
  "binary": "mdsf",
  "commands": {
    "format": {
      "arguments": ["format", "$PATH"]
    }
  }
}
```

As seen in the example above we used the argument `$PATH`. That is a special keyword that is automatically replaced with the actual path to the file.

If you wish to test the tool locally you can generate the code for the new tool by running `mise codegen`. Otherwise the code will automatically be generated when the `plugin.json` files is committed.

### Tool metadata

`plugin.json` also supports some metadata keys that are used when generating the documentation for the tool.

The metadata fields does **not** affect mdsf when it is ran.

The current metadata keys are:

| Key           | Datatype   | Description                                                |
| ------------- | ---------- | ---------------------------------------------------------- |
| `description` | `string`   | Description of what the tool does.                         |
| `homepage`    | `string`   | Link to the repository/homepage/documentation of the tool. |
| `categories`  | `string[]` | The purpose of the tool (`formatter` or `linter`).         |
| `languages`   | `string[]` | The languages/file types the tool supports.                |

`tools/mdsf/plugin.json`

```json
{
  "$schema": "../tool.schema.json",
  "binary": "mdsf",
  "commands": {
    "format": {
      "arguments": ["format", "$PATH"]
    }
  },
  "description": "mdsf is a markdown codeblock formatter and linter",
  "homepage": "https://github.com/hougesen/mdsf",
  "languages": ["markdown"],
  "categories": ["linter", "formatter"]
}
```

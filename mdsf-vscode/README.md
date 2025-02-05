# mdsf Visual Studio Code Extension

mdsf is a code formatter for markdown files that allows you run execute ordinary formatters like rustfmt, gofmt and stylua on markdown code blocks.

## Usage

This extension requires mdsf to already be installed . That can be done by following the [installation guide](https://github.com/hougesen/mdsf?tab=readme-ov-file#installation).

### Setting mdsf as default formatter

```jsonc
{
  // settings.json
  // ...
  "[markdown]": {
    "editor.defaultFormatter": "hougesen.mdsf"
  }
  // ...
}
```

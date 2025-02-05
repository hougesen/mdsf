# mdsf Visual Studio Code Extension

[![](https://img.shields.io/visual-studio-marketplace/v/hougesen.mdsf?color=374151&label=Visual%20Studio%20Marketplace&labelColor=000&logo=visual-studio-code&logoColor=0098FF)](https://marketplace.visualstudio.com/items?itemName=hougesen.mdsf)
[![](https://img.shields.io/visual-studio-marketplace/v/hougesen.mdsf?color=374151&label=Open%20VSX%20Registry&labelColor=000&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&logoColor=0098FF)](https://open-vsx.org/extension/hougesen/mdsf)

mdsf is a code formatter for markdown files that allows you run execute ordinary formatters like rustfmt, gofmt and stylua on markdown code blocks.

## Usage

This extension requires mdsf to already be installed . That can be done by following the [installation guide](https://github.com/hougesen/mdsf?tab=readme-ov-file#installation).

### Setting mdsf as default formatter

```json
{
  // settings.json
  // ...
  "[markdown]": {
    "editor.defaultFormatter": "hougesen.mdsf"
  }
  // ...
}
```

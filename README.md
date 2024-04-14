# Reword.app to Anki

Convert word lists from [reword.app](https://reword.app) to Anki decks.

## How

- Download Reword's apk, from which you want to import words (e.g. from [apkpure.com](https://apkpure.com/developer/POAS%20Apps)).
- Install and run this tool, as described below.

## Installation

```sh
cargo install --git=https://github.com/istudyatuni/rewordapp2anki.git
```

Then you can run it as `rewordapp2anki`.

## Exporting

Categories are saved as tags, so you can export all words at once, and then create filtered decks for specific categories.

When specific categories are selected, words that are in those categories are exported, even if those words occur in other categories. Also, category tags that are not selected will not be removed from the exported words.

## Supported apps

Not all applications are currently supported. After launching this application, you will see a list of supported applications.

## Development

Pre-commit hook

```sh
git config core.hooksPath .githooks
```

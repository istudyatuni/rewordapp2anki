# Reword.app to Anki

Convert word lists from [reword.app](https://reword.app) to Anki decks.

## How

- Download Reword's apk, from which you want to import words (e.g. from [apkpure.com](https://apkpure.com/developer/POAS%20Apps)).
- Install and run this tool, as described below.
- If you want to convert custom categories, you don't need an apk file, see [below](#exporting-custom-categories) for usage.

### Note on .XAPK files

XAPK files are just ZIP archives, and required APK file is inside it. Open XAPK file, search inside for APK file with name like `ru.poas.[some name].apk`, extract it, and then select extracted APK when asked.

## Installation and usage

You can download pre-built binaries from [latest release](https://github.com/istudyatuni/rewordapp2anki/releases/latest). Unpack downloaded archive, then:

- Linux: run as `./rewordapp2anki`
- Windows: run from the file explorer

## Exporting

### Exporting built-in categories

Select "APK file" in first question.

Categories are saved as tags, so you can export all words at once, and then create filtered decks for specific categories.

When specific categories are selected, words that are in those categories are exported, even if those words occur in other categories. Also, category tags that are not selected will not be removed from the exported words.

### Exporting custom categories

Select "Custom categories" in first question.

To convert custom categories you need to "Share" if from the app, and then select these `.reword` files when asked. Select as many files as you want, then press `Esc` to continue.

Select a directory to add all `.reword` files from this directory. To select current directory, enter `.`

## Supported apps

Not all applications are currently supported. After launching this application, you will see a list of supported applications.

## Development

Pre-commit hook

```sh
git config core.hooksPath .githooks
```

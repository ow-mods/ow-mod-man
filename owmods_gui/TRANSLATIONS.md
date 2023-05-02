# Translation Guide

Thank you for taking the time to write a translation! Below you'll find a guide on how to create one.

## Grab the template

To get started, grab [the template.json file](https://github.com/Bwc9876/ow-mod-man/blob/main/owmods_gui/frontend/src/assets/translations/template.json).

In addition, you may want to keep [english.json](https://github.com/Bwc9876/ow-mod-man/blob/main/owmods_gui/frontend/src/assets/translations/english.json) open to get an idea of what each key is for.

## How translations work

Translations are fairly straightforward, they're represented by a JSON object where each key is the key and each value is the translation for said key. For example if I had the `APP_NAME` key:

```json
{
    "APP_NAME": "Outer Wilds Mod Manager"
}
```

The value on the right of the `:` inside of the quotation marks is what will be displayed when your language is selected

### Variables

Translations also have a special syntax for variables, `$name$`. **the words in between the dollar signs should not be translated**.

The manager will insert variables during runtime, you just need to tell it where.

For example the `VERSION` key has one variable name `version` that the manager inserts:

```json
{
    "VERSION": "Version: $version$"
}
```

At runtime `$version$` will be replaced with `1.2.3` or whatever the current version is. To get an idea for what variables a key can have see `english.json`.

### The _ Key

The `_` key is a bit special, this key is used in the event that a translation cannot be found. It will be passed the variable `$fallback$` which will have the english translation as a fallback. It will also have `$key$`, which will be the key we were trying to translate, useful for debugging. Try to put some sort of message in this key so the user knows to report it in the event a key is missing. For example in english:

```json
{
    "_": "Missing $key$: $fallback$"
}
```

## Submitting Your Translation

After you're done translating, you can submit your translation by [making a new issue](https://github.com/Bwc9876/ow-mod-man/issues/new/choose) and choosing the "Translation" option.

To update a translation please do the same.

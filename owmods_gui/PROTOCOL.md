# Install URIs and the owmods protocol

This document describes the owmods protocol and how to use it to install mods from other applications.

## General Structure

`owmods://install-type/payload`

All URLs should start with owmods://
Then they should follow with the install type they want like `install-mod` or `install-url`
Finally they should have the payload for the install

## Install Types

- `install-mod` - Installs a mod from the mods database, the payload should be the mod unique name
- `install-url` - Installs a mod from a url, the payload should be the url to install from, **Not URI encoded**
- `install-zip` - Installs a mod from a zip file, the payload should be the path to the zip file, note you shouldn't really need to use this because every user's computer is different, this is just used internally for drag and drop
- `install-prerelease` - Installs a mod from a prerelease (in the mods database), the payload should be the mod unique name

## Examples

- owmods://install-mod/Bwc9876.TimeSaver
- owmods://install-url/<https://example.com/Mod.zip>
- owmods://install-zip//home/user/Downloads/Mod.zip
- owmods://install-prerelease/Raicuparta.NomaiVR

## Notes

- The payload should **NOT** be URI encoded, it should be just the raw payload
- In the case of `install-url` the url should be a direct download link, not a page with a download button
- In the future this might become more advanced and have query params and stuff. If this happens `install-prerelease` will be changed to `install-mod` with a query param, however this will be backwards compatible
- Note that users have to open the mod manager at least once before this will work, this is because the protocol is registered when the mod manager is opened.

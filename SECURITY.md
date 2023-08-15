# Security Policy

## Supported Versions

Any version of the manager we officially distribute (see [Publishing](https://github.com/ow-mods/ow-mod-man/blob/main/ARCHITECTURE.md#publishing)) is supported. We scan for outdated dependencies and dependabot alerts to circumvent upstream threats. As well as scan our own code with CodeQL.

Versions with the auto-updater (Windows (msi and nsis), AppImage) are signed with a cryptographic key that makes all versions of the manager refuse to install an update from an untrusted source.

## Reporting a Vulnerability

Please report security issues by adding an issue or by contacting `bwc9876@outerwildsmods.com` if the issue is sensitive.

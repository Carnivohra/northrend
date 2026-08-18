# Northrend

Northrend is a lightweight, from-scratch reimplementation of the original World of Warcraft® game engine, written in Rust. The goal is to recreate the original engine on a portable platform that can be easily extended through independent game plugins.

Northrend is provided as a library rather than a standalone executable. Games supply their own implementation and content, then start the engine from their own executable.

>Northrend is experimental and under active development. Its APIs, architecture, and supported formats may change substantially.

## Goals

- A small and modular engine architecture.
- High performance with minimal runtime overhead.
- Platform-independent window, input, rendering, audio, physics, and networking contracts.
- Efficient streaming and support for game data formats used by different World of Warcraft versions.
- A game-facing API through which individual game projects define their assets, worlds, entities, gameplay, cameras, shaders, and other content.
- Faithful reproduction of relevant original engine behavior without using original implementation code.

## Disclaimer

Northrend is independently written from scratch and does not rely on decompilation, disassembly, or access to Blizzard's implementation. It contains no Blizzard code, binaries, assets, leaked material, credentials, encryption keys, or circumvention tools.

File format support is provided by original parsers. Users must supply their own legally obtained game data, and contributions derived from proprietary or unlawfully obtained material are not accepted.

Northrend is an independent, unofficial project. It is not affiliated with, authorized by, sponsored by, or endorsed by Blizzard Entertainment, Inc., Microsoft Corporation, or any of their affiliates.

World of Warcraft®, Warcraft®, Blizzard Entertainment®, and related names and logos are trademarks or registered trademarks of Blizzard Entertainment, Inc. and/or their respective rights holders. References to them are made solely to describe compatibility goals and historical engine behavior; no ownership, license, endorsement, or other relationship is claimed or implied.

The project is not intended to impersonate an official product, connect to Blizzard services without authorization, bypass access controls, facilitate cheating, or interfere with any official game or service.

See Blizzard Entertainment's official [logo and trademark guidelines](https://www.blizzard.com/en-us/legal/8bcb0794-6641-4ce3-a573-8eb243bab342/blizzard-entertainment-logo-and-trademark-guidelines) and [legal information](https://www.blizzard.com/legal/).

## License

Northrend is licensed under the [Mozilla Public License, Version 2.0](LICENSE).

Copyright © 2026 Carnivohra and Northrend contributors.

Copyright in the original Northrend source code and documentation remains with its respective contributors. This license applies only to the original Northrend project material. It does not grant any rights to Blizzard intellectual property, trademarks, game data, code, or assets.

This notice is provided for clarity and is not legal advice.

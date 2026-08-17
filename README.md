# Northrend

Northrend is an independent, lightweight game engine written from scratch in Rust. Its long-term technical goal is to recreate the architecture and behavior of the original World of Warcraft® game engine while remaining modular, efficient, and portable.

The engine is built directly on low-level libraries such as `winit` and `wgpu`. It does not use an existing game engine. Games integrate Northrend as a library, provide their own game implementation, select the assets they need, and start the engine from their own executable.

## Status

Northrend is experimental and under active development. Its APIs, architecture, and supported formats may change substantially.

## Goals

- A small and modular engine architecture.
- High performance with minimal runtime overhead.
- Platform-independent window, input, rendering, audio, physics, and networking contracts.
- Efficient streaming and support for game data formats used by different World of Warcraft versions.
- A game-facing API through which individual game projects define their assets, worlds, entities, gameplay, cameras, shaders, and other content.
- Faithful reproduction of relevant original engine behavior without using original implementation code.

## Independent implementation

Northrend is an original, independently written implementation. It is not a reverse-engineering project and does not rely on decompilation, disassembly, modification, translation, or access to Blizzard's source or binary implementation.

The project does not contain, incorporate, or distribute:

- Blizzard source code or leaked proprietary material;
- World of Warcraft client or server binaries;
- original game assets, data files, artwork, models, textures, audio, music, cinematics, or text;
- authentication credentials, encryption keys, or circumvention tools.

Support for a file format means that Northrend provides an independently written parser or loader. No Blizzard game data is bundled with this repository. Users are solely responsible for obtaining and using any external data legally and in accordance with all applicable agreements and laws.

Contributions copied or derived from leaked source code, proprietary Blizzard materials, or other unlawfully obtained material are not accepted.

## Non-affiliation

Northrend is an unofficial project. It is not affiliated with, authorized by, maintained by, sponsored by, or endorsed by Blizzard Entertainment, Inc., Microsoft Corporation, or any of their affiliates.

The project is not intended to impersonate an official Blizzard product, connect to Blizzard services without authorization, bypass access controls or technological protection measures, facilitate cheating, or interfere with any official game or service.

## Trademarks and intellectual property

World of Warcraft®, Warcraft®, Blizzard Entertainment®, and all related names, logos, characters, locations, story elements, game data, and other intellectual property are owned by Blizzard Entertainment, Inc. and/or their respective rights holders. All rights in those materials are reserved by their respective owners.

World of Warcraft and Warcraft are trademarks or registered trademarks of Blizzard Entertainment, Inc. in the United States and/or other countries. Blizzard Entertainment is a trademark or registered trademark of Blizzard Entertainment, Inc. in the United States and/or other countries.

References to Blizzard products are made solely to describe compatibility goals and the historical engine behavior being independently reimplemented. No trademark ownership, license, sponsorship, endorsement, or other relationship is claimed or implied.

See Blizzard Entertainment's official [logo and trademark guidelines](https://www.blizzard.com/en-us/legal/8bcb0794-6641-4ce3-a573-8eb243bab342/blizzard-entertainment-logo-and-trademark-guidelines) and [legal information](https://www.blizzard.com/legal/).

## Project copyright

Copyright in the original Northrend source code and documentation remains with its respective contributors. Unless a separate license explicitly states otherwise, all rights are reserved and no license to Blizzard intellectual property is granted by this repository.

This notice is provided for clarity and is not legal advice.

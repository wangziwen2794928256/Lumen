# Music · Phase 10

Music is a bundled Official Module (API 1.0) with a lazy route, Registry-controlled navigation and guarded IPC. Manual recording is always available on desktop: title, artist, album, platform, UTC play time, actual seconds and an optional existing journal link. Records support revision checks, idempotent requests, editing and soft trash/restore.

Windows media observation is off by default. Consent names one exact source application ID. The Kernel adapter checks that source before reading song metadata, initializes WinRT in a dedicated MTA thread and times out asynchronous requests after two seconds. Missing session/metadata or permission errors leave manual entry usable. No Spotify/OAuth, network calls or third-party execution are included.

The conservative tracker counts advancing observed playback only. Pauses, seeks and offline gaps do not add elapsed time. An episode contributes one play once the configurable 5–600 second threshold is reached, then updates duration every five seconds. Known ad titles are rejected; unlabelled ads cannot reliably be identified from this API. The last partial update may undercount. Multiple simultaneous players and pre-launch history are not reconstructed.

Song, Artist and Album clouds remain separate from journal words. Date-filtered totals include all live records; lists show 100 latest plays and cloud queries return up to 500 groups (50 displayed). Data is private in core SQLite via additive migration009; no production fixtures are seeded. See [data contract](../data/MUSIC.md), [platform support](INSTALLATION.md) and [Chinese](../zh-CN/MUSIC.md). Original ADR-010-media remains in the owner's local decision history.

Module switching now keeps the application shell mounted during lazy loading; hover/focus preloads code, and short opacity transitions honor reduced motion. Today and settings forms use explicit spacing. These changes retain ordinary page unmount semantics for autosave and listeners.

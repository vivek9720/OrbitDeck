# OrbitDeck

OrbitDeck is a Rust library for decoding disconnected satellite ground-station
coordination bundles. The imagined system is used by regional emergency teams
that operate portable antennas after storms, fires, or earthquakes, when normal
network access is unavailable and contact plans must be exchanged as signed
binary files or chunked radio streams.

The library parses a multi-section OrbitDeck bundle containing a string pool,
ground-station inventory, antenna contact windows, orbit arcs, telemetry frames,
event journals, catalog hints, and a compact policy bytecode program. After
decoding, OrbitDeck scores the contact plan, replays telemetry and event logs,
runs command-policy scripts, and produces a scheduling report with findings.
The parser is intentionally layered: later stages depend on handles, aliases,
and state derived from earlier sections, which gives fuzzing real structured
state to explore instead of a one-header format.

The repository includes cargo-fuzz style harnesses, per-target seed corpora, a
fuzzing dictionary, and a ClusterFuzzLite build script that relies only on local
path dependencies so it can build offline.

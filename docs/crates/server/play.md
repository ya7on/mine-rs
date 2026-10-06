# Connection-only Play

Play initializes a spectator in overworld at (8, 100, 8). It sends Login,
absolute position and zero velocity, spawn position, Start waiting for chunks,
cache center (0, 0), and one chunk enclosed by batch start/finish packets.
A matching teleport confirmation must echo ID, position and rotation. Other
bounded Play frames are discarded: no movement, interaction or world simulation
is implemented. Currently the connection closes after this confirmation.

The chunk is all air across overworld's 24 sections (-64 through 319).
Three client heightmaps contain zero relative heights; skylight is full across
26 light layers, block light is empty. A section includes both block and fluid
counts in 26.3. Single-value palettes have no long-array length prefix.
These layouts were verified against Mojang's official 26.3 server classes
LevelChunkSection, PalettedContainer.Data and ClientboundLevelChunkPacketData.

Air is static block-state ID 0, verified from the official blocks report.
The void biome is ID 59 in our exported registry order, overworld dimension
is ID 0. These assumptions must be updated together with registry exports.
Only the initial chunk is sent, with no persistence or later streaming.

Loopback tests verify the entire sequence, section counts, palette IDs,
heightmaps and light masks. Real-client loading remains an external check.

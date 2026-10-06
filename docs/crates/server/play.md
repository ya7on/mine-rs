# Connection-only Play

Play initializes a spectator in overworld at (8, 100, 8). It sends Login,
absolute position and zero velocity, spawn position, Start waiting for chunks,
cache center (0, 0), and one chunk enclosed by batch start/finish packets.
A matching teleport confirmation must echo ID, position and rotation. Other
bounded Play frames are discarded: no movement, interaction or world simulation
is implemented. The connection stays open after confirmation. It closes on EOF, malformed
required replies or an expired deadline.

The chunk is all air across overworld's 24 sections (-64 through 319).
Three client heightmaps contain zero relative heights; skylight is full across
26 light layers, block light is empty. Each mask is a VarInt-prefixed byte
array, with little-endian bit ordering; the 26-layer mask is `04 ff ff ff 03`.
The older long-array BitSet encoding causes light-array decoding to misalign. A section includes both block and fluid
counts in 26.3. Single-value palettes have no long-array length prefix.
These layouts were verified against Mojang's official 26.3 server classes
LevelChunkSection, PalettedContainer.Data and ClientboundLevelChunkPacketData.

Air is static block-state ID 0, verified from the official blocks report.
The void biome is ID 59 in our exported registry order, overworld dimension
is ID 0. These assumptions must be updated together with registry exports.
Only the initial chunk is sent, with no persistence or later streaming.

Loopback tests verify the entire sequence, section counts, palette IDs,
heightmaps and light masks. After correcting the mask encoding, an actual
packet captured from the loopback server was decoded completely with the
official 26.3 ClientboundLevelChunkWithLightPacket.STREAM_CODEC and re-encoded
byte-for-byte identically; all 26 sky arrays had length 2048. This verifies
packet wire compatibility, not completion of the client's loading screen.
Real-client loading remains an external check.

## Connection maintenance

The server sends an initial Keep Alive immediately after the chunk, then one
every ten seconds (measured from the preceding send). Only one request may be
outstanding. Its reply must echo the current ID and consume the whole body;
an unexpected ID or unsolicited reply closes the connection. The reply
must arrive within fifteen seconds. Late replies cause the next request to be
sent immediately when its ten-second schedule has already passed.
Teleport confirmation also has a fifteen-second deadline, independent of
Keep Alive replies and other client traffic.

Play borrows the socket's read/write halves inside the existing connection
task. A complete-frame read has a thirty-second deadline and remains pinned
across timer events, so partially received frames survive outbound Keep Alive.
There are no extra tasks, queues or shared mutable connection state.

Loopback tests cover startup and Keep Alive echo/rejection. In-memory duplex
tests with virtual time cover periodic requests across a partial frame,
non-resetting deadlines, trailing reply bytes and invalid teleport echoes.

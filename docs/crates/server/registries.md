# Minimal Configuration experiment

This server targets the vanilla 26.3 client (protocol 777). Registry values
are sourced from the client's `minecraft:core` pack after an exact Known
Packs match. There is no fallback to inline NBT for other pack versions.

`crates/server/data/registries-777.json` is a fixed experiment, not a full
vanilla registry database. It lists 32 synchronized registries and 18
selected entries. All other registries are sent with empty entry lists.
Each nonempty registry contains one entry, so its numeric ID is zero.
Future Play packets must respect these IDs, not vanilla ordering.

The selected biome is `the_void`, the dimension is `overworld`, the damage
type is `generic`. Default animal/sound variants and a painting are retained
because the client requires nonempty variant registries even without entity
gameplay. Variants use unconditional spawn rules rather than references to
omitted biomes. The overworld dimension references the overworld clock and
the `in_overworld` timeline tag; these are supplied with the day timeline.
It also references the static block tag `infiniburn_overworld`. This tag
is supplied with an empty entry list: the client requires the tag's presence
to decode the dimension, while infinite-burning block behavior is outside
this connection-only experiment. No vanilla block numeric IDs are assumed.
This setup does not describe playable world content or create chunks.

Source: Mojang's official 26.3 client archive, SHA-1
`e877b6a07acd633fb3bb475002175cec036e7b87`, whose `version.json` declares
protocol 777. Registry names come from `RegistryDataLoader`'s
`SYNCHRONIZED_REGISTRIES`; entry identifiers and dependencies were checked
against the archive's vanilla JSON data. The archive is not vendored.

Configuration tolerates Client Information and plugin payloads between
required responses. It sends registry listings, vanilla Feature Flags,
the timeline Update Tags, and Finish Configuration, then waits for an
empty acknowledgement. The coordinator closes at the Play boundary until
Play is implemented. An unmatched core pack receives a Configuration
Disconnect using network NBT.

For protocol 777, clientbound Update Tags is `0x0E`; `0x0C` is Transfer.
After receiving Finish Configuration acknowledgement, the server logs
`Configuration acknowledged by client`. A vanilla client reaching this
message proves it accepted Configuration, even though Play is not implemented.

Loopback integration tests prove the wire sequence, registry counts,
omitted NBT, tag ID and rejection path. They do **not** prove that the
vanilla client accepts this reduced set. That requires a real 26.3 client
run, which has not been performed. Required tags or entries may need to be
added after that check. Do not call this set a proven protocol minimum.

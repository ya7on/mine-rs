# Vanilla Configuration registries

The server targets vanilla 26.3 (protocol 777). It sends all 32 synchronized
registries, containing 432 entries, and 773 fully resolved tags across 15
registries. Tags contain vanilla members instead of empty placeholders.

Registry values are sourced from the client's `minecraft:core` pack after an
exact Known Packs match. Registry Data omits inline NBT. A different pack
selection receives a Configuration Disconnect; there is no inline fallback.

Dynamic IDs follow the entry order in `registries-777.json`. Dynamic tags use
this same order. Static IDs (blocks, items, etc.) come from the official server
registry report. Future Play packets must respect these dynamic IDs rather
than assume vanilla ordering. Nested tags are expanded and deduplicated;
missing mandatory references and cycles fail export. Optional missing
references are skipped.

## Sources and reproduction

Inputs are Mojang's official 26.3 client archive, SHA-1
`e877b6a07acd633fb3bb475002175cec036e7b87`, and the registry report generated
from the official server archive, SHA-1
`33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c`. Registry names come from
`RegistryDataLoader.SYNCHRONIZED_REGISTRIES`. Archives are not vendored.

Generate the report in a temporary directory using Java 25:

```sh
java -DbundlerMainClass=net.minecraft.data.Main -jar /path/to/server.jar --reports --output /path/to/generated
```

Regenerate the checked-in data from the repository root:

```sh
python3 tools/export_registries.py --client /path/to/client.jar --registry-report /path/to/generated/reports/registries.json --output crates/server/data
```

The exporter verifies the client version, protocol and checksum, and pins the
report's SHA-256. It performs no downloads or server startup. Generated data
contains identifiers and tag membership, not registry NBT.

## Lifecycle and verification

Configuration tolerates Client Information and plugin payloads between required
responses. It sends registry listings, vanilla Feature Flags, Update Tags
(`0x0E` for protocol 777), and Finish Configuration, then waits for an empty
acknowledgement. Play now initializes a spectator and closes after teleport confirmation.

Loopback integration tests verify the wire sequence, dataset counts, omitted
NBT, representative vanilla tag membership and pack rejection. They do not
execute the vanilla client's registry loader. A real 26.3 client reaching
`Configuration acknowledged by client` confirms acceptance of Configuration;
this revised full dataset still requires that check. Play sends one empty chunk; connection maintenance is the next step.

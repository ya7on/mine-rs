# mclib

`mclib` contains shared Minecraft protocol types and codecs. Its packet layout
follows the earlier `ya7on/mclib` design: directions are modules, individual
packets have individual source files, and packet fields use explicit Minecraft
types such as `MCString` and `MCLong`.

Packet bodies live under `mclib::packets` and are grouped by protocol state,
then by direction when a state contains packets in both directions. The
`handshaking` state implements the serverbound `Handshake` packet, and the
`status` state is documented below.

Packets implement `MCType`, so `pack()` and `unpack()` operate on their fields in
protocol order. For packets these methods process the body only. Transport
framing owns the packet-length and packet-ID VarInts; keeping those concerns
separate lets a body codec be reused with compressed and uncompressed frames.

## Packet framing

`PacketFrame` reads and writes the uncompressed wire representation:

1. packet length as a VarInt of at most three bytes;
2. packet ID as a VarInt;
3. the already encoded packet body.

It enforces the protocol maximum of 2,097,151 bytes for the packet ID and body
combined. Compression is outside this type and can be introduced as a separate
framing layer when login-state compression is implemented.

The constructor only stores the packet ID and body and returns `Self`.
`write` validates the packet ID and total size before writing any bytes and
computes the length from the encoded ID and body. `write` delegates encoding to `pack`; protocol validation is owned by
encoding, not construction.

The optional `tokio-io` feature enables async `read` and `write`, accepting
Tokio `AsyncRead` and `AsyncWrite`. No features are enabled by default;
`PacketFrame`, `pack`, and body codecs are available without Tokio.
`pack` remains synchronous and validates and encodes the whole frame before
`write` sends any bytes. Body codecs remain synchronous and operate on memory.
Cancelling a partial frame read requires closing the stream; restarting that
read loses framing. With `tokio-io`, `mclib` depends on Tokio's I/O traits,
not a runtime.

## Status state

The status module implements the four protocol-777 status bodies documented in
`plans/wiki.md`:

- clientbound `StatusResponse` and `PongResponse`;
- serverbound `StatusRequest` and `PingRequest`.

The handshaking module implements the serverbound `Handshake` packet
(protocol version, server address, port, and intent enum), letting a caller
select the target protocol state.

Status strings enforce the protocol's limit of 32,767 UTF-16 code units
and the related UTF-8 byte limit of three times that count. Supplementary
characters such as emoji count as two UTF-16 units. Long timestamps use Minecraft's signed, big-endian 64-bit
representation. Decoders report malformed and incomplete input as errors.

## Login state

The login module implements serverbound `LoginStart` for
protocol 777 (Login packet ID `0x00`), following `plans/wiki.md`:
a username limited to 16 UTF-16 code units, followed by a 16-byte UUID.
The UUID is a client claim, not authenticated identity. Username policy and
offline identity selection belong to the server. This body codec does not
control server connection handling.

Serverbound `LoginAcknowledged` (Login packet ID `0x03`) has an empty
body. The client sends it after Login Success; receiving it allows the
server to proceed to Configuration. Checking packet order and complete body
consumption belongs to the server, as with other packet body codecs.

Clientbound `LoginSuccess` (Login packet ID `0x02`) encodes a `GameProfile`
followed by a separate session UUID, as specified for protocol 777. This
layout must not be reused for older protocol versions. `GameProfile` has
a UUID, a 16-unit username and at most 16 properties. Property names are
limited to 64 UTF-16 units, values to 32767, and optional signatures to 1024.

Clientbound `Disconnect` (Login packet ID `0x00`) carries a JSON text
component in a protocol string. Configuration and Play disconnect reasons
use network NBT instead; the Login codec cannot be reused for those states.
As with Status JSON, the codec validates string encoding and bounds, while
the caller owns the JSON document's content.

## Field types

The `types` module provides the wire types used by packet bodies. Every type
implements `MCType` and reports malformed input as errors instead of panicking:

- fixed-width integers `MCByte`, `MCUnsignedByte`, `MCShort`, `MCUnsignedShort`,
  `MCInt`, `MCLong` and IEEE 754 `MCFloat`, `MCDouble` (big-endian);
- `MCVarInt` and `MCVarLong`, bounded at five and ten bytes;
- `MCBoolean`, `MCString`, `MCByteArray`, and the generic `MCPrefixedArray`;
- `MCUuid`, `MCPosition` (packed 26/26/12-bit), and `MCBitSet`;
- Java Edition network NBT in `mclib::nbt`, exposing the `Nbt` value tree
  with a root tag ID and payload, without a root name.

All types are re-exported from the crate root, which is the single import
point for consumers. The shared error type (`ProtocolError`) lives in
`error.rs` at the crate root and is re-exported the same way. `MCType` lives
in `codec.rs`: it is the encoding contract shared by types and packets, not a
wire type.

Length-prefixed types reuse `MCVarInt`'s `MCType` implementation directly.
VarInt/VarLong encoders shift unsigned wire bits; decoders accumulate into
the signed result, preserving two's-complement values without byte-array
reinterpretation. Collection and string bounds are checked before decoding
their contents.

## Network NBT

`Nbt::encode_network` and `Nbt::decode_network` handle one root tag, including
its type byte. Decoding has one entry point taking `&mut dyn Read`, including
byte slices via `Nbt::decode_network(&mut bytes.as_slice())`.
Any value represented by `Nbt` can be a root; `TAG_End` is a
compound terminator, not a standalone value in this API. Packet fields that
use it to mean absent NBT must handle that sentinel themselves. The codecs
target Java Edition network NBT since 1.20.2, not named file roots, compression
or Bedrock's VarInt format.

Private encoder and decoder structures hold the output buffer and input reader.
Each NBT type has a corresponding packing or unpacking method; a payload match
dispatches to those methods. Strings and compound names share the string methods. A compound stores
type, name and payload for each entry, ending with `TAG_End`. A list stores
one element type and a signed four-byte count, then payloads only; mixed
element types are rejected when encoding. Empty lists accept any element
type and nonpositive counts when decoding, and encode canonically with type
zero and count zero. Array counts are nonnegative signed four-byte integers.

Names and string values share a modified UTF-8 codec with an unsigned
two-byte byte count. NUL uses two bytes and supplementary characters use
UTF-16 surrogate pairs. Decoding requires canonical modified UTF-8 and rejects
unpaired surrogates, since Rust `String` cannot represent them. Empty names
are accepted. See the [NBT specification](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/NBT)
and [Java modified UTF-8 definition](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/io/DataInput.html#modified-utf-8).

Both directions limit nesting to 512 edges from the root. String buffers
are bounded by the `u16` length prefix. Collection buffers grow incrementally
instead of allocating the full declared count; truncated input reports an
I/O error. Decoding consumes one root, leaving subsequent stream data intact.

This corrects the earlier codec's missing root type and VarInt collection
counts. Bytes produced by that earlier implementation are incompatible with
the network format and are not preserved as a separate compatibility mode.

## Lints and tests

The workspace lint level (pedantic, nursery, `unwrap_used = deny`) applies to
production code; clippy is run without `--all-targets`, and tests are verified
by `cargo test` alone. Test code carries no clippy allow attributes.

## Error categories

All codecs, including NBT and packet framing, report `ProtocolError` through
three categories: `Io` preserves the underlying I/O error (including
`UnexpectedEof` for truncated input), `Overflow` covers exceeded numeric,
length, size, and nesting bounds, and `InvalidData` covers invalid values,
encodings, and structures. Errors do not identify individual wire types or
carry their values and limits; validation remains local to each codec.

Test both configurations with `cargo test -p mclib --no-default-features` and
`cargo test -p mclib --no-default-features --features tokio-io`.

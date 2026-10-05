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

## Status state

The status module implements the four protocol-777 status bodies documented in
`plans/wiki.md`:

- clientbound `StatusResponse` and `PongResponse`;
- serverbound `StatusRequest` and `PingRequest`.

The handshaking module implements the serverbound `Handshake` packet
(protocol version, server address, port, and intent enum), letting a caller
select the target protocol state.

Status strings enforce the protocol's 32,767-character limit and the related
UTF-8 byte limit. Long timestamps use Minecraft's signed, big-endian 64-bit
representation. Decoders report malformed and incomplete input as errors.

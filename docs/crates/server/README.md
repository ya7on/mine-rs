# server

`server` is a minimal Minecraft server binary supporting Status and offline
Login for protocol 777, with vanilla Configuration registries. Play initializes a spectator at (8, 100, 8) and verifies teleport confirmation.
Chunk loading and connection maintenance are subsequent steps.

## Scope and limitations

- Handshaking routes to Status or Login. Transfer remains unsupported.
- Login proceeds to experimental Configuration, requiring the client's
  exact vanilla 26.3 core pack. After Finish Configuration acknowledgement,
  Play sends Login and an absolute position, then closes after teleport confirmation. This is not yet a complete
  vanilla client connection. See [registries.md](registries.md) for the
  vanilla dataset and its verification limits.
- Only uncompressed packet framing (`mclib::PacketFrame`). No compression and
  no encryption, which matches the status stage of the protocol.
- The Legacy Server List Ping (`0xFE`) is not handled.
- I/O uses Tokio with one task per connection on a multithread runtime.
  Graceful shutdown and outbound message queues are not implemented.

## Architecture

- `config` parses the CLI (`--bind`, `--port`, `--status-json`) into a
  `Config`. Defaults listen on `0.0.0.0:25565` and serve a placeholder JSON
  document for protocol 777.
- `listener` binds the address and spawns one Tokio task per accepted
  connection. Failures on a single connection never stop the accept loop.
- `connection` owns the socket and drives phases through sequential async
  calls: Handshaking → Status → close. `connection/handshaking.rs` reads and
  validates the handshake packet and returns a typed `Status`, `Login`, or
  `Transfer` outcome. Unknown intents are protocol errors. `Connection::run`
  selects the handler from this outcome; phase handlers do not
  choose the next phase. `connection/status.rs` optionally answers a status
  request, then echoes the ping, without separate waiting states. It returns
  `StatusAndPing` or `PingOnly`; both exchanges end the connection.
  After Status returns successfully, `Connection::run` explicitly shuts down
  the socket's write side; dropping the connection releases the socket.
  On errors the caller must drop the connection, as the listener does.
  `connection/login.rs` reads Login Start, derives an offline profile, sends
  Login Success and waits for Login Acknowledged. It returns the profile to
  the coordinator, or `None` after a controlled Login rejection.
  `connection/configuration.rs` negotiates Known Packs and sends the fixed
  vanilla registry set before Finish Configuration. Its boolean result
  indicates acknowledgement or controlled rejection; the coordinator owns
  routing and socket shutdown. `connection/play.rs` initializes the spectator
  and validates the echoed teleport ID, position and rotation. Other bounded
  frames are discarded because gameplay is outside this connection demo.

Login accepts only protocol 777 and nonempty ASCII usernames containing
letters, digits or underscores (maximum 16 UTF-16 units). Offline UUIDs use
Java's name UUID algorithm: MD5 of `OfflinePlayer:<name>` with UUID v3 bits,
without a namespace prefix. Client UUID claims are ignored. Profiles have
no properties and each Login Success has a random v4 session UUID.
There is no authentication, encryption or compression. A wrong version or
invalid username receives a JSON Login Disconnect; malformed packets close
the socket. Packet bodies must be consumed completely, including the empty
acknowledgement. After acknowledgement the client is in Configuration, so
the server must not send a Login Disconnect. Each Login read uses the same
complete-frame deadline as Status.

The state machine enforces the protocol's sequencing rules: the handshake must
arrive first with intent 1 (Status), at most one status request may be
answered before the ping, the ping request is answered by echoing its timestamp, and any other
packet ID or ordering closes the connection. Each connection also has a
30-second timeout for each complete frame, including its length prefix and
payload. Partial progress does not reset the deadline; expiry reports an I/O
`TimedOut` error. The status state has no disconnect packet; every error is therefore
fatal for the connection.

Wire encoding is delegated to `mclib`: packet bodies (`Handshake`,
`StatusRequest`, `PingRequest`, and the clientbound responses) come from
`mclib::packets`, and transport framing (packet length and packet ID VarInts)
from `mclib::PacketFrame`. The server uses async `PacketFrame::read` and `write`; body codecs operate
synchronously on memory. The server enables mclib’s `tokio-io` feature.
The connection applies the complete-frame timeout.
Each `Connection` owns one Tokio socket without cloning or splitting it;
`Connection::new` returns `Self`, while `run` and `listener::listen` are async.
Reads are sequential: cancelling a partial read requires closing the connection,
so the reader must not be raced against resumable events in `select!`.
`ConnectionError` lives in `error.rs` and is exported at the crate root as
`server::ConnectionError`. The server owns sequencing, I/O, and policy.

## Running

```
cargo run -p server -- --bind 0.0.0.0 --port 25565 --status-json '{"version":{"protocol":777}}'
```

`RUST_LOG=info` enables the connection log.

## Testing

Unit tests cover CLI parsing. Framing tests live in `mclib` and use Tokio
duplex streams. `tests/status.rs` uses async Tokio loopback servers and clients,
The client drives the
protocol with `PacketFrame` sequences, asserting the JSON answer, the echoed
pong, and clean connection closure on protocol violations.

Clippy's strict lint level applies to production code only; it is run without
`--all-targets`, and tests are verified by `cargo test` alone. Test code
carries no clippy allow attributes.
A current-thread runtime test verifies that a stalled connection does not block
another client completing the Status exchange.

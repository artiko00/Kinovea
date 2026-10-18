# Spec Delta

## Purpose

Exposes Kinovea's headless capabilities to agents through the Model Context Protocol, over stdio by default, with the lifecycle, discovery and error semantics MCP clients expect.

## ADDED Requirements

### Requirement: Stdio server starts from the binary
`kinovea mcp` SHALL start an MCP server speaking JSON-RPC over stdin/stdout, answer `initialize` with server name `kinovea`, the crate version and the `tools` and `resources` capabilities, and exit 0 when stdin closes. Logs SHALL go to stderr only.

#### Scenario: Handshake
- **WHEN** a client sends `initialize` then `tools/list`
- **THEN** the response lists every tool of this change with a JSON schema for its input and nothing is written to stdout except JSON-RPC messages

#### Scenario: Client disconnects
- **WHEN** stdin reaches end of file while a session is open
- **THEN** open video readers are released and the process exits 0 within one second

### Requirement: Optional streamable HTTP transport
`kinovea mcp --http <addr>` SHALL serve the same tools over MCP streamable HTTP bound to the given address, refusing to bind non-loopback addresses unless `--allow-remote` is passed.

#### Scenario: Loopback bind
- **WHEN** started with `--http 127.0.0.1:0`
- **THEN** the chosen port is printed on stderr and a client completes `initialize` over HTTP

#### Scenario: Remote bind without flag
- **WHEN** started with `--http 0.0.0.0:8000` and no `--allow-remote`
- **THEN** the server exits 2 with a message explaining the flag

### Requirement: Errors are structured and never crash the server
Tool failures SHALL be returned as MCP tool errors with a stable `code` (`invalid_params`, `not_found`, `unsupported_media`, `external_tool_missing`, `internal`) and a human-readable message; a panic inside one tool call SHALL be caught and reported as `internal` while the server keeps serving.

#### Scenario: Unknown session id
- **WHEN** a tool is called with a session id that does not exist
- **THEN** the result is an error with code `not_found` naming the id

#### Scenario: Tool panics
- **WHEN** a tool implementation panics
- **THEN** the caller receives an `internal` error and the next call on the same connection succeeds

### Requirement: Long operations report progress and honour cancellation
Tracking, video export and sequence export SHALL send MCP progress notifications when the client supplied a progress token and SHALL stop within one frame when the request is cancelled.

#### Scenario: Cancelled export
- **WHEN** a video export is cancelled after the first progress notification
- **THEN** the tool returns a cancelled result, the partial file is removed and the session is still usable

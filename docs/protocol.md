# Host ↔ supervisor protocol (v1)

Frozen in W00. Decision and evidence: ADR-0005. Types: `crates/omni-proto`. Codec: W04. Supervisor: W05 (Unix),
W06 (Windows).

**Start.** The host execs `hugr-omni-supervisor --host-pid <host pid> [--pipe <name>]` lazily on its first spawn,
once per host process. It finds the binary via the `HUGR_OMNI_SUPERVISOR` env var (tests/dev), then next to the
native module, then next to the current executable.
- **Unix:** `socketpair(AF_UNIX, SOCK_STREAM)`, CLOEXEC, no SIGPIPE (`MSG_NOSIGNAL` / `SO_NOSIGPIPE`). The
  supervisor's end is its fd 0. It gets its own process group, so a terminal Ctrl-C to the host's job does not
  reach it.
- **Windows:** the host creates `\\.\pipe\hugr-omni-<host pid>-<128-bit random hex>` with
  `FILE_FLAG_FIRST_PIPE_INSTANCE` and `PIPE_REJECT_REMOTE_CLIENTS`, and starts the supervisor with
  `bInheritHandles = FALSE` and `CREATE_NO_WINDOW` (its own console). It accepts the connection only if
  `GetNamedPipeClientProcessId` is the pid it created. No inheritable host handle exists at any point (R7).

The supervisor arms host-death detection before it serves anything:
- **Linux:** `pidfd_open`, otherwise `getppid()` polling, reported in `Ready.info` (R4).
- **macOS:** kqueue `NOTE_EXIT` + `getppid()`.
- **Windows:** a process handle on the host.

It then sends `Ready`. The host refuses another `version`. A forked host child that inherited the client refuses
to use it (creator-pid check, R6).

**Frame.** `u32 LE length` (of what follows, 1..=1 MiB), then `u8 kind`, then the payload.
- Integers are LE. `bytes` = `u32 len` + data. `list` = `u32 count` + items. `opt u32`: 0 = none.
- Bytes from the OS are raw on Unix and WTF-8 on Windows.
- Every field is present in the order below; no trailing bytes.
- An oversized or malformed frame ends the connection; for the supervisor that is host death.

| kind | message | payload |
|---|---|---|
| 0x01 | `Spawn` | req u64 · program bytes · argv list<bytes> · env list<(bytes, bytes)> · cwd bytes · pty u8 (0/1) · cols u16 · rows u16 · stdin u8 · stdout u8 · stderr u8 (slot: 0 null, 1 pipe, 2 merge) · grace_ms u32 · handles 3×u64 |
| 0x02 | `Go` | req u64 · id u64 |
| 0x03 | `Stop` | req u64 · id u64 · grace_ms u32 |
| 0x04 | `Resize` | req u64 · id u64 · cols u16 · rows u16 |
| 0x05 | `List` | req u64 · id u64 |
| 0x06 | `Release` | req u64 · id u64 |
| 0x81 | `Ready` | version u32 · pid u32 · info u32 (bit 0 pidfd host, bit 1 pidfd members) |
| 0x82 | `Spawned` | req u64 · id u64 · pid u32 · pty 2×u64 |
| 0x83 | `SpawnFailed` | req u64 · code u8 (1 not found, 2 not executable, 3 bad cwd, 4 invalid, 5 io) · errno i32 · msg bytes (UTF-8) |
| 0x84 | `Exited` | id u64 · kind u8 (0 code, 1 signal) · value u32 (code, or signal number) |
| 0x85 | `Stopped` | req u64 · id u64 · forced u8 · elapsed_ms u32 |
| 0x86 | `Processes` | req u64 · id u64 · list<(pid u32 · ppid opt u32 · name bytes, empty = unknown)> |
| 0x87 | `Ack` | req u64 · id u64 · result u8 (0 ok, 1 gone, 2 unknown, 3 error, 4 closed) |

**Replies.** Every request gets exactly one reply with its `req`, including when several host threads overlap (R2).

| Request | Reply |
|---|---|
| `Spawn` | `Spawned`, or `SpawnFailed` (nothing left running) |
| `Go` | `Ack ok` once exec succeeded, or `SpawnFailed` (then no `Exited` follows) |
| `Stop` | `Stopped`, **only once the tree is gone**; at once if it already is |
| `Resize` | `Ack ok` / `closed` / `unknown` |
| `List` | `Processes` (empty once gone), or `Ack error` (incomplete inventory) / `Ack unknown` |
| `Release` | `Ack` |

- **`Stop`:** overlapping calls share the earliest deadline.
- **Unsolicited messages:** `Ready` (once, first) and `Exited` (at root exit; descendants may live on).
- **Tree ids:** unique for the life of one supervisor; the host pairs them with the supervisor generation.
- **No `Signal` message.** Unlike S5, no contract feature needs a signal without a deadline; `Stop{grace_ms: 0}`
  is the forced kill.

**Descriptors.** I/O never crosses the supervisor.
- **Unix pipes:** the host creates them with `pipe2(O_CLOEXEC)`. The child ends travel as SCM_RIGHTS on the first
  byte of the `Spawn` frame, one per `pipe` slot, in stdin/stdout/stderr order.
- **Unix PTY:** the supervisor opens `/dev/ptmx` and returns the master as SCM_RIGHTS on `Spawned` (`pty = [1, 0]`).
  It keeps a dup for `Resize`.
- **Windows:** the host `DuplicateHandle`s its pipe ends into the supervisor (`DUPLICATE_CLOSE_SOURCE`) and puts
  the values in `handles`. For a PTY, the supervisor returns ConPTY output-read / input-write values in `pty`; the
  host pulls them out with `DUPLICATE_CLOSE_SOURCE`.

**Trees.**
- **Kill unit:** the session on Unix (every root is a session leader); the Job on Windows.
- **Graceful stop:** SIGTERM (+SIGHUP for a PTY) + SIGCONT to the root's group and every live session member;
  `CTRL_BREAK` from the supervisor's console for a pipe root on Windows, `ClosePseudoConsole` on a worker for a
  PTY root.
- **Forced stop:** SIGKILL, re-swept until the session is empty; `TerminateJobObject` on Windows.
- **Pinning:** the root stays unreaped until its session has no live member, `Release` notwithstanding (R3).
  A gone tree is never signalled again.
- **`List`:** reports exactly the members `Stop` would reach (the zombie root excluded). `ppid` is set only when the
  parent is in the same list.
- **Host death** (EOF, pidfd/kqueue/handle, `getppid`, protocol error): `Stop` every live tree with its own
  `grace_ms`, from one shared inventory, then exit when all are gone (hard bound: max grace + 2 s).
- **Supervisor death:** every pending host call of that generation fails with `IO`, and the next spawn starts a new
  supervisor. Windows trees die with their Jobs; Unix trees are declared unprotected (GUARANTEES).

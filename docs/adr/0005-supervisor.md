# ADR-0005 — A supervisor process owns every managed child

Status: **accepted by the lead, pending spike S5** (2026-10-02). Supersedes the in-host parts of ADR-0001,
ADR-0002 and ADR-0003 listed below.

## Context

Independent Codex reviews rejected ADR-0001/0002 and ADR-0003 as product baselines
(`docs/research/adr-reviews-s1-s2.md`). Their must-fix findings share one root cause: process lifecycle work
running **inside the host** (Node/Bun/Deno/Python) cannot be made safe:

- forking the multi-threaded host without exec (watchdog) has no async-signal-safety proof;
- a host that sets `SIGCHLD` to `SIG_IGN` or reaps with `waitpid(-1)` destroys the zombie that pins a
  process-group ID, so a later `killpg` can hit an unrelated group;
- the window between spawning a child and registering it with a watchdog loses trees when the host dies;
- `posix_spawn` cannot run the post-fork work we need (`TIOCSCTTY`, closing inherited host fds,
  resetting signal state) on every supported libc;
- Windows graceful stop and PTY Ctrl+C both require changing **process-global** console state of the host
  (`AttachConsole`/`FreeConsole`, `SetConsoleCtrlHandler(NULL, FALSE)`).

## Decision

1. **A small helper executable, `hugr-omni-supervisor`, ships next to the native module for each target and
   is started lazily, once per host process, by exec (fresh runtime, no inherited allocator or lock state).**
2. **The supervisor creates every managed process.** The host sends spawn requests over a private channel
   (Unix: socketpair; Windows: anonymous pipe pair) and never forks.
   - Unix: the supervisor is single-threaded, so `fork` + child-side setup is safe there: `setsid`,
     `TIOCSCTTY` for PTYs, `chdir`, `dup2` of the stdio ends it received, closing every other descriptor,
     default signal dispositions and an empty mask, then `exec`; exec failure is reported through a CLOEXEC
     pipe. It owns `SIGCHLD`, reaps its own children, and keeps a root unreaped until the group has no other
     members or the host released the handle — so numeric group IDs are never reused under it.
   - Windows: the supervisor owns its console state (Ctrl+C not ignored), creates children with
     `CreateProcessW` + `PROC_THREAD_ATTRIBUTE_JOB_LIST` (+ `PSEUDOCONSOLE` for PTYs) and std-equivalent
     argument quoting, holds the Job handles (`KILL_ON_JOB_CLOSE`) and sends CTRL_BREAK from its own console.
     `ClosePseudoConsole` runs on a dedicated worker while an independent deadline enforces `graceMs`.
3. **I/O stays in the host.** Pipe read ends and the PTY master/output pipe are passed to the host
   (SCM_RIGHTS / `DuplicateHandle`); the host pumps output itself, so the supervisor is not on the data path.
4. **Host death:** the supervisor notices the channel closing (plus pidfd / kqueue / process handle on the
   host), stops every tree with one bounded deadline, reaps, and exits. On Windows, the supervisor's own death
   closes the Jobs and the kernel kills the trees.
5. **Supervisor death (Unix):** the host detects the closed channel, fails every live `Child` with `IO`, and
   restarts the supervisor for new spawns. Trees whose parent died are declared unprotected in GUARANTEES.
6. **The host installs no signal handlers and no exit hooks** (unchanged from ADR-0002).

## What remains accepted from the spikes

- ADR-0001: Windows containment at creation (now via `JOB_LIST`, not `NtResumeProcess`); nested Jobs work;
  never use or allow breakaway; graceful stop on Windows is best effort, forced is guaranteed; Unix group
  signalling, setsid escapes declared; the spawn-latency baselines.
- ADR-0002: KILL_ON_JOB_CLOSE for Windows host death; no signal chaining in bindings.
- ADR-0003: own PTY layer; `/dev/ptmx` with `O_CLOEXEC`; `ptsname_r` (Linux) / `TIOCPTYGNAME` (macOS);
  ConPTY with flags 0, closed PTY-side ends, explicit null standard handles; PTY kill unit is the session.

## Consequences

- One more shipped file per target (size measured in S5); the npm platform packages carry it.
- One IPC round trip per spawn; K4 must still hold (≤ 1.25× the stdlib).
- GUARANTEES gets explicit tiers: Windows trees are kernel-contained; Unix trees are contained per process
  group (session for PTYs) while the supervisor lives; `setsid` escapes and supervisor death are declared.
- Contract tests and spike evidence must **assert** outcomes and fail CI on a bad result; printed counters
  are observations, not acceptance (lesson from both reviews).

## Validation (spike S5, must pass before the W00 seam freeze)

Asserting tests on Linux, macOS and Windows, with Node, Bun, Deno and CPython hosts where relevant:
spawn through the supervisor (pipe and PTY), fd/handle passing, exit events, `stop()` with SIGTERM-resistant
trees, host death in 7 ways including during spawn, a host that ignores `SIGCHLD` / reaps with `waitpid(-1)`,
PID-reuse sentinel, supervisor kill, PTY Ctrl+C on Windows with the host left untouched, ConPTY close with a
stubborn writer under a short `graceMs`, and spawn latency against the stdlib in the same job.

# Guarantees

What hugr-omni promises on each OS, and the evidence for it. A row is **planned** until its scenario or KPI is green
on that OS in CI; then it links the run. A promise without evidence on an OS is not made on that OS (INV-09).
The promises themselves are `docs/api-contract.md`; the items are `docs/acceptance.md`.

## Containment tiers (ADR-0005)

| | Linux | macOS | Windows |
|---|---|---|---|
| Kill unit | the root's session | the root's session | the Job (no breakaway) |
| A descendant that leaves on purpose | `setsid` (a new session) **escapes** (declared); changing process group inside the session stays contained. A deliberate "relay" — processes that fork and exit faster than the supervisor's scans, every time — can likewise evade confirmation; ordinary programs never do this, and containing hostile code is the sandbox's job | same as Linux | cannot leave: breakaway is refused |
| Graceful stop | SIGTERM (+SIGHUP for a PTY) + SIGCONT to the session | same as Linux | CTRL_BREAK to console processes (best effort); `ClosePseudoConsole` for a PTY |
| Forced stop after `graceMs` | SIGKILL until the session is empty: guaranteed | same as Linux | `TerminateJobObject`: guaranteed; `stop()` resolves when the Job is empty and every member's handle is signalled, or at most 1 s after the Job is empty when a member cannot be inspected (a process that denies SYNCHRONIZE, or a lost Job notification) |
| Host process dies (any way, incl. SIGKILL) | the supervisor stops every tree within its grace; if its descriptor limit is forced below its 3 reserved descriptors, members outside the root's process group can be unreachable, and it then exits 1 with a stderr report (never 0) | same as Linux | same, and the Jobs close if the supervisor dies too |
| Supervisor process dies | trees keep running, **unprotected** (declared); the next spawn starts a new supervisor | same as Linux | trees die with their Jobs |
| PID reuse | a gone tree is never signalled again (root pinned until the session is empty) | same as Linux | the root handle is held until the tree is gone and released |
| Host state | no signal handler, no exit hook, no fork in the host | same as Linux, plus a declared window: an fd received from the supervisor gets CLOEXEC a moment after creation, so a concurrent `fork` in the host can inherit it | the host's console state never changes |

## Promises

| Item | Linux | macOS | Windows | Evidence |
|---|---|---|---|---|
| C-SPAWN-01 resolution on the child's PATH (+PATHEXT) | planned | planned | planned | scenario |
| C-SPAWN-02 args byte-identical, `.cmd`/`.bat` safe or refused | planned | planned | planned | scenario |
| C-SPAWN-03 relative program and cwd | planned | planned | planned | scenario |
| C-ERR-01 / C-ERR-02 typed errors before anything runs | planned | planned | planned | scenarios |
| C-ENV-01 environment | planned | planned | planned | scenario |
| C-IO-01..04 output never blocks the child; loss counted in order; nothing lost at exit | planned | planned | planned | scenarios + K5 |
| C-RUN-01 `run()` complete or `OUTPUT_LIMIT` | planned | planned | planned | scenario |
| C-KILL-01 / C-KILL-02 whole tree, one deadline | planned | planned | planned | scenarios + K1, K3 |
| C-KILL-03 deliberate escape behaves per tier | planned | planned | planned | scenario |
| C-EXIT-01 exit codes and reason | planned | planned | planned | scenario |
| C-PROC-01 `processes()` = what `stop()` reaches | planned | planned | planned | scenario |
| C-SCOPE-01 scope exit kills the tree | planned | planned | planned | scenario |
| C-TMO-01 / C-TMO-02 timeout and cancellation | planned | planned | planned | scenarios + K2 |
| C-HOST-01 host exit leaves trees per tier | planned | planned | planned | idiom tests + K1 |
| C-PTY-01..04 terminal size, interaction, no lost output, tree stop | planned | planned | planned | scenarios |
| ConPTY on builds < 26100: per-pseudoconsole handle leak | – | – | to be measured (W12w) | W12w |
| C-TS-01 / C-TS-02 TS on Node 22/24, Bun, Deno | planned | planned | planned | runner |

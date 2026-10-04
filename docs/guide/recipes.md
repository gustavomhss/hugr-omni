# Recipes for agents

Four jobs an agent does all day, as code you can paste: run a dev server, run tests under a deadline, drive an
interactive terminal, and clean up when something is cancelled or ends. TypeScript first; the Rust option names are
the same (see the end).

Every block here is run by CI (`scripts/readme-check`) on each OS, inside a small project with an `npm run dev` that
prints `ready` and an `npm test` that passes. To adapt one, change the command.

Which function: `run` when the program ends by itself and you want its whole output; `spawn` when it keeps running, or
you want to read as it goes, write to it, or stop it.

## 1. A dev server: wait until it is ready, stop the whole tree

`npm run dev` is npm, a shell, node and often more processes behind it. Stopping only the first leaves the rest
holding the port. `stop()` ends all of them, and `await using` calls it when the scope ends, even if the code throws.

```ts
import { spawn } from "hugr-omni";

// timeoutMs is the budget of the whole session: when it is spent, the tree is stopped.
await using server = spawn("npm", ["run", "dev"], { timeoutMs: 10 * 60_000 });

let ready = false;
for await (const line of server.lines()) {
  if (line.text.includes("ready")) {
    ready = true;
    break; // leaving the loop detaches the output for good
  }
}
if (!ready) throw new Error(`the dev server ended before it was ready: ${JSON.stringify(await server.wait())}`);

// ... fetch pages, run the browser test ...
console.log(await server.processes()); // [{ pid, parentPid, name }, ...]: what stop() would end right now
// Leaving the scope stops npm, the shell and node.
```

- `lines()` and `output` are two views of one consumer. Claim it once; a second `for await` is `INVALID_ARGUMENT`.
- A server that stays alive without ever printing `ready` ends the loop only when the budget stops it, so say how long
  you can wait.
- `processes()` answers "why is the port busy?" with pids and parent links, never command-line arguments.

## 2. Tests with a deadline

A test run that hangs (a worker that never exits, a prompt nobody answers) must not hang the agent. With `timeoutMs`,
`run()` stops the whole tree, jest workers included, and still resolves, with `reason: "timeout"` and the output so
far.

```ts
import { run } from "hugr-omni";

const result = await run("npm", ["test"], { timeoutMs: 5 * 60_000, graceMs: 5_000 });

if (result.reason === "timeout") {
  throw new Error(`the tests hung; the whole tree was stopped. Output so far:\n${result.stdout}`);
}
if (!result.success) {
  throw new Error(`the tests failed (exit ${result.exitCode}):\n${result.stdout}${result.stderr}`);
}
console.log("tests passed");
```

What a hang looks like, with a deadline short enough to try:

```ts
import { run } from "hugr-omni";

const hung = await run("node", ["-e", "setInterval(() => {}, 1000)"], { timeoutMs: 1_000, graceMs: 500 });
console.log(hung.reason, hung.success); // timeout false
```

- A non-zero exit never throws. Timeouts do not throw either; both are results you read.
- `graceMs` is how long the tree gets to wind down (a cooperative program finishes its cleanup) before it is forced.
- Output above `maxOutputBytes` (16 MiB per stream by default) stops the tree and rejects with `OUTPUT_LIMIT`; the
  first bytes are in `error.result`. For more, use `spawn()` and read it as it comes.

## 3. An interactive terminal

Some programs only talk to a terminal: prompts, REPLs, `git rebase -i`. With `pty`, the child sees a real terminal
(openpty on Unix, ConPTY on Windows) of the size you ask for.

```ts
import { spawn } from "hugr-omni";

const program = `process.stdout.write("name? "); process.stdin.once("data", (d) => { console.log("hello " + String(d).trim()); setInterval(() => {}, 1000); });`;
await using term = spawn("node", ["-e", program], { pty: { cols: 100, rows: 30 } });

let seen = "";
let answered = false;
for await (const chunk of term.output) { // one loop for the whole conversation: leaving it detaches for good
  seen += chunk.data;
  if (!answered && seen.includes("name?")) {
    answered = true;
    await term.write("alice\r"); // \r is Enter
  }
  if (seen.includes("hello alice")) break;
}
await term.write("\x03"); // Ctrl-C interrupts the foreground program
console.log((await term.wait()).reason);
```

- Read `output` (chunks), not `lines()`, to see a prompt: it has no newline yet.
- The terminal shows everything, including the echo of what you typed; chunks are labeled `pty`.
- There is no `closeStdin()` on a terminal: send the program's own end of input, for example `"\x04"`.
- `term.resize(cols, rows)` changes the size later. `run()` with `pty` takes no `input`: nobody types.

## 4. Cleanup: cancel, errors, and the end of your own process

Cancelling stops the whole tree first, and then tells you. For a `spawn()`, `wait()` resolves with
`reason: "aborted"`:

```ts
import { spawn } from "hugr-omni";

const controller = new AbortController();
const child = spawn("node", ["-e", "console.log('working'); setInterval(() => {}, 1000)"], { signal: controller.signal });

for await (const line of child.lines()) {
  if (line.text === "working") controller.abort(); // the user pressed cancel
} // the output ends once the tree is stopped
console.log((await child.wait()).reason); // aborted
```

For a `run()`, the promise rejects with `ABORTED`, and `error.result` holds what was collected:

```ts
import { OmniError, run } from "hugr-omni";

try {
  await run("node", ["-e", "console.log('partial'); setInterval(() => {}, 1000)"], { signal: AbortSignal.timeout(2_000) });
} catch (e) {
  if (!(e instanceof OmniError) || e.code !== "ABORTED") throw e;
  console.log("cancelled, the tree is stopped; output so far:", e.result?.stdout);
}
```

Without `await using` (plain Node), `finally` does the same:

```ts
import { spawn } from "hugr-omni";

const server = spawn("npm", ["run", "dev"]);
try {
  for await (const line of server.lines()) if (line.text.includes("ready")) break;
  console.log("the server is ready, doing the work");
} finally {
  await server.stop(); // the whole tree, also when the work above threw
}
```

- When your own process ends, by any means (a normal end, `process.exit()`, an uncaught exception, SIGINT, SIGTERM,
  even SIGKILL), the supervisor stops every tree it started, within its grace. hugr-omni installs no signal handler
  and no exit hook in your process, so there is nothing to register. What holds on each OS, and the one thing that
  escapes on purpose (`setsid`), is in [GUARANTEES.md](../../GUARANTEES.md).
- `stop()` after the tree is gone returns the same result again. It is always safe to call twice.

## Rust

The options have the same meaning and, in Rust style, the same names: `Command::new(..).args(..)`, `.timeout(..)`,
`.grace(..)`, `.cancel_on(token)`, `.pty(PtySize { cols, rows })`, `.stdin(..)`, `.merge_stderr(..)`, then `.run().await`
or `.spawn()` / `.spawn_pty()` for a child you stream from, write to and `stop(None).await`. Dropping a child kills its tree at
once without waiting; call `stop(None)` for a graceful end. The table that places every TypeScript and Rust name under its
concept is [scripts/surface-check/parity.txt](../../scripts/surface-check/parity.txt); Python joins it in v0.2.

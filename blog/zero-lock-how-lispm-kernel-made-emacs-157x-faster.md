Fifty years of engineering sit inside GNU Emacs. Nearly everything a modern AI agent harness tries to assemble from scratch is already there: you can browse the web, manage mail, write code, reach remote servers, and even brew coffee over a hardware protocol. During the Editor Wars, people mocked Emacs as "an operating system disguised as a text editor." Looking at AI agents in 2026, that old joke describes the exact target every harness is chasing. Most harnesses have to build fragile Computer Use layers that scrape pixels and simulate clicks because they live outside the user's software. Emacs does not need to imitate an operating system for an agent; it already is one.

That is why we built [LispM](https://lispm.ai/). Our goal is to give Emacs a modern interface and interaction model built around agents, while collapsing the sprawling tool menus of conventional harnesses in [LispM Agent](https://lispm.ai/agent/) down to a single operator: `eval_elisp`.

## The Single-OP Idea, and Emacs's GIL

Most agent frameworks grow by addition. Browsing needs a browser tool, mail needs a mail tool, version control needs a git tool, and before long the prompt carries dozens of tool schemas. Every new wrapper bloats the context window, yet the agent is still stuck ordering off a fixed menu: anything outside that menu does not exist.

Inside Emacs, buffers, windows, terminals, browser views, mail, and Org documents are all Lisp functions living in one self-documenting world. Every function carries its own documentation, every variable can be inspected on the spot, and every edit lands on Emacs's undo tree. Exposing `eval_elisp` replaces that long list of single-purpose wrappers with one Turing-complete operator. Instead of picking from pre-baked actions, an agent asks Emacs what it knows how to do, writes the Lisp expression a task calls for, and pauses for your approval whenever an action would change files or run a command.

There is one hard ceiling on this design, and it sits in Emacs Lisp itself: a global lock. Like other dynamic languages of its era, Emacs runs Lisp and garbage collection on a single thread; even its cooperative Lisp threads share that one global lock. When several coding agents run side by side, scanning hundreds of megabytes of session histories, negotiating network handshakes, or spawning external programs stalls that thread. Keys stop echoing and frames drop.

Python spent more than thirty years fighting the same bottleneck, from the birth of its Global Interpreter Lock in the early 1990s to the free-threaded runtime in recent releases, because decades of extensions assumed nothing ran in parallel. Emacs's C core is battle-tested, and tearing it apart to make Elisp preemptively multi-threaded would break forty years of packages. Rather than rewrite Emacs's core, we kept its architecture intact and placed a kernel beside it: `lispm-kernel`.

## Iterate, Then Iterate Again

If LispM is an operating system for agents, `lispm-kernel` is its kernel. It schedules external work, governs resource budgets, and ensures slow tasks never block Emacs or starve one another.

At first, we wrote `lispm-kernel` in Rust. Rust gave us strictness and memory safety, and we loaded it straight into Emacs as a native module. That initial design proved the scheduling idea, yet living inside Emacs's own address space carried an obvious risk: a single native fault could take the editor and every open buffer down with it. Translating Lisp data back and forth across a C boundary also felt foreign in a Lisp Machine.

Then we rewrote the kernel in Common Lisp, running it as an independent process beside Emacs. The two Lisps talk in S-expressions over standard streams, while Emacs takes incoming results in tiny slices between frames so typing always comes first. Both halves of LispM finally spoke the same language, Common Lisp gave us true multi-core threads with no global lock, and process isolation meant a crashed kernel could never touch an unsaved buffer.

Now, over the past few days, we have torn out the kernel's internals once more and rebuilt the entire runtime without a single mutex.

## Beyond Mutexes: Why We Chose CSP

Moving heavy work into the Common Lisp kernel had already brought major speedups. Scanning hundreds of agent session transcripts ran more than forty times smoother on Emacs's UI thread than reading them in Elisp, and background package downloads and command execution stopped freezing the cursor during network waits.

Even so, one problem lingered inside the kernel: mutexes. Shared task queues, output buffers, and caches all relied on locks. In concurrent and distributed systems, mutexes bring familiar failure modes. Every fast check queues up behind contended locks on the hot path; cancelling one stalled job forces broadcast wakeups that rouse unrelated workers just to fight for the lock and go back to sleep; and subsystems that look safe in isolation risk deadlocks the moment they compose across boundaries or meet operating-system signals.

When teams move away from locks, they usually look to the Actor Model. Yet Actors carry quiet trade-offs for a runtime kernel: actor mailboxes are unbounded by default, hiding overload until memory runs out, and splitting sequential work across actor turns breaks normal call stacks, error recovery, and resource cleanups.

For the new `lispm-kernel`, we turned to Tony Hoare's Communicating Sequential Processes (CSP) instead. CSP is a complete formal model for lock-free asynchronous concurrency, and in several respects it fits systems work even better than Actors. Instead of scattering state across unbounded mailboxes, CSP connects a small, fixed set of sequential loops through explicit channels with built-in backpressure.

## A Kernel Without Locks

In `lispm-kernel` 0.9.0, no two threads ever lock or mutate the same state. One sequential loop owns scheduling, priorities, and admission; workers pull jobs from private channels and run in parallel without touching shared tables; and the session cache publishes immutable snapshots so readers never wait on writers.

Flow control lives in the channels themselves. Streaming output carries a strict byte budget, so a noisy background job pauses cleanly whenever Emacs is busy with user input rather than flooding memory. Cancelling a stale task sends a targeted signal down its channel, stopping that job at once without waking the rest of the worker pool. On the Emacs side, views always render immediately from the latest snapshot while fresh readings arrive quietly in the background.

![Benchmark chart comparing GNU Emacs UI stall latency across 256 session transcripts (68.6 MB JSONL): pure Emacs Lisp pauses for 206.35 ms with 80 ms GC, the earlier mutex kernel pauses for 5.42 ms, and the lock-free CSP lispm-kernel 0.9 pauses for 1.31 ms with 0 ms GC (157x faster), and 1.64 ms versus 266.52 ms at p95 (162x faster) alongside eight active agent terminals.](/assets/blog/zero-lock-how-lispm-kernel-made-emacs-157x-faster/benchmark.webp "Scanning 256 agent transcripts (68.6 MB JSONL) in GNU Emacs: worst-case UI stall drops from 206.35 ms in pure Elisp to 1.31 ms with the lock-free CSP lispm-kernel (157x faster, 0 ms GC), and 1.64 ms at the 95th percentile alongside eight active agent terminals.")

The difference shows directly under the cursor. Scanning 256 agent transcripts (68.6 MB of session logs) in pure Emacs Lisp freezes the editor for **206.35 ms**, dropping a dozen frames at 60 fps while spending 80 ms in garbage collection. Our earlier mutex kernel brought that pause down to **5.42 ms**. With the lock-free CSP kernel, worst-case UI stall drops another 76% to **1.31 ms**, with zero Emacs garbage collection, making Emacs **157x faster** to respond during heavy scans, and **162x faster** at the 95th percentile (**1.64 ms** versus **266.52 ms**) even while eight live agent terminals stream output concurrently.

## Try It

`lispm-kernel` 0.9.0 is open source under AGPL-3.0-or-later, with prebuilt binaries for macOS and Linux at [github.com/LispMachineLab/lispm-kernel](https://github.com/LispMachineLab/lispm-kernel). If you already use [LispM Desktop](https://lispm.ai/desktop/), Software Update brings it automatically; if you want to [download LispM](https://lispm.ai/download/) on macOS or Linux, one line installs the whole system:

```sh
curl -fsSL https://lispm.ai/install.sh | sh
```

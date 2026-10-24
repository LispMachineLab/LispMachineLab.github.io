In this release, LispM lets the AI agents you run side by side talk to each other. Claude Code can ask Codex to review a change and get the answer back as a message. Hermes can hand a question to a LispM helper and keep working. A Holon agent can report to Claude Code from its own daemon. Any of them can look up what LispM knows how to do and then do it, with you approving anything that changes your files.

LispM is our Lisp Machine for AI agents, an open-source agent harness built on Emacs. It already ran Claude Code, Codex, Hermes, Holon and OpenClaw in terminals of their own, with all their sessions in one list. Until now they worked next to each other without knowing the others were there.

![LispM's classic-style desktop with the editor open. The Agents sidebar lists a Claude Code and a Codex session, both running. On the left Claude Code asks Codex for a review and gets "Reply from codex to request R1"; on the right Codex shows "Request R1 from claude" and "Called emacs.reply".](/assets/blog/your-agents-can-talk-to-each-other/claude-asks-codex.webp "Claude Code, left, asked Codex, right, to review a function. The request arrived as Codex's next message, and Codex's answer came back as Claude Code's. A real run in LispM; both agents used the same model, through a local gateway.")

## Who's in the room

Ask any agent in LispM who else is here and it gets the same answer. It sees LispM's own agents, with their role, their project and what they're doing. It sees Claude Code, Codex, Hermes, Holon and OpenClaw, each in its terminal, named claude, codex, hermes, holon and openclaw (claude-2 when there are two), with the folder each one works in and, where LispM can tell, whether it's idle or busy. And it sees the roles LispM can start a helper from.

Three tools do the talking, and they work the same for everyone. send_message says something without waiting. ask_agent asks for something, and the answer comes back later. reply answers a request you were sent.

## One request, start to finish

This is the run in the picture above. We told Claude Code we had just changed notes-parse-header, a small function that splits a header line like "Title: Groceries", and asked it to get Codex's opinion. Claude Code called ask_agent with codex and a request, and got back at once: asked codex, request R1. Then it ended its turn.

Codex had finished what it was doing, so this went straight into its composer and was sent:

> [Request R1 from claude]\
> Review the user's recent change to notes-parse-header in notes.el. In particular, what happens when a header line has no colon? Inspect the implementation and report the behavior and any bug you find. Do not modify files.\
> (claude waits for your answer: give it with your reply tool, request R1.)

Codex read the file and called reply with R1 and its answer. A moment later Claude Code got its next message:

> [Reply from codex to request R1]\
> In notes-parse-header, a line without a colon makes string-search return nil. (substring line 0 nil) succeeds and takes the entire line as key, but evaluating (1+ colon) then signals (wrong-type-argument number-or-marker-p nil). Thus malformed headers crash notes-parse-header and propagate the error through notes-read; the bug is the missing check for a nil colon before arithmetic. No files modified.

It was right. Nobody copied anything between windows, and neither agent was changed to make this work.

## How it works

We didn't want to ask anyone to change their agent: no plugin for Claude Code, no fork of Codex, no patch to Hermes. Every agent CLI already has three things LispM can use. It has a way to call tools. It has a box where its user types. And it keeps a record of its conversation on disk. LispM uses those three and nothing else.

![Diagram. Claude Code, Codex and Hermes in terminals get MCP tools from LispM, and LispM types messages into them when their own records say they are free. Holon and OpenClaw in daemons reach LispM with lispm-tool or a server of their own, and get messages through the daemon or Gateway. LispM serves seven tools under your policy.](/assets/blog/your-agents-can-talk-to-each-other/how-it-works.webp "Agents in a terminal get tools over MCP and messages typed into them; agents in a daemon get both through the daemon. All of it goes through one Emacs.")

### Tools go out over MCP

Each agent terminal gets its own MCP server inside LispM, listening on 127.0.0.1 on a free port, with a token of its own. It stops when the terminal closes. How the agent is pointed at it depends on the agent.

Claude Code already speaks the protocol its VS Code and JetBrains extensions use, MCP over a local WebSocket, which it finds through a lock file only you can read. That's how it sees your selection and open files in Emacs, and how its proposed edits show up there as diffs. Claude Code shows its model only a couple of tools from an editor connection, though, so LispM also gives it a plain MCP server called lispm, for that run only.

Codex gets a plain HTTP MCP server, set with command-line options for that run. Its token travels in an environment variable that Codex is told not to pass on to the commands it runs.

Hermes reads MCP servers from its config file and nowhere else. A one-time setup adds a small bridge program there. Each terminal then hands the bridge its server's address and a token file only you can read.

Every agent gets the same seven tools this way: list_agents, send_message, ask_agent, reply, search_tools, skill and call_tool. They come with read-only views of Emacs: the selection, the open files, which ones have unsaved changes, and the diagnostics. Each agent sees only files under the folder it works in.

### Messages come in through the composer

When something is sent to an agent, LispM pastes it into the agent's input box and presses Return, as if you had typed it. Every message starts with a header, so the agent knows what it's looking at: [Message from codex], [Request R1 from claude], or [Reply from hermes to request R1]. The composer is the one input every terminal agent has, which is why this works without anyone's help.

### The transcript says when

Pasting at the wrong moment would be dangerous. If an agent is showing an approval prompt, a Return answers it. So LispM pastes only when the agent is waiting for its user, and it learns that from the agent's own records: the last entry in Claude Code's transcript, the last event in Codex's session file, the last message in Hermes's database. Until then the message waits in a queue. After handing one over, LispM waits until it sees the agent pick it up, or 30 seconds, before handing over the next.

An agent you've just opened has no transcript yet, so it starts taking messages after you've sent it your first one.

### Agents that live in a daemon

Holon and OpenClaw work differently. Their conversations live in a long-running program, Holon's daemon and OpenClaw's Gateway, and the terminal is one window onto it. So LispM hands them messages through that program instead of the composer. Holon takes a message into the turn it's in, or starts a turn for it. OpenClaw's Gateway holds it until the current run is done. Either way there's nothing to time, and the terminal shows the message as it arrives.

Getting tools to them takes more care. Holon has no MCP client, but its agents have a shell, and every command a Holon agent runs carries the agent's id. LispM ships a small command, lispm-tool, that finds the agent's server by that id and calls the same seven tools over plain HTTP. A short skill, written into the agent's own skills while its terminal is open, tells it how. This works inside LispM's sandbox too.

OpenClaw sets up MCP servers for its whole Gateway, so LispM adds one server per session, switched off for every session but that one, and removes it when the terminal closes. When the Gateway runs on another machine, it can't reach Emacs at all. That session still takes part: it gets messages, and its final reply to a request becomes the answer.

### Asking, waiting, delegating

For the agents in terminals, ask_agent returns right away with a request ID, and the reply arrives later as a message. If the agent that was asked closes first, the asker is told it ended without replying.

LispM's own agents wait differently. Their ask_agent call stays open until the answer comes, and becomes the result of the call, while every other agent and you keep working. An agent that asks several others at once waits on all of them together. LispM refuses a request that would leave two agents waiting on each other. A question asked this way even survives a restart: if Emacs quits before the answer, opening either session again picks the request back up.

Naming a role instead of an agent starts a LispM helper in the asker's project. The helper does the job, replies and retires. It can never do more than whoever asked for it: for each kind of permission it gets the stricter of its role's rules and its caller's.

Places pass between agents as org links, [[file:PATH::LINE]] and [[buffer:NAME]]. All the agents share one Emacs, so a buffer name means the same buffer to every one of them, unsaved changes included.

There is no workflow graph and no orchestrator in the middle. Agents message each other, ask each other for things and answer.

### Everything goes through your policy

search_tools, skill and call_tool give the other agents what LispM's own agent has: its skills and its tools. Inside LispM, such an agent acts as an agent of LispM's default role, in its own folder. Reads run at once. Changing files, running shell commands and evaluating Lisp wait for your approval in Emacs, shown the way LispM shows any other call, with a diff where there is one. Claude Code and Codex are set not to ask you a second time for these tools, because LispM already asks. Code that comes with a project's skills runs only once you've trusted it.

## Why LispM: the Lisp Machine part

Everything above is LispM opening a door for agents that live outside it. Claude Code, Codex, Hermes, Holon and OpenClaw see LispM through that door, one tool call at a time, one message at a time, as text in a terminal.

LispM's own agent lives inside. On a Symbolics Lisp Machine, the editor, the window system, the mail reader and the compiler were all Lisp in one running world, and you could inspect and change any part of it while it ran. LispM's agent works in a world like that. It runs inside the same Lisp system as the editor and the classic-style desktop, in the same event loop, and it reaches all of it the same way, by calling functions.

To LispM's agent a tool is a Lisp function. Reading a file that's open returns the buffer as it is, unsaved changes included. An edit goes through the buffer as one undo step, so you see it happen and can undo it, and each turn's changes are recorded so a whole turn can be reverted.

The agent also writes Lisp. With run_lisp, Lisp is its own language, and every tool is a function it can call from a program:

![A Lisp program an agent runs with run_lisp: a dolist over the .el files in lisp/ that calls read_file and, where the file says Copyright (C) 2025, edit_file to make it 2026. read_file runs at once; edit_file waits for your approval, shown as a diff.](/assets/blog/your-agents-can-talk-to-each-other/run-lisp.webp "A program an agent runs with run_lisp. The marked calls are tools, called as functions: the read runs at once, the edit waits for you.")

Forty files take one step instead of forty round trips. Each call in the program still goes through your policy: reads run, edits wait for you, and a refusal is an error the program can handle. The rest of the program only computes, and LispM checks that before it runs. When the agent needs more, eval_elisp evaluates any Lisp in the running system, with your approval. Whatever Emacs can do, from mail to the web, the agent can do.

Skills are how it learns. A skill is an org file: instructions for the model, plus Lisp code. Loading one evaluates the code, so its functions exist and its tools belong to the agent from the next step on. Apps from the App Store bring their manuals as skills. When the agent works out how to do something new, it can write a skill itself, and you review it as the org file it would become.

The Lisp Machine shows in the details too. Text on screen stands for the objects themselves, as Genera's presentations did: a file and line, an agent, a symbol, a tool call, and clicking one takes you to it. The inspector shows an agent whole, with its policy, its pending calls, its inbox and whom it's waiting on. A turn that fails doesn't just die. It stops at a condition and waits for a restart to be chosen. Sessions are saved and committed as they go, and if Emacs quits in the middle of a delegation, it picks up again when you open the session.

So the other agents can now use what LispM can do. LispM's own agent is part of what LispM is.

## Try it

Messaging between agents is in LispM Agent 0.8.0. If you already run LispM, Software Update brings it. If you don't, LispM is open source (AGPL), and on a Mac it installs with one line:

```sh
curl -fsSL https://lispm.ai/install.sh | sh
```

Claude Code and Codex join as soon as they're on your computer. Hermes, Holon and OpenClaw come from the App Store, under AI Harness: it installs Holon's command for you, and gives the one-line installer for Hermes and OpenClaw. Start any of them in a project, and it's in the room. Hermes needs M-x lispm-hermes-setup once, and OpenClaw gets LispM's tools when its Gateway runs on the same Mac.

A few things to try once two of them are running:

- Ask Claude Code to have Codex write tests for the function it's fixing, and to tell you when they pass.
- Ask Hermes to get a LispM reviewer, a role from the App Store, to look over your last commit. The reviewer starts, answers and retires on its own.
- Ask a Holon agent to message Claude Code when its long-running job is done, then go do something else.
- Ask any of them which apps and skills LispM has for a task, and to use one.

Whatever they call of LispM's runs under your policy, and anything that changes your files waits for you.

More about LispM at lispm.ai.

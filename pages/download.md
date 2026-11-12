One line installs LispM on macOS or GNU/Linux. GNU Emacs 31.1, the editor LispM is built on,
comes with it, so you need not know Emacs to begin:

```sh
curl -fsSL https://lispm.ai/install.sh | sh
```

The installer script at [install.sh](https://github.com/LispMachineLab/lispm-desktop/blob/main/install.sh)
keeps LispM's files in `~/.lispm`. On a Mac (`macOS 11` or later, on Apple silicon or Intel) LispM
installs Apple's command line tools if they are missing and puts `LispM.app` in `Applications`. On
GNU/Linux (`Debian 12` or `13`, or `Ubuntu 22.04` or `24.04`, on `amd64` or `arm64`) LispM installs
its own GNU Emacs 31.1 with xwidgets in `/opt/lispm/emacs` via `apt`, beside any Emacs of the
system, and adds the `lispm` command and LispM to the applications menu.

From then on, Software Update in LispM brings each release by itself, and running the install line
again updates LispM too. To uninstall LispM, see
[LispM Desktop's installation guide](https://github.com/LispMachineLab/lispm-desktop#installation).

## LispM Experimental (GNU Emacs 32)

LispM Experimental runs the same LispM desktop on GNU Emacs 32, still in development, for features
that only Emacs 32 supports, such as OpenGL drawn inside an Emacs buffer for Doom. Once installed,
you can also switch live between GNU Emacs 31 and GNU Emacs 32 inside LispM from Control Panels →
System → Kernel. LispM Experimental runs on macOS only:

```sh
curl -fsSL https://lispm.ai/install.sh | LISPM_EMACS=32 sh
```

Run the same line again to update LispM Experimental. To move `LispM.app` back onto GNU Emacs 31.1:

```sh
curl -fsSL https://lispm.ai/install.sh | LISPM_EMACS=31 sh
```

## Already use Emacs?

LispM Agent, LispM Desktop and LispM Browser also stand on their own inside your existing GNU Emacs
30.1 or later.

### LispM Agent

Run LispM's AI agents in your own Emacs 30.1 or later, then run `M-x lispm` in any project
([github.com/LispMachineLab/lispm-agent](https://github.com/LispMachineLab/lispm-agent)):

```elisp
(use-package lispm
  :vc (:url "https://github.com/LispMachineLab/lispm-agent" :rev :newest)
  :config (lispm-mode-line-mode 1))
```

### LispM Desktop

Run the full LispM desktop—including Symbolics Open Genera 2.0, Mac OS 9 Platinum & Dracula, and
Windows 3.2 ([/desktop/](/desktop/))—in your own Emacs 30.1 or later with SVG, `git` and `curl` on
macOS or GNU/Linux
([github.com/LispMachineLab/lispm-desktop](https://github.com/LispMachineLab/lispm-desktop)):

```sh
git clone https://github.com/LispMachineLab/lispm-desktop ~/lispm-desktop
emacs --init-directory ~/lispm-desktop
```

### LispM Browser

Run LispM Browser, the WebKit browser with Vimium keys and Chrome extensions, in an Emacs 30.1 or
later built with xwidgets
([github.com/LispMachineLab/lispm-browser](https://github.com/LispMachineLab/lispm-browser)):

```elisp
(package-vc-install "https://github.com/LispMachineLab/lispm-browser")
(require 'lispm-browser)
(lispm-browser-setup)
```

## Models and API keys

Claude Code and Codex, once installed on your computer, and the LispM App Store's Hermes, Holon and
OpenClaw show beside LispM's own agents in one sidebar. LispM's own agents need only
`OPENAI_API_KEY` or `ANTHROPIC_API_KEY` set in your shell's profile, where `LispM.app` reads it, or
in your own Emacs's environment. For any other local or hosted server speaking the OpenAI Chat
Completions API or Anthropic Messages API over HTTPS, see
[LispM Agent's backends guide](https://github.com/LispMachineLab/lispm-agent#backends).

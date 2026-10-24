LispM's App Store has a new shelf, Classic AI, with three programs on it. Doctor, the ELIZA-style therapist from 1966. SHRDLU, Terry Winograd's blocks world from 1970. And MYCIN, the expert system Stanford built in the 1970s to work out what was causing a patient's infection.

Each one opens in its own window on LispM's classic-style desktop, and each one actually runs. LispM is our Lisp Machine for AI agents. It is built on Emacs, which is why MYCIN and SHRDLU can run their old Common Lisp almost untouched, and why Doctor came with the editor.

![A 67-second film: LispM in 2026 rewinds to 1966, then one take of the LispM desktop with Doctor, SHRDLU and MYCIN at work, and a fast-forward back to 2026.](/assets/blog/three-old-ais-running-again/film.mp4 "The Classic AI film, 67 seconds, with sound.")

## Doctor (1966)

Joseph Weizenbaum wrote ELIZA at MIT in the mid-1960s, in MAD-SLIP, on an IBM 7094. Its best-known script, DOCTOR, played a Rogerian therapist: find a key word in what you typed, turn "I" into "you", and hand it back as a question. His secretary, who had watched him build it, reportedly asked him to leave the room so she could talk to it alone. Weizenbaum spent much of the rest of his career arguing that we shouldn't hand human judgement to machines.

The Doctor in LispM isn't Weizenbaum's code. That would need a 7094. GNU Emacs has shipped its own doctor.el since the mid-80s, written in Emacs Lisp and working the same way, so this one came with the editor. We gave it a window and a New Session button, and made a single Return send your line (the stock one wants two).

This is what it said to us today:

```
I feel lonely when the computer is off.
Are you lonely often?
My friends say I am addicted to my phone.
Why do you say that?
I think the chatbots understand me.
Earlier you said you felt lonely?
You sound like my mother.
Tell me something about your family.
```

"Earlier you said" is the trick people remember. It keeps a short list of things you mentioned and pulls one out when nothing else matches. There is no model of you anywhere in it, and it still keeps a conversation going for longer than you'd expect.

![The Doctor window on the LispM desktop. Four lines typed to it and its four answers, ending with "Tell me something about your family."](/assets/blog/three-old-ais-running-again/doctor.webp)

## SHRDLU (1970)

Winograd wrote SHRDLU at MIT between 1968 and 1970, in Lisp and Micro-Planner on a PDP-6. You talk to it in English about a table with some blocks, pyramids and a box, and it moves them with a hand. Tell it to pick up a big red block and it first takes the green block off the top. Tell it to grasp "the pyramid" when there are three, and it says it doesn't know which pyramid you mean. Ask what's in the box and it looks.

What runs in LispM is the Common Lisp port from the 1990s, which Tom Sgouros revived in 2014. Our fork adds a few fixes, each checked against the 1972 code. It isn't the whole program. Some words from Winograd's published dialogue are missing (it doesn't know "own" or "mine"), and it no longer understands some of the later questions, like when it picked something up. The App Store page says so.

The part we added is the drawing. SHRDLU plans a whole command before it moves anything, so LispM reads the moves it reports and animates the hand: up, across, down, carrying whatever it grabbed.

![The SHRDLU window. The dialogue on top, a drawing of the table below, with the hand putting a blue block into the box.](/assets/blog/three-old-ais-running-again/shrdlu.webp)

## MYCIN (1970s)

MYCIN was Edward Shortliffe's doctoral work at Stanford, with Bruce Buchanan, Stanley Cohen and others. It asked a doctor about a patient with a serious infection and the cultures taken, named the bacteria most likely to blame, each with a certainty factor, and recommended antibiotics. At any question you could type WHY, and it showed the rule it was trying and what it already knew.

In a blinded study published in JAMA in 1979, eight infectious-disease experts judged its antibiotic choices for ten meningitis cases acceptable about 65% of the time. The five faculty specialists it was compared with scored between 42.5% and 62.5%. It was never used on patients.

LispM runs the version Peter Norvig rebuilt for Paradigms of Artificial Intelligence Programming (1992): the EMYCIN shell and six of the rules, enough to tell five kinds of bacteria apart. It's a teaching copy, not a medical tool.

![The MYCIN window, showing the rule it was trying when asked why, and its findings: Enterobacteriaceae 0.800, Pseudomonas 0.760.](/assets/blog/three-old-ais-running-again/mycin.webp)

Every app in the store now comes with a page like this one: what it does, where it comes from, who wrote it and under which license.

![SHRDLU's page in the LispM App Store, with a pixel-art banner and a description.](/assets/blog/three-old-ais-running-again/app-store-shrdlu.webp)

## Why bother

These three get mentioned all the time in arguments about AI agents, mostly by name. Doctor is the shortest demonstration of how little it takes for software to seem to listen. SHRDLU took plain English and acted on a small world, which is roughly what an agent with tools does now. MYCIN showed its reasoning whenever you asked, and that is still what we want from agents. Trying them tells you more than reading about them.

Classic AI is in the LispM App Store from version 0.9.0. LispM is open source (AGPL), and on a Mac it installs with one line:

```sh
curl -fsSL https://lispm.ai/install.sh | sh
```

Then open the App Store and pick Classic AI. More about LispM at lispm.ai.

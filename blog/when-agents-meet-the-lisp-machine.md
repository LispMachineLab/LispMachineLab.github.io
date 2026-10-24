The extraordinary boom of LLMs today reminds me of the Lisp Machine of the 1970s, the crystallization of the previous golden age of artificial intelligence, once the object of the highest hopes and now, like Atlantis, a lost ruin. The Lisp Machine can be said to embody the ultimate idea of the last AI boom, Code as Data, code as logic and logic as data, in the hope that logic would deduce itself, unfold itself and flourish by itself. How closely it matches today’s Transformer-based LLMs, and one could even say it is a shade more elegant.

Of course, owing to the barber paradox, the Lisp Machine failed: logic cannot, by referring to itself, complete an introspection of its own order, just as today’s LLMs cannot complete their own reduction without input from outside. In an age poor in both computing power and theory, such failure was inevitable, yet at least, 50 years ago, humanity had already attempted to build its Tower of Babel, in pursuit of the realm of God.

## Emacs Is the Last Relic of the Lisp Machine

The attacks on Emacs and the praise of it converge on a single point: it tries to encompass everything, to the point of becoming “an operating system disguised as an editor”. Although this is thoroughly un-Unix, it answers the ultimate expectations people held of the Lisp Machine and of AGI: give it enough functions and enough possibilities, and emergence may follow. Hence the Emacs we see today has a built-in webkit browser, built-in mail, a built-in todo list, and plugins for everything, as if it were a seedbed that had waited 50 years, prepared expressly for Agents.

The philosophy of Emacs is never to step outside its frame, not even to call the operating system, for everything is done here. Some Emacs zealots even claim that its built-in Org-mode can manage everything in life.

Hmm, how very Agent, and that is why I built this project today. The Lisp machine for Agents.

[github.com/LispMachineLab/lispm-agent](https://github.com/LispMachineLab/lispm-agent)

## Introspection Is the Most Important Step of a Harness

The Agent frameworks of today are mostly devoted to preparing tools for the model, and the list of tools in the prompt is the whole world of the Agent; whatever lies outside that list does not, as far as it is concerned, exist. Humans, however, do not come to know the world this way: faced with an unfamiliar system, the first thing one asks is what it is, and what it can do. Introspection is therefore the most important step of a Harness, for the ceiling of an Agent lies not in how many tools it has, but in the boundaries of the environment it inhabits.

In this respect, Emacs is almost unique. It calls itself the “self-documenting” editor: every function carries its own documentation, every variable can be inspected on the spot, and even a veteran of 20 years still relies on asking Emacs in order to know Emacs. In Emacs, therefore, an Agent needs to learn only the same thing: ask first, then read, and only then act.

Of course, the barber paradox remains, and an Agent cannot complete a full introspection of its own order. A Harness, however, never needed complete introspection; it needs only to make each step of introspection cheap enough, and to keep that bit of “outside force” in the loop, and that outside force is the human.

## You Only Need to Expose Emacs Itself

The prevailing approach to building Agents is one of addition: browsing the web calls for a browser tool, reading mail for a mail tool, and the more tools there are, the fuller the context, and the more lost the model becomes. Emacs, however, finished all of this over the past 40 years: browser, mail, calendar and terminal are, as far as Emacs is concerned, not isolated programs but functions within one and the same Lisp image. We therefore need not wrap Emacs for the Agent; we need only expose Emacs itself to it.

This was also my most unexpected discovery while building this project: I invented almost nothing. The context is an org document, a tool is a Lisp function, an approval is the state of a todo item, memory is org files in git, and rollback is undo; every concept of a Harness already has its counterpart in Emacs, and all that was missing was a new name.

As for the universal interface, Unix has the text stream, in which every program is a new dialect, whereas Emacs has Lisp, one language and one world. For a language model, this is without doubt the more fitting home.

## A Step Further: Lambda Calculus and Turing Completeness

These days the essence of an Agent is ordering from a menu, and tools are its menu; an Agent’s capability is the length of that menu. When an Agent is allowed to execute Lisp directly, however, its nature changes: it no longer chooses from a finite set of actions but writes the action itself, and the space of actions expands from a single list to the set of all Lisp programs, which is therefore Turing complete. It can write a tool for itself and use it at the very next step; self-deduction, self-unfolding, the dream of 50 years ago here becomes concrete for the first time.

Lambda calculus tells us that all computation is nothing more than the definition and application of functions, and Lisp is the language closest to lambda. Every Lisp programmer knows that oldest of loops, Read, Eval, Print, Loop, and the Agent loop of today is merely its reprint, except that the one who performs Read has changed from a human to a model.

Of course, Turing completeness does not come without a price. The halting problem and the barber paradox are the same diagonal blade, and a Turing-complete system cannot prove itself; every execution by an Agent therefore still requires a human’s approval.

The Tower of Babel fell because God confounded the language of humankind. In Emacs, however, humans and Agents speak the same language.

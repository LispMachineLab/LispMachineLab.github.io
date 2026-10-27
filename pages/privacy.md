*Effective 28 October 2026.*

This policy covers the lispm.ai website and LispM Mail, the mail app of LispM. LispM is
made by [LispMachineLab](https://github.com/LispMachineLab); write to
[help@lispm.ai](mailto:help@lispm.ai) with any question about it.

## In short

LispM runs on your computer. LispM Mail keeps your mail, and what signs you in, on your
computer, and sends your mail only to your mail provider's servers. LispMachineLab runs no
server that receives your mail, and never sees it.

## LispM Mail

### What it reads and does

When you add an account, LispM Mail logs in to your provider's mail servers, IMAP to read and
SMTP to send, with your password or, for Gmail and Outlook.com, with the token their own
sign-in page gives it. From Google it asks for one permission, `https://mail.google.com/`,
the only one with which a mail program can read and send mail over IMAP and SMTP. With it,
LispM Mail lists your mailboxes and messages and shows them; sends the messages you write;
keeps your drafts and sent mail in their mailboxes; and marks, moves or deletes messages when
you tell it to.

### Where it keeps it

- Messages and mailbox lists it has fetched are kept in a folder on your computer, `lispm-mail`
  in Emacs's own directory, so that they show at once and offline.
- Messages written while offline wait in the same folder until they can be sent.
- Your password, or the token that signs your account in again, is kept in the macOS Keychain;
  on other systems, where your Emacs keeps secrets (auth-source).
- The token that logs in for an hour is held in memory only, never written down.

### Who it shares it with

No one. Your mail goes only between your computer and your provider's servers, encrypted.
LispM Mail sends nothing to LispMachineLab, shows no ads, sells nothing, and does not use your
mail to train AI models. It makes two other connections, and neither carries your mail:

- to fill in the servers of a provider it does not know, it asks Thunderbird's database of
  providers (autoconfig.thunderbird.net) about the domain of your address, the part after @,
  and nothing else;
- pictures a message links to on the web load only when you ask, or when you turn on *Load
  pictures from the web* in its settings; the servers holding them then see your computer's
  address, as with any picture on the web.

### Agents

LispM is a harness for AI agents, and LispM Mail gives no message to an agent on its own. If
you ask an agent in LispM to work with your mail, it reads what you point it to and sends that
to the AI model you chose for it, as you asked, under that model provider's terms.

### Removing it

Removing an account in LispM Mail's settings deletes its sign-in and the mail kept of it on
your computer. To take back LispM Mail's access to your Google account, go to
[myaccount.google.com/permissions](https://myaccount.google.com/permissions); to your Microsoft
account, [account.live.com/consent/Manage](https://account.live.com/consent/Manage).

### Google user data

LispM Mail's use and transfer to any other app of information received from Google APIs will
adhere to the
[Google API Services User Data Policy](https://developers.google.com/terms/api-services-user-data-policy),
including the Limited Use requirements.

## The lispm.ai website

The site is pages served by GitHub Pages. It sets no cookies, runs no analytics or trackers,
and loads nothing from other sites: its pictures and films come from lispm.ai itself. GitHub,
which serves it, sees the address your browser connects from, as any web server does; see
[GitHub's privacy statement](https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement).

## Children

LispM and this site are not meant for children under 13, and collect nothing about anyone.

## Changes

A new version of this policy is posted on this page, with its date. A change to what LispM
Mail does with your mail is posted here before an update of LispM Mail makes it.

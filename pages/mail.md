LispM Mail puts your mail accounts in one classic-style window: a folder list with the unread
counts, a table of messages you can sort by any column, and the message you choose under it,
in its box of headers. Replies and new mail are written in their own window and sent in the
background.
It is made of the mail Emacs has always had, imap.el, smtpmail and message mode, and it runs
on your computer.

## Your accounts

Type your address, and LispM Mail fills in its servers.

- **Gmail and Google Workspace** sign in on Google's own page, in your browser. LispM Mail
  never sees your Google password.
- **iCloud, Yahoo, Fastmail, QQ Mail, NetEase** and other providers log in with the password
  their settings make for mail programs.

*Test Connection* logs in to both servers and says how it went.

## Private by design

Your mail goes only between your computer and your provider, and stays on your computer. LispM
Mail has no server of its own, sends nothing to us, shows no ads, and loads pictures from the
web only when you ask. Removing an account deletes its sign-in and the copy of its mail on your
computer. The [privacy policy](/privacy/) says exactly what LispM Mail reads, keeps and sends.

### What it asks of Google

When you sign in with Google, LispM Mail asks to read, compose, send and delete your Gmail:
the one permission with which a mail program reads and sends mail over IMAP and SMTP. It uses
it only to show your mail and send what you write, in LispM Mail on your computer.

## Get LispM Mail

In LispM, open the App Store and install **Mail**. It is free software, under the GNU AGPL,
and its source is at [github.com/LispMachineLab/lispm-mail](https://github.com/LispMachineLab/lispm-mail).

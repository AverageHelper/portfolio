---
title: "How to move a Git repo from GitHub to Forgejo"
description: "Move your code someplace you control, and leave a read-only code mirror behind."
date: "2024-10-09"
---

Move your code someplace you control, leaving a read-only code mirror behind:

1. Choose a new home for your code.
2. Close what you can of GitHub's copy.
3. Set up a GitHub Actions job to auto-close new GitHub PRs and redirect contributors to your new forge. (Doing this now because GitHub rejects Actions pushed from mirrors)
4. Use Forgejo's Migration UI to pull from GitHub.
5. Use Forgejo's Push Mirror UI to push new code to GitHub.
6. Update your local copy to point at the new forge.
7. Set up backlinks between GitHub and your forge.
8. Update references to your repository in your code.

## Step 0: Choose a forge

There are lots of competent Git hosting software (or "forges") out there:

- ...

I chose Forgejo.

Codeberg is sortof the "flagship" option for hosted Forgejo instances. If you're used to GitHub Actions for CI/CD, and don't want to port to Woodpecker instead, expect to host your own Forgejo Actions runner.

There's also [git.gay](https://git.gay), which seems competently run, tho I am biased. :3

I run my own small Forgejo instance at [git.average.name](https://git.average.name). (At the time of writing, registrations for my instance are open, but intended mainly to facilitate contributions to existing projects there. Once federation is ready, I'll likely turn registrations off entirely.)

## Step 1: Prepare GitHub's copy

- shut off unneeded units (projects, wiki)
- best if not relying too much on github-specific spaces like Discussions
- leave Actions on

## Step 2: Prepare access tokens in GitHub an Forgejo

- These facilitate mirror pushes from Forgejo to GitHub

## Step ?: ???

## Why leave GitHub?

- They're closed-source, which feels cheeky.
- Microsoft is known for Embrace, Extend Extinguish ideology.
- Their latest investment in the "AI" hype train, ignoring code licenses and automatically hoovering up every public codebase for training ~~liar machines~~ LLMs.
- I want more control over where my code lives.
- I want to learn more about how Git works.
- GitHub's market share causes people to confuse Git and GitHub.
- Healthy market competition.
- For fun! :3

## Why leave a code mirror?

Other people's convenience. If you've been on GitHub long enough, there's a good chance someone will think to look for you there, or reference your code from there in some way. Leaving the repo up on GitHub, with backlinks and source-level pointers to the new forge, serve as effective signposts without breaking existing systems.

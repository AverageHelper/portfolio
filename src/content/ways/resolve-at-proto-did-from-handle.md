---
title: "How to resolve an AT Proto DID from a handle or username"
description: "In case you want to link to a user profile in a way that can't break if their handle changes."
date: "2024-11-04"
---

In case you want to link to a user profile in a way that can't break if their handle changes, you might consider constructing the URL using the user's stable DID instead of their current handle.

For example, you might link to my profile ([@average.name](https://avg.average.name)) using https://bsky.app/profile/average.name or https://bsky.app/profile/did:plc:zxthjxcmxpjl372uwgrm6dxi (or using some other AT Proto client, of course).

To resolve a user's DID, we can call the `com.atproto.identity.resolveHandle` REST API on any AT Proto data server. For example, calling against Bluesky's:

```sh
curl -s 'https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle=average.name'
```

This responds with a JSON payload that contains my DID.

We can parse the value out nicely using `jq`, strip the quotation marks using `tr`, and even make a shell command from this like so:

```sh
function did() {
	url='https://bsky.social/xrpc/com.atproto.identity.resolveHandle?handle='"$1"
	curl -s $url | jq .did | tr -d '"'
}
```

Then use it like so:

```sh
did average.name
```

## Bluesky alternatives

You don't have to call against bsky.social's API for this! In my own setup, I call against my own AT Proto PDS (also at https://average.name). Feel free to use your own! (You can probably call other people's PDS as well, but please don't be rude ;)

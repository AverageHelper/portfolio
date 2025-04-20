---
title: "How to change your User-Agent string on Firefox or Firefox-based browsers"
description: ""
date: "2024-10-16"
---

1. `about:config`
2. Dismiss warning
3. Create `general.useragent.override` as a String
4. Set the value to whatever you want as your `User-Agent` header text.

Some useful ones:

- Some websites may behave differently based on your User Agent string, sometimes in an effort to get you to use Chrome or a Chromium-based browser. If they check the UA string for this, you can get around that this way.
- For example: music.apple.com uses a different link to iTunes on Windows agents and Linux vs Mac. ([Example](https://music.apple.com/us/album/oh-%E3%82%B9%E3%82%B1%E3%83%88%E3%83%A9-%E3%83%A6%E3%83%BC%E3%83%AA-on-ice-%E3%82%AA%E3%83%AA%E3%82%B8%E3%83%8A%E3%83%AB-%E3%82%B9%E3%82%B1%E3%83%BC%E3%83%88%E3%82%BD%E3%83%B3%E3%82%B0collection/1184259235))

## Step ??

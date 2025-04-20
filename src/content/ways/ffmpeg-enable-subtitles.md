---
title: "How to enable subtitles by default on a MKV video"
description: "If you happen to have a .mkv file with a subtitle track, and you want that track to be enabled or selected (e.g. in VLC) by default, without having to select it again every time FFMPEG has you covered."
date: "2024-11-19"
---

If you happen to have a .mkv file with a subtitle track, and you want that track to be enabled or selected (e.g. in VLC) by default, without having to select it again every time FFMPEG has you covered:

```sh
ffmpeg -i in.mkv -c copy -disposition:s:0 default out.mkv
```

This makes a copy of your input file (`in.mkv`) while setting the subtitle stream (the `:s:` part in the `disposition` flag) to play by default. The result will be `out.mkv`.

---

I learned this information from [this StackOverflow answer](https://stackoverflow.com/a/35235287).

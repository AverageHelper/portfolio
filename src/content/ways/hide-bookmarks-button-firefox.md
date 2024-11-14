---
title: "How to hide the Add Bookmark button from Firefox's URL bar"
description: "If you use an external Bookmarks manager (such as Linkwarden or Raindrop.io), you may want to hide the default Add Bookmark button (the Star) from your URL bar."
date: "2024-11-14"
---

If you use an external Bookmarks manager (such as [Linkwarden](https://linkwarden.app/) or [Raindrop.io](https://raindrop.io/)), you may want to hide the default Add Bookmark button (the Star) from your URL bar.

First, [create a userChrome.css file](https://www.userchrome.org/how-create-userchrome-css.html).

Then, add the following somewhere underneath the `@namespace` line:

```css
#star-button-box {
	width: 0.1px !important;
	overflow: hidden !important;
	padding-inline: 0 !important;
}
```

[Other sources](https://old.reddit.com/r/firefox/comments/24hygu/how_do_you_get_rid_of_the_bookmark_star/ch7g0h9/) suggest setting [`display: none`](https://developer.mozilla.org/en-US/docs/Web/CSS/display#none), [or](https://support.mozilla.org/en-US/questions/1009385) [`visibility: collapse`](https://developer.mozilla.org/en-US/docs/Web/CSS/visibility#collapse), but these also hide the Add Bookmark dialog box (e.g. what comes up when you press Ctrl+D), which then causes bookmarks to be added immediately, which defeats the point. The above solution retains the ability to cancel creating the bookmark accidentally, **as of Firefox 132**.

---

I learned this information from [this Reddit comment](https://old.reddit.com/r/FirefoxCSS/comments/zqy2vr/any_way_to_detach_add_bookmark_dialog_from_the/j10mk5c/).

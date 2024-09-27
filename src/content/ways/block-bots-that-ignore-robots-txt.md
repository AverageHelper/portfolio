---
title: "How to block bad bots (that ignore robots.txt) using Caddy"
description: "The robots.txt file is a standard way to request that specific bots not scrape your site. But some bots are said to ignore that request and scrape anyway."
date: "2024-09-26"
---

The [robots.txt](https://www.robotstxt.org/robotstxt.html) file is a standard way to request that specific bots not scrape your site. But some bots [are said to ignore that request](https://www.tomshardware.com/tech-industry/artificial-intelligence/several-ai-companies-said-to-be-ignoring-robots-dot-txt-exclusion-scraping-content-without-permission-report) and scrape anyway.

In order to avoid being scraped, smaller webservers may wish to resort to more creative measures.

## Assumptions

This document assumes you're serving your site using [Caddy 2.x](https://caddyserver.com/), and configuring Caddy using a [Caddyfile](https://caddyserver.com/docs/caddyfile). For example, my Caddyfile looks something like this:

```caddy
average.name {
	reverse_proxy :8080
}
```

In short, this tells Caddy to enforce HTTPS for the `average.name` domain, and to manage a reverse proxy to another local HTTP webserver running adjacent to Caddy on port 8080. There are other reverse proxy softwares out there that can do the same thing, but I use Caddy and so does this document.

Be sure to use **your own domain name** in place of `average.name` for the purpose of this tutorial.

## Step 0: Have a robots.txt file

The first step, of course, is to politely request that certain bots not scrape your site. If they respect that request, then your webserver can avoid doing some extra work!

[My site](/robots.txt) serves a robots.txt file that borrows heavily from [the one at seirdy.one](https://seirdy.one/robots.txt). (You might consider borrowing from [Codeberg's](https://codeberg.org/robots.txt) robust one as well.) I would like bots to respect this file. Unfortunately, [some are known not to do that](https://www.theverge.com/2024/7/25/24205943/anthropic-ai-web-crawler-claudebot-ifixit-scraping-training-data). So, as a fallback, we'll take a more heavy-handed approach.

## Step 1: Define a Regular Expression (Regex) that lists the bad bots

This expression is constructed from the list of `User-Agent` entries in my robots.txt file which have `Disallow: /` set:

```
Adsbot|peer39_crawler|TurnitinBot|NPBot|SlySearch|BLEXBot|CheckMarkNetwork|BrandVerity|PiplBot|MJ12bot|ChatGPT-User|GPTBot|Google-Extended|Applebot-Extended|Claude-Web|anthropic-ai|ClaudeBot|FacebookBot|meta-externalagent|AI2Bot|Amazonbot|Bytespider|cohere-ai|Diffbot|facebookexternalhit|FriendlyCrawler|ICC-Crawler|ImagesiftBot|img2dataset|OAI-SearchBot|Omgili|Omgilibot|PerplexityBot|PetalBot|Scrapy|Timpibot|VelenPublicWebCrawler|YouBot
```

These I've asked politely [in robots.txt](/robots.txt) not to crawl my site at all. If they proceed anyway, we'll have a special treat for them >:3

## Step 2: Define a matcher

In your Caddyfile, inside your site block, construct a [named matcher](https://caddyserver.com/docs/caddyfile/matchers#named-matchers). Use [`header_regexp`](https://caddyserver.com/docs/caddyfile/matchers#header-regexp) to match requests whose [`User-Agent`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) header matches your regex from Step 1. The matcher should also omit the `/robots.txt` path specifically, as we still want to serve our polite request to bad bots.

```caddy
average.name {
	@badrobots {
		# Bots that self-report with one of these User-Agent strings are matched:
		header_regexp User-Agent Adsbot|peer39_crawler|TurnitinBot|NPBot|SlySearch|BLEXBot|CheckMarkNetwork|BrandVerity|PiplBot|MJ12bot|ChatGPT-User|GPTBot|Google-Extended|Applebot-Extended|Claude-Web|anthropic-ai|ClaudeBot|FacebookBot|meta-externalagent|AI2Bot|Amazonbot|Bytespider|cohere-ai|Diffbot|facebookexternalhit|FriendlyCrawler|ICC-Crawler|ImagesiftBot|img2dataset|OAI-SearchBot|Omgili|Omgilibot|PerplexityBot|PetalBot|Scrapy|Timpibot|VelenPublicWebCrawler|YouBot

		# The matcher does not catch if the request is for robots.txt:
		not path /robots.txt
	}

	# ...
}
```

## Step 3: Define behavior for bad bots

Now, use the matcher somewhere. This example uses the [`respond`](https://caddyserver.com/docs/caddyfile/directives/respond) directive to tell Caddy to serve only the string `:3` to bad bots.

```caddy
average.name {
	@badrobots {
		# Defined in Step 2...
	}
	respond @badrobots ":3"

	# ...
}
```

Alternatively, you might consider using the [`redir`](https://caddyserver.com/docs/caddyfile/directives/redir) directive to [redirect bots to some very large file hosted elsewhere](https://noise.j-w.au/@j/113190172515240794). It's up to you what you do.

## Result

If you've configured Caddy correctly, then normal users will get normal website:

```sh
curl https://average.name/
<!DOCTYPE html>
...
```

And bots will get silliness:

```sh
curl https://average.name/ -A "GPTBot"
:3
```

These bots may avoid silliness by reading and respecting your robots.txt file:

```sh
curl https://average.name/robots.txt -A "GPTBot"
User-agent: *
Disallow: /api/*
...
```

## Disclaimers

Unfortunately, this method only works when scrapers reliably self-report their User-Agent string consistently. Some sneaky ones might send a different string, or not send one at all.

## Reusable Snippet

If your Caddyfile defines multiple websites, you might consider wrapping your bot-blocking logic in a [snippet](https://caddyserver.com/docs/caddyfile/concepts#snippets) and reusing with [`import`](https://caddyserver.com/docs/caddyfile/directives/import), rather than defining the matcher in each server block:

```caddy
# Robots that ignore robots.txt get a fun treat :3
(block_bad_bots) {
	@badrobots {
		# We ask these bots in robots.txt not to proceed
		header_regexp User-Agent Adsbot|peer39_crawler|TurnitinBot|NPBot|SlySearch|BLEXBot|CheckMarkNetwork|BrandVerity|PiplBot|MJ12bot|ChatGPT-User|GPTBot|Google-Extended|Applebot-Extended|Claude-Web|anthropic-ai|ClaudeBot|FacebookBot|meta-externalagent|AI2Bot|Amazonbot|Bytespider|cohere-ai|Diffbot|facebookexternalhit|FriendlyCrawler|ICC-Crawler|ImagesiftBot|img2dataset|OAI-SearchBot|Omgili|Omgilibot|PerplexityBot|PetalBot|Scrapy|Timpibot|VelenPublicWebCrawler|YouBot

		# Always send robots.txt, even to bad bots
		not path /robots.txt
	}

	respond @badrobots ":3"
}

average.name {
	import block_bad_bots

	reverse_proxy :8080
}
```

For best results, be sure to only do this to bots that are actually named in all of your webservers' robots.txt files, otherwise your webserver will be rude to nice bots and do extra work!

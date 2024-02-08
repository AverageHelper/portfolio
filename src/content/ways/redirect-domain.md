---
title: "How to redirect to a new domain on Cloudflare"
description: "If you own two domains, and want one to redirect cleanly to the other while using Cloudflare, the configuration is complex."
date: "2024-02-07"
---

If you own two domains, and want one to redirect cleanly to the other while using Cloudflare, the configuration is complex.

Say, for example, you want requests to redirect as follows:

- `avg.name` to `average.name`
- `foo.avg.name` to `foo.average.name`
- `bar.avg.name/baz` to `bar.average.name/baz`
- etc.

First, ensure both domains are listed in your [Cloudflare Dashboard](https://dash.cloudflare.com) under "Websites". The **target domain** (e.g. `average.name`) should already be configured with appropriate DNS for its subdomains.

Next, for clients to attempt to resolve user requests, your **alias domain** (e.g. `avg.name`) must have some DNS records. Configure the **alias domain** with two `CNAME` records:

- `CNAME @ average.name` – point **root** to the target domain
- `CNAME * average.name` – point **all subdomains** to the target domain

Finally, in the "Rules" section of your **alias domain**, find the "Redirect Rules" section. Create a rule as follows:

- When: any incoming request's Hostname ends with the **alias domain** (e.g. the expression `(ends_with(http.host, "avg.name"))`),
- Then: run a Dynamic URL redirect with the expression `concat("https://", substring(http.host, 0, -8), "average.name", http.request.uri)`.
  - Replace the `-8` value with the length of your **alias domain**.
- Status code as appropriate (e.g. `302`).
- Enable "Preserve query string".
- Name the rule something appropriate like "Redirect to main".

Then deploy the new rule.

After a few moments, all requests at your **alias domain** will redirect as expected.

You can do other transformations in the Target Expression field if you need. You'll find the reference documentation for the available [Fields](https://developers.cloudflare.com/ruleset-engine/rules-language/fields/) and [Functions](https://developers.cloudflare.com/ruleset-engine/rules-language/functions/) on Cloudflare's site.

---
title: "How to prevent Docker from bypassing your firewall on Linux"
description: "If you're running a service using Docker that you want open to the internet via a reverse proxy, and not via Docker's open port, you'll need to disable Docker's default IPTables behavior."
date: "2024-04-06"
---

If you're running a service using Docker that you want open to the internet via a reverse proxy, and not via Docker's open port, you'll need to disable Docker's default IPTables behavior.

For example, consider a web service that:

- uses Caddy to serve HTTPS on port 443,
- uses Docker to run an HTTP service on port 3000,
- has Caddy configured to serve a reverse proxy to that Docker service, and
- has `ufw` configured to block all connections except on ports 22 and 443.

By default, Docker automatically opens port 3000 anyway, making your internal _HTTP_ service available on the web on port 3000!

Here's how to fix that:

First, add this line to your `/etc/default/docker` file:

```sh
DOCKER_OPTS="--iptables=false"
```

(You may need to modify the existing `DOCKER_OPTS` if one is already configured.)

Then set `"iptables"` to `false` in your `/etc/docker/daemon.json` file. If that file does not exist, you can create it such that it looks something like this:

```json
{ "iptables": false }
```

Finally, restart Docker

```sh
sudo service docker restart
```

Now, you're free to use `ufw` or your firewall software of choice to manage your system's open ports, and plug your reverse proxy of choice as you like!

---

I learned this information from [this StackOverflow answer](https://stackoverflow.com/a/73416641).

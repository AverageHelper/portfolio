---
title: "How to configure `dict` to run locally on macOS"
description: "Set up and use a standard Dictionary Server Protocol (RFC2229) client and server locally"
date: "2024-10-09"
---

## Step 1: Install the `dict` client

Using [Homebrew](https://brew.sh/): `brew install dict`

## Step 2: Configure `dictd`

Create a new file at `/opt/homebrew/etc/dictd.conf` that contains the following:

```conf
# Allow only local access
access {
    allow 127.0.0.1
    deny *
}

# Dictionaries:
database wn {
    data /usr/local/share/dictd/wn.dict.dz
    index /usr/local/share/dictd/wn.index
}
database gcide {
    data /usr/local/share/dictd/gcide.dict.dz
    index /usr/local/share/dictd/gcide.index
}
database moby-thesaurus {
    data /usr/local/share/dictd/moby-thesaurus.dict.dz
    index /usr/local/share/dictd/moby-thesaurus.index
}
```

This config includes references to three dictionary database entries, which we will install in the next step. See the [man page](https://man.archlinux.org/man/dictd.8.en) for more info on the config syntax and options.

## Step ??: Download some dictionaries

### WordNet (dict-wn)

1. [Download](https://packages.ubuntu.com/focal/all/dict-wn/download) the [dict-wn](https://packages.ubuntu.com/focal/all/dict-wn) Ubuntu package manually.
2. Extract the `dict-wn_3.0-36_all.deb` package: `ar -x dict-wn_3.0-36_all.deb`
3. Extract the `data.tar.xz` archive: `tar -xf data.tar.xz`
4. In the new `usr/share/dictd` directory, copy the `wn.dict.dz` and `wn.index` files into `/usr/local/share/dictd`.
5. You may get rid of the Ubuntu files at this point, you only need the dictionary index and data files.

### Comprehensive English Dictionary (dict-gcide)

Repeat the above steps for the [dict-gcide](https://packages.ubuntu.com/focal/dict-gcide) Ubuntu package.

Decompressing `data.tar.zst` may require a slightly different command: `tar --zstd -xvf data.tar.zst`

### Moby Thesaurus (dict-moby-thesaurus)

Repeat the above steps for the [dict-moby-thesaurus](https://packages.debian.org/buster/dict-moby-thesaurus) Debian package.

## Start the server

With your dictionary files in place, start the daemon:

```sh
sudo dictd
```

The server should now be listening locally on port `2628`.

## Restart the server

After updating the config, you may need to restart. First, find the process ID using `sudo lsof -i -P | grep LISTEN | grep :2628`

Then, kill the process directly. For example, if the above command gave you something like:

```
dictd  59267  nobody  [...]
```

Then you can end the process with: `sudo kill 59267`

Then restart the daemon as above.

Consider also [these](https://web.archive.org/web/20140917131745/http://abloz.com/huzheng/stardict-dic/dict.org/).

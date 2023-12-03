# portfolio

A basic web site to plug my projects and things, accessible at https://average.name.

Feel free to poke around, I guess.

# Usage

We use [Nunjucks](https://mozilla.github.io/nunjucks) to generate static HTML from templates.

## Install Dependencies

We'll need these for our build tools.

```sh
$ npm ci
```

## Make changes

The contents of the `/templates` directory get compiled into static HTML assets.

The rest of the site lives in `/pages` verbatim.

## Build the site

Our output

```sh
$ npm run build
```

## Preview the site

This command will build the site before starting a local Wrangler server to emulate Cloudflare Pages:

```sh
$ npm start
```

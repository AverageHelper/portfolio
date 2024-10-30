# portfolio

A basic web site to plug my projects and things, accessible at https://average.name.

Feel free to poke around, I guess.

# Usage

We use [Astro](https://astro.build) to generate static HTML from templates. Astro runs on Node, and our webserver uses Deno. Make sure both of these are installed on your system.

## Install Dependencies

We'll need these for our build tools.

```sh
deno install --frozen
npm ci
```

## Make changes

The contents of the `/src` directory gets compiled into static HTML assets.

The contents of the `/public` directory get copied into the output folder as-is.

The build result lives in `/dist`, and gets sent verbatim to the web host. Please do not modify these files directly.

The `/functions` directory contains the back-end logic, including serving static files and responding to WebFinger requests.

Use the following command to build and run a live development webserver:

```sh
deno task dev
```

The webserver will restart whenever files it depends on have changed. Live browser reload is not yet implemented.

## Build the site

This command will build static site assets to `/dist` and download runtime dependencies, without starting a webserver:

```sh
deno task build
```

## Run unit tests

To run unit tests and make sure everything is working as expected:

```sh
deno task test
```

## Run the site with Deno

This part is mainly for my own notes. Go run your own website! lol

After the site is built, this command will run a production-ready webserver:

```sh
deno task start
```

To run in the background as a daemon, use [`pm2`](https://pm2.keymetrics.io/docs/usage/quick-start/) like so:

```sh
pm2 start ./app.sh --name portfolio
```

The app will run on port `8787`.

## Run the site with Docker Compose

This part is mainly for my own notes. Go run your own website! lol

```sh
docker compose up -d
```

The app will run on port `8787`. You can modify that in [`compose.yaml`](compose.yaml) or use a `docker run` command instead.

## Contributing

This project lives primarily at [git.average.name](https://git.average.name/AverageHelper/portfolio). Read-only mirrors also exist on [Codeberg](https://codeberg.org/AverageHelper/portfolio) and [GitHub](https://github.com/AverageHelper/portfolio). Issues or pull requests should be filed at [git.average.name](https://git.average.name/AverageHelper/portfolio). You may sign in or create an account directly, or use one of several OAuth 2.0 providers.

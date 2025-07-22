# portfolio

A basic web site to plug my projects and things, accessible at https://average.name via HTTPS or gemini://average.name via [Gemini protocol](gemini://geminiprotocol.net).

Feel free to poke around, I guess.

# Contributing

We use [Astro](https://astro.build) to generate static HTML from templates, which we serve statically using [Rocket.rs](https://rocket.rs/). Astro runs on Node, and our webserver uses Rust. Make sure both of these are installed on your system in order to build the webserver.

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

## Build the site

This command will build static site assets to `/dist` and download runtime dependencies, without starting a webserver:

```sh
deno task build
cargo build --release
```

## Run unit tests

To run unit tests with code coverage, use the `test.sh` script at the directory root. This will prompt you to install the necessary components.

To run unit tests without code coverage, run `cargo test`.

## Run the site with Rust

This part is mainly for my own notes. Go run your own website! lol

After the site is built, this command will run a production-ready webserver:

```sh
cargo run --release
```

By default, the HTTP server runs on port `8787`, and the Gemini capsule on port `1965`.

## Run the site with Docker Compose

This part is mainly for my own notes. Go run your own website! lol

```sh
docker compose up -d --no-deps --build
```

The HTTP servers run on port `8787`, and the Gemini capsule on port `1965`. You can modify these ports in [`compose.yaml`](compose.yaml), use a `docker run` command instead, or set the `HTTP_PORT` and `GEMINI_PORT` environment variables.

## Certificates

### For local development

If you're already developing a Gemini capsule locally, copy those certs into a new `.certs` directory in this project. Otherwise, create TLS certificates like so:

```sh
mkdir -p .certs
cd .certs
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 1103760 -nodes -subj '/CN=localhost' -addext 'subjectAltName=DNS:localhost'
```

Note that the `-days` arg in the command above sets the cert to expire sometime around the year 3024. This is sufficient for use with Gemini capsules or local development, but not much else.

> [!IMPORTANT]
> Change the `/CN=` and `subjectAltName=` parts to your expected internet domain when using in production. Consider also setting a shorter `-days` parameter, and setting a cron job to re-issue a new cert when this one expires.

The `<project>/.certs` directory must contain `key.pem` and `cert.pem`, or the server will panic.

## Contributing

This project lives primarily at [git.average.name](https://git.average.name/AverageHelper/portfolio). Read-only mirrors also exist on [Codeberg](https://codeberg.org/AverageHelper/portfolio) and [GitHub](https://github.com/AverageHelper/portfolio). Issues or pull requests should be filed at [git.average.name](https://git.average.name/AverageHelper/portfolio). You may sign in or create an account directly, or use one of several OAuth 2.0 providers.

# portfolio

A basic web site to plug my projects and things, accessible at https://average.name.

Feel free to poke around, I guess.

# Usage

We use [Astro](https://astro.build) to generate static HTML from templates.

## Install Dependencies

We'll need these for our build tools.

```sh
npm ci
```

## Make changes

The contents of the `/src` directory gets compiled into static HTML assets.

The contents of the `/public` directory get copied into the output folder as-is.

The build result lives in `/dist`, and gets sent verbatim to the web host. Please do not modify these files directly.

The `/functions` directory contains a Worker file that directs dynamic webserver activities, such as responding to WebFinger requests.

Use the following command to run a live webserver:

```sh
npm start
```

## Build the site

This command will build the site and update `/dist`:

```sh
npm run build
```

## Preview the site

This command will build the site to `/dist` and start a local static webserver:

```sh
npm run preview
```

## Contributing

This project lives primarily at [git.average.name](https://git.average.name/AverageHelper/portfolio). Read-only mirrors also exist on [Codeberg](https://codeberg.org/AverageHelper/portfolio) and [GitHub](https://github.com/AverageHelper/portfolio). Issues or pull requests should be filed at [git.average.name](https://git.average.name/AverageHelper/portfolio). You may sign in or create an account directly, or use one of several OAuth 2.0 providers.

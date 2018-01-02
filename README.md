# Drow

__NOTE: DROW IS CURRENTLY IN AN ALPHA STATE, AND SHOULD NOT BE TRUSTED.__

Drow is a work-in-progress static site generator designed for extreme ease of
use. Selling points include:

- It has few features.
- It is not configurable.
- It has an excellent command-line interface.
- It is well documentated.
- It is easy to fork and modify.

If these don't sound appealing to you, there are many other
[static site generators](https://www.staticgen.com/) to try.

## Table of Contents

- [Design Goals](#design-goals)
- [Structure](#structure)
- [Commands](#commands)
    - [`drow start`](#drow-start-directory)
    - [`drow page`](#drow-page-title)
    - [`drow post`](#drow-post-title)
    - [`drow build`](#drow-build)
    - [`drow admin`](#drow-admin)
- [Contributing](#contributing)
- [License](#license)

## Structure

Every Drow site starts with a `drow start`. This copies the base Drow project
template, which looks like this:

```
|- Drow.toml
|- assets/
|- pages/
|- posts/
|- templates/
```

`Drow.toml` is where you put any variables you want your pages, posts, or
templates to be able to use.

`assets/` just has its contents copied into the root of the site during
building; so `assets/css/` becomes just `css/`.

`pages/` is where all the standalone pages for your site go. They must be
markdown files (`.md` extension), and they're transformed like so: `<title>.md`
becomes `<title>/index.html` in the site.

`posts/` is where all the posts for your site go. They must be markdown files
too, and they must be named `<yyyy>-<mm>-<dd>-<title>.md`. They get transformed
like so: `<yyyy>-<mm>-<dd>-<title>.md` becomes
`blog/<yyyy>/<mm>/<dd>/<title>/index.html`.

`templates/` is where all the templates for your site go. Templates can load
other templates, and every page or post _must_ list what template it uses in
a TOML frontmatter.

## Commands

### `drow start [<DIRECTORY>]`

This creates a new Drow site in the given directory, or in the current
directory if no directory is given.

### `drow page <TITLE>`

This creates a new page with the given title.

### `drow post <TITLE>`

This creates a new post with the given title combined with the full current date.

### `drow build`

Builds the site once, putting the results into the `docs/` folder for easy
deploys with GitHub Pages.

### `drow admin`

__This is a planned feature, to be implemented eventually.__

Starts two local servers:

- One is a live-reloading server for your Drow site.
- The other is an admin panel for managing your Drow site.

This admin panel allows you to add, delete, or edit every file in your Drow,
and to deploy updates to GitHub Pages. In essence, it is a web panel for your
static site!

Note that neither of these local servers has any sort of authentication on
them, and that the admin panel allows complete editing of your Drow site.

__NEVER RUN THESE SERVERS IN A PUBLICLY-ACCESSIBLE LOCATION.__

## Contributing

To learn how to contribute, see [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

Drow is MIT licensed. You can see the full license text in [`LICENSE.md`](LICENSE.md).


# Drow

__NOTE: DROW IS CURRENTLY IN AN ALPHA STATE, AND SHOULD NOT BE TRUSTED.__

Drow is a work-in-progress static site generator designed for extreme ease of
use. It has few features. It is not configurable. It has an excellent CLI and
documentation, and a developer who cares. ❤️

## Table of Contents

- [Design Goals](#design-goals)
- [Structure](#structure)
- [Commands](#commands)
    - [`drow setup`](#drow-setup-directory)
    - [`drow page`](#drow-page-title)
    - [`drow post`](#drow-post-title)
    - [`drow build`](#drow-build)
    - [`drow admin`](#drow-admin)
- [Contributing](#contributing)
- [License](#license)

## Design Goals

__1. Provide as simple a mapping from source to site as possible.__

It should be easy to predict what your final site will look like just by
looking at the source of your repository.

__2. Be easy to fork and modify for your own purposes.__

Drow is minimal, and that means that it may not do exactly the things you
want it to do. In that case, it should be easy to fork the Drow repo,
modify it to do what you want, and install it.

__3. Include only the core features people actually use.__

Some static site generators are packed with features. This can be really
great, but if you're like me you know what you want your site to do, and
don't like dealing with the complexity added by additional features. Drow
isn't packed with features, but that helps make it extremely easy to use!

__4. Include only the bare minimum configuration options.__

By the same token, more configurability means more chances to be confused
or for something to go wrong. Drow is not configurable, which means it's not
hard to predict how things will behave!

__5. Be easy to deploy.__

Who wants to worry about deploying? Drow is designed to work solely with
GitHub Pages, and in fact _only_ supports the project pages set up, with
the site deployed from the `docs/` folder. This means that Drow actually
doesn't even have a command to handle deploys! Just `drow build` your site,
then `git push` to deploy!

__6. Be fun to write posts in.__

I wrote Drow to manage my own blog, and I wanted something that would be
fun to write in. That's why `drow admin` exists. Writing posts in the
command line is annoying, and there's no reason to have to do it! Instead,
with Drow, you can write them in a nifty local admin panel and have
everything work exactly as you'd expect!

## Structure

Every Drow site starts with a `drow setup`. This copies the base Drow project
template, which looks like this:

```
|- Drow.toml
|- assets/
|- pages/
|- posts/
|- templates/
```

`Drow.toml` is where you put any configuration you want your pages, posts, or
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

### `drow setup [<DIRECTORY>]`

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


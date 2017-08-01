# Drow

Drow is a work-in-progress static site generator designed just how I like it.

Drow's design goals are:

- Provide as simple a mapping from source to final site as possible.
- Be easy to fork and modify for your own purposes.
- Run as quickly as possible.
- Include only the features that I actually use.
- The only configuration is what is minimally required.

Drow assumes you are publishing through GitHub pages, and is designed to support
that use case only.

If there is a feature you want to add to Drow, you can suggest it, but know that
the preference is for you to fork Drow and modify it to your needs. This is very
much on purpose, as personally I like to have as much control over the generation
of my sites as possible. If you do too, then Drow may be a good starting point.

## How Building This Site Should Work

Templates are used when generating each page. Every page defaults to the same
template, but may use an alternative template if desired.

Static files and assets are copied directly to the root of the generated site.

Pages are run through the assigned template and are then copied into the root
of the generated site at `/<page>/index.html`. Obviously, no "blog" page is
allowed, as one is automatically generated.

Posts are put into the `/blog` directory, with the file name
`yyyy-mm-dd-title.md` turned into the file
`/blog/<yyyy>/<mm>/<dd>/<title>/index.html`

The `/blog` directory also contains `/blog/archives/index.html`, which is an
auto-generated history of all the posts on the site.

Finally, it also has `/blog/tags/<tag>/index.html`, each of which lists the
posts tagged with the given tag, and `/blog/tags/<tag>/atom.xml`, which is an
auto-generated RSS feed of posts with that tag.

There's also, at the root level, `atom.xml` for the all-posts RSS feed.

## Drow Admin

This is a new idea I am working on, but I'd like to be able to conveniently
edit a Drow site's configuration, write new posts, and edit existing posts,
without working in the command line. I love the command line, but I don't
find it very conducive to long form writing.

The idea is to add a command `drow admin` that:

1. Starts up a local live-reloading server running the Drow site.
2. Starts up a separate local server providing a web interface to edit
   the Drow configuration, edit posts, add posts, edit pages, and add pages.

## Drow File Structure

Drow is organized into one configuration file and four folders:

- `Drow.toml`
- `assets`
- `pages`
- `posts`
- `templates`

__`Drow.toml`__

Contains all configuration data for the site, which will be made accessible
to the template pages.

__`assets`__

The CSS, JS, and images for the site. The contents of this folder are copied
directly into the root of the generated site. So `assets/css` becomes `./css`.

__`pages`__

Pages which will be served on the final site. The index page gets special
treatment, and will be the homepage of the site. Every other pages `blah.html`
becomes `blah/index.html`.

__`posts`__

The posts, which made Drow sites into blogs. These are processed as described
in the above section.

__`templates`__

The templates from which all the HTML will be generated.



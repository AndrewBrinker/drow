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
   the Drow configuration, edit posts, and add posts.


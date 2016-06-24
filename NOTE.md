# How Building This Site Should Work

Layouts are used when generating each page. Every page defaults to the same
layout, but may use an alternative layout if desired.

Static files and assets are copied directly to the root of the generated site.

Pages are run through the assigned layout and are then copied into the root of
the generated site at `/<page>/index.html`.

Posts are put into the `/blog` directory, with the file name `yyyy-mm-dd-title`
turned into the file `/blog/<yyyy>/<mm>/<dd>/<title>/index.html`

The `/blog` directory also contains `/blog/archives/index.html`, which is an
auto-generated history of all the posts on the site.

Finally, it also has `/blog/tags/<tag>`, each of which contains `index.html`,
which lists the posts tagged with the given tag, and `atom.xml`, which is an
auto-generated RSS feed of posts with that tag.

There's also, at the root level, `atom.xml` for the all-posts RSS feed.


---
layout: post
title: "Joining the Rust Community"
date: 2016-05-25 18:38:24 -0400
comments: true
categories: Rust 
---

If you're new to the Rust community, and unsure of what to read, how to take part,
or where to ask questions, this post is for you. The Rust community is growing and
active, and it can be difficult to understand what all the parts of it are, and how
to go about orienting yourself in the Rust world. Hopefully, by the end of this
post you will have a better sense of things.

<!-- more -->

## What To Read

The first thing to read about Rust is the ["The Rust Programming Language,"][trpl]
which is the official Rust book, maintained by the Rust team. It's updated
regularly, and is the go-to introduction to the language.

Next, check out the [Frequently Asked Questions][faq] page, and remember it for later.
There are a variety of questions you'll probably have that are already answered here.

If you're looking to get into the unsafe portions of Rust, read the
[Rustonomicon][nomicon]. It provides a much higher degree of detail about the
specifics of a variety of Rust features, and along with the Rust Book provides the
most complete and official treatment of the language as a whole.

If you want a variety of resources developed by people from around the Rust
community, the [rust-learning][rust-learning] repo is regularly adding new content
to help you understand Rust.

When you need to know what's provided in the Rust standard library, [read the API
docs.][API] If you're using DuckDuckGo, you can quickly search the Rust API docs
by adding `!rust` to the beginning of your searches.

To understand Cargo, the Rust package manager, check out the official
[Cargo Guide][cargo-guide], which includes a lot of detail about what Cargo does,
and how it works.

There's also the [Compiler Error Index][error-index], which you can also read using
`rustc --explain <error number>`, and the [syntax index][syntax-index], which
lists and describes all of Rust's syntactic features.

If you want to stay up to date with what's happening in the world of Rust, check out
[This Week in Rust][twir], which links to blog posts and projects from the community,
highlights quality Rust crates, and lists upcoming Rust events around the world.

Finally, check out Rust's policies on [security][security],
[copyrights][copyright], and the [code of conduct][coc].

## How To Take Part

You can contribute to Rust in a number of ways:

Tackle an issue in the Rust compiler issue tracker, maybe one marked
[easy][e-easy] or one with [mentoring][e-mentor] available.

Contribute to one of the other Rust project repositories, like the
[Rust website][rust-www], the [Cargo repo][cargo-repo], or one of the repos in the
[nursery][nursery] (these are repositories being considered for "official" status
as part of the Rust project).

You can also read and give input on [Rust RFCs][rfcs] (Requests for Comments),
which are proposals for improvements to the Rust language.

This Week in Rust includes a section listing
[opportunities for participation][twir-participation] on a variety of Rust
projects around the community. This is the link to the most recent This Week in Rust
post (at the time of this article's publication), but each week should have a similar
section.

If you don't necessarily want to write code, writing documentation is always
appreciated. There are a number of [open issues][rust-docs] for improving the
official Rust documentation, and there are always [Rust crates][crates] in need of
better documentation as well.

You can also do what I'm doing here, and write blog posts. Whether you're new to
Rust or an old hand at it, explaining what you're doing, how you're doing it, and
the problems you have and things you learn is all valuable to other programmers
both inside and outside of the community!

## Where to Ask Questions

Finally, where do you go to ask questions? There are a number of options in the Rust
community, including:

The official Rust [users forum][users-forum], which is a central location to ask
about and discuss Rust. If you're looking to have a technical discussion about the
internals of the Rust compiler, or ideas for the Rust project, you can alternatively
use the Rust [internals forum][internals-forum].

If you prefer immediate chat, there's always IRC. Rust's IRC channels are all on the
Mozilla IRC network (`irc.mozilla.org`). The Rust website includes an up-to-date
[list of major Rust IRC channels][irc-channels], and a number of projects also
maintain their own project-specific channels as well.

If you want to meet in person, there are
[over 50 Rust user groups worldwide][user-groups], as well as the
[Rust calendar][calendar], which includes a variety of regular events and
conferences.

If you like Reddit, the [Rust subreddit][subreddit] is active, and a useful place
to ask and answer questions, and to keep up with the goings-on of the Rust community.

There's also always [Stack Overflow][stack-overflow], which has the Rust tag to
keep track of Rust-related questions.

## Conclusion

This is by no means a complete map to the world of Rust, but it hopefully helps
point you in the direction of what you need. In the end, the best way to become
comfortable in the Rust community is to take part in it! Ask questions! Contribute
where you can!

The Rust community places a high premium on friendliness, and I hope your first
interactions with the community reflect that. I certainly feel that the Rust
community is one of the best programming communities in which I've taken part, and I
hope that you enjoy it, however you get involved, and whatever your skill level is.

Have fun, and welcome to the Rust community!

[trpl]: https://doc.rust-lang.org/book/
[faq]: https://www.rust-lang.org/faq.html
[nomicon]: https://doc.rust-lang.org/nomicon/
[rust-learning]: https://github.com/ctjhoa/rust-learning
[API]: https://doc.rust-lang.org/std/
[cargo-guide]: http://doc.crates.io/guide.html
[error-index]: https://doc.rust-lang.org/error-index.html
[syntax-index]: https://doc.rust-lang.org/book/syntax-index.html
[security]: https://www.rust-lang.org/security.html
[copyright]: https://www.rust-lang.org/legal.html
[coc]: https://www.rust-lang.org/conduct.html
[e-easy]: https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3AE-easy
[e-mentor]: https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3AE-mentor+
[rust-www]: https://github.com/rust-lang/rust-www
[cargo-repo]: https://github.com/rust-lang/cargo
[nursery]: https://github.com/rust-lang-nursery
[rfcs]: https://github.com/rust-lang/rfcs/
[twir]: https://this-week-in-rust.org/
[twir-participation]: https://this-week-in-rust.org/blog/2016/05/23/this-week-in-rust-131/#call-for-participation
[rust-docs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs
[crates]: https://crates.io/crates
[users-forum]: https://users.rust-lang.org/
[internals-forum]: https://internals.rust-lang.org/
[irc-channels]: https://www.rust-lang.org/community.html#irc-channels
[user-groups]: https://www.rust-lang.org/user-groups.html
[calendar]: https://calendar.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc@group.calendar.google.com&pli=1
[subreddit]: https://www.reddit.com/r/rust
[stack-overflow]: http://stackoverflow.com/questions/tagged/rust


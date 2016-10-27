title: The Rust Community
date: 2016-05-28 22:38:20 -0400
tags: Rust

---

I love the Rust community, I really do. Like I said in [my recent post about
how to take part in it][joining], I think the Rust community is one of the most
consistently welcoming and friendly programming communities with which I've
interacted.

That being said, I think there are some problems with the way the Rust community
presents itself and Rust to the outside world. In this post, I'd like to outline
those problems, and some ideas for how the Rust community can be even better at
explaining and pitching Rust to people who may not be sold on the idea yet.

<!-- more -->

## What We Get Right

The biggest thing I think the Rust community gets right is being helpful. There
are a lot of places you can go for help in the Rust community ([IRC][irc], [the user forum][user-forum], [/r/rust][subreddit],
[Stack Overflow][stack-overflow], [Twitter][twitter]), and in my experience
people in the community are extremely willing to give help and feedback. Whether
that means answering questions, providing feedback, or trying to direct other
people to contribute, the Rust community is quite good at getting people the
answers and help they're looking for. This is fantastic!

The Rust community also has a real care for designing high-quality APIs. This
_does_ mean that some things may take longer than may be desirable to reach
1.0, but it also means that if and when they do, you can be pretty well
assured that the API design is good. Look at the impending [version 1.0 of the
Rust regex crate][regex] for a nice example of this kind of care.

The Rust community is also _very_ out in the open. It may take some digging to
find discussions sometimes, but that's more about volume than anything else.
Everything you need to follow what's happening in Rust, including team
meetings, can be [found online][meeting-minutes]. When things are
insufficiently covered in issues or pull requests or RFCs, you can usually
expect a blog post to cover the idea (and there's work on [making those blog
posts easier to find][planet], too).

## What We Get Wrong

All of that being said, no community is perfect, and the Rust community is no
exception. Before I get into the specifics, I want to say that I think these
problems come from a good place, and that they can absolutely be improved.
Part of why I'm writing this is to put a light on them a bit, and maybe get
people thinking about how they talk when they talk about Rust.

The biggest problem is that we have a tendency to be too effusive when talking
about Rust. In the eyes of many of us (myself certainly included), Rust is a
_big deal_. We want to spread the word! There are real problems in software
development, particularly the development of efficient, scalable, large
systems that Rust addresses, and we want to get people onto it as quickly as
possible. That being said, there remain [real reasons not to choose
Rust][why-not], and it doesn't solve everything. It's worthwhile to remember
that.

This means, first and most importantly, doing more to empathize with the
specific situations of other developers and projects. To listen to what they're
dealing with, and to give an honest assessment of whether Rust would make sense
for their situation, even if it means telling them that Rust isn't the right
choice.

Maybe Rust doesn't have the right libraries (Yet! I hope! Maybe it's an
opportunity to fill the gap). Maybe the complexity of dealing with Rust's
safety guarantees isn't worthwhile for the domain (as lightweight as the
complexity of Rust can feel once you know it, there is a ramp up time, and it
will be a bit of a fight in some domains). All of this is fine. Rust doesn't
have to be and certainly can't be everything to everyone.

The more insiduous cousin of this problem is worse: judging people for _not_
choosing Rust. This may mean comments on a new project asking why it's not
written in Rust. This may mean comments on an old project about why it hasn't
been rewritten in Rust. Consistently though, it means condescension and
arrogance that is nothing but off-putting to people outside of the community
(and speaking for myself, someone inside of it).

Other languages are not the enemy. There is no enemy. Rust isn't trying to,
will not, and cannot "kill C / C++ / Java / (language of choice)." And other
people aren't wrong or bad for choosing something that isn't Rust, nor is
there an imperative to use Rust for anything. The imperative is better
software, more correctness, more confidence in what we build. If Rust is how
you get that, good for you. If it's not, well good for you too.

There's a lot to learn from other languages, and a lot to be done to bring
people from other languages into our community. None of that is served by
attacking, belittling, demeaning, or condescending to other languages or
the people in their communities. We don't need to tear them down for Rust
to succeed. In fact doing so only makes success harder.

With all of this said, I don't think that people in the Rust communiy who do
this are trying to do any of these things. It's easy to root for your team,
and easy to point out the flaws in systems that have been around a long time.
I guarantee you though, Rust has problems we haven't found yet, or haven't
happened yet. They'll happen, they'll be found. Nothing is perfect.

## What We Get Wrong (About Rust)

This follows on from the previous points. If we're going to talk about what
Rust offers, we need to be careful to get those guarantee statements right.
There are certain mistakes in explaining Rust that many people will get wrong,
with sometimes serious changes in meaning, and it can be problematic.

The most obvious example, which anyone in the Rust community has almost
certainly done themselves and/or seen someone else do, is claiming that Rust
guarantees no race conditions. It does not. It guarantees no data races. I'll
let [John Regehr explain the difference][regehr] (he does it better than I
could here).

This is one example, but it's a prototype for the problem. There's a lot we
still don't know about the exact guarantees that Rust provides, and parts of
the language remain unspecified. The [Rust Belt][rust-belt] project will soon
be working on solidifying our understanding of Rust's guarantees, and there
are language standardization efforts to crystalize things like the [Rust
memory model][memory-model], which will have substantive effects on [what we
can claim about Rust][tootsie-pop].

All of this is to say that erring on the side of caution when making statements
about Rust does or doesn't do is probably the right thing. It's not hard to
come back and clarify that Rust does more than you thought. It's harder to
come back and say it does less.

## Conclusion

I started this post with the most important thing to remember, which is that we
all like Rust, and all of us in the community want to see it continue to grow,
gain adoption, and succeed. Because we honestly believe that it's a quality
language that solves real problems, and that having more Rust in the world is
a good thing. But there are things to keep in mind.

Let's prosletize Rust, yeah? It's easy to do. There's a lot to like. Rust
is well-situated to fit into systems new and old. It has Cargo, a good FFI, an active
community, financial backing, the list goes on. But let's do it better. Let's
do it in a way that builds the community, and makes sure people feel welcome
and excited, not defensive.

The next time you're writing or talking about Rust, particularly with someone
who's new to it, just remember: empathy, restraint, and honesty. That's how we
get people to try it, to learn, and to hopefully come to like it. Let's focus
on what we're already good at, being welcoming and helpful, without the judgment
or conflict that only drives people away.

[joining]: http://www.suspectsemantics.com/blog/2016/05/25/joining-the-rust-community/
[irc]: https://www.rust-lang.org/community.html#irc-channels
[user-forum]: https://users.rust-lang.org/
[subreddit]: https://www.reddit.com/r/rust
[stack-overflow]: http://stackoverflow.com/questions/tagged/rust
[twitter]: https://twitter.com/hashtag/rustlang
[regex]: https://github.com/rust-lang/rfcs/pull/1620
[meeting-minutes]: https://github.com/rust-lang/meeting-minutes
[planet]: https://internals.rust-lang.org/t/should-there-be-a-rust-planet/3434
[why-not]: https://www.reddit.com/r/rust/comments/4kqhqz/why_arent_you_using_rust_at_work/
[regehr]: http://blog.regehr.org/archives/490
[rust-belt]: http://plv.mpi-sws.org/rustbelt/
[memory-model]: https://github.com/rust-lang/rfcs/issues/1447
[tootsie-pop]: http://smallcultfollowing.com/babysteps/blog/2016/05/27/the-tootsie-pop-model-for-unsafe-code/


---
layout: post
title: "Cell Types in Rust"
date: 2016-04-02 08:54:54 -0700
comments: true
categories: 
    - Types
    - Rust
published: false
---

This was not my original plan, but I guess it's now a "_ Types in Rust" series.
In this post, I will be going through Rust's "Cell types" `Cell` and `RefCell`,
and covering Rust's notion of "interior mutability." This is another one of
those less-discussed parts of the language that new Rust programmers often
seem to struggle with, so I figure it's worth covering here.

This post will start with coverage of Rust's mutability and aliasing rules,
then jump into an explanation of `Cell` and `RefCell`---why they exist, when
you should use one or the other---and end with some real-world examples of
their use.

<!-- more -->

## Mutability vs. Uniqueness

Rust programmers often talk about `&` as indicating a reference and `& mut` as
indicating a mutable reference. This is effectively true, but the reality is
a little more nuanced. More correctly, `&` indicates a _shared reference_, and
`& mut` indicates a _unique reference_. Rust's borrowing rule is that you may
have either an unbounded number of shared references to a resource, or a single
_unique reference_, but not both. The reason for this rule is that mutation
is only safe if _aliasing_---having multiple references to the same resource---
is guaranteed not to happen. Hence _unique references_ are the only references
through which mutation is safe.

There is, however, a small white lie in this explanation. There are in fact two
dual notions of mutability in Rust: exterior mutability and interior mutability.

To illustrate, let's look at the example the Rust book gives for exterior
mutability: `Arc`. An `Arc` is an atomically reference-counted "smart pointer."
It allows you to make copies of a pointed-to resource while ensuring all
allocated memory is reclaimed when no more references exist to that resource
(the "atomic" portion of its name means that it is thread-safe to share an
`Arc` between threads, and isn't material to this discussion).

In the following code, we get an `Arc` pointing to the number `5`, which is
then cloned, meaning that two references exist to the `5` (both references go
out of scope at the end of the `main` function body, so the memory is then
reclaimed as expected).

```rust
use std::sync::Arc;

fn main() {
    let x = Arc::new(5);
    let y = x.clone();
}
```

What may seem surprising about the above code is that `x` is _not_ declared
mutable, and yet `clone()` must by design have to increment its internal
reference count, which should require mutation.

To quote the Rust book, "when we something is 'immutable' in Rust... we mean
something has 'exterior mutability.'"




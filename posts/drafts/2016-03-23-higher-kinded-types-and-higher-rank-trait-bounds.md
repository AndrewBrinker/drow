---
layout: post
title: "Higher-Kinded Types and Higher-Rank Trait Bounds"
date: 2016-03-23 19:32:33 -0400
comments: true
categories:
    - Rust
    - Types
published: false
---

Higher-Kinded Types and Higher-Rank Trait Bounds are terms that get thrown around
the Rust community reasonably often, and seem to be a source of confusion for a
number of people. In this post, I am going to attempt to clarify the purpose of
each, and to reduce the confusion between the two.

The truth of the matter is that Higher-Kinded Types (HKTs) and Higher-Rank Trait
Bounds (HRTBs) are very different concepts with unfortunately similar names. One
(HRTBs) exists in Rust today, the other (HKTs) is a desired feature for the future.
So what do these actually mean? Well...

<!-- more -->

## Higher-Kinded Types

### TOC

- What are Higher-Kinded Types (basic explanation)
- What are Higher-Kinded Types (type theory explanation)
- Why are Higher-Kinded Types useful (programming perspective)
- Why are Higher-Kinded Types useful (verification perspective)

Let's cover these first. Support for Higher-Kinded Types in any language means that
so-called "type constructors" (like `Vec<T>`, without a concrete parameter) can be
used in the same places a bonafide type (like `i32`) can be used. This simple idea
allows for a greater degree of expression at the type level, and can be used to
encode a variety of useful things.

Let's develop some notation to clarify what we're talking about here. Imagine that,
for every type, we mark the number of "holes" it has ("holes" being un-filled type
parameters). For a type with no holes, like `i32`, or `bool`, or `String`, we write
`*` (which we call "Type").

If a type has one hole, like `Vec<T>`, then we write `* -> *`. If you think this
looks like a notation for functions, you're right! `Vec<T>` can be thought of as a
function taking in a type of kind `*` and returning a new type of king `*`. If the
parameter gets filled, the result is the result of applying our function. So
`Vec<T>` has "kind" `* -> *` while `Vec<i32>` has kind `*`, because the hole has
been filled by `i32`.

## Higher-Rank Trait Bounds

- What are Higher-Rank Trait Bounds (basic explanation)
- What are Higher-Rank Trait Bounds (type theory explanation)
- Why are Higher-Rank Trait Bounds useful? (programming perspective, particular focus on implementing closures)
- What are Higher-Rank Trait Bounds useful (theory perspective)


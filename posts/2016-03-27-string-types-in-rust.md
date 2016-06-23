---
layout: post
title: "String Types in Rust"
date: 2016-03-27 20:12:27 -0400
comments: true
categories: 
    - Rust
    - Types
---

This seems to be a common issue, so let's talk about the different string types
in the Rust programming language. In this post I'm going to explain the
organization of Rust's string types with [`String`][string] and [`str`][str] as
examples, then get into the lesser-used string types---[`CString`][cstring],
[`CStr`][cstr], [`OsString`][osstring], [`OsStr`][osstr], [`PathBuf`][pathbuf],
and [`Path`][path]---and how the [`Cow`][cow] container can make working with
Rust strings easier.

<!-- more -->

The most important thing to understand is that string types in Rust come in
pairs, which I'll call the "owned" variety and "slice" variety (_the term
"variety" is being used here to mean roughly "set of types", to make clear that
"owned" and "slice" are not actual Rust types, but a notion of organization for
Rust's string types_). The "owned" variety of strings---[`String`][string],
[`CString`][cstring], [`OsString`][osstring], and [`PathBuf`][pathbuf]---have
ownership over their contents (hence the name!), and can grow or shrink. The
"slice" variety of strings---[`str`][str], [`CStr`][cstr], [`OsStr`][osstr], and
[`Path`][path]---are views into some collection of characters. There's also the
[`Cow`][cow] wrapper type, which can make working with the two varieties of
strings easier while retaining good performance characteristics.

## "Owned" vs. "Slice" varieties of Strings

[`String`][string] and [`str`][str] are, by far, the most common string types
in Rust, so I will use them to illustrate the difference between the two
varieties of string types. [`String`][string] (the "owned" variety of string
type) is a wrapper for a heap-allocated buffer of UTF-8&ndash;encoded unicode
codepoints. [`str`][str] (the "slice" variety of string type) is a buffer of
UTF-8&ndash;encoded unicode codepoints that may be on the heap or in the
program memory itself.

When you create a string literal in Rust, it is by default of type `&str`.
There are two things this could mean, depending on how that buffer was
created.

### "Slice" on the Heap

If the reference is taken from a [`String`][string], it will be a reference to
the contents of the [`String`][string]'s internal buffer, which is on the heap.
Handing out these references is a common pattern for effectively using "owned"
variety strings in Rust. For convenience's sake, all references to "owned"
variety strings coerce to references to their "slice" variety equivalent. That
is, `&String` becomes `&str`. It is considered good practice to use the latter
type for function parameters taking a reference to a string.

### "Slice" in Program Memory

If the buffer is a string literal, with or without an explicit lifetime, then
the buffer will be located in program memory. In fact, all string literals have
the `'static` lifetime by default, meaning they can be safely referenced from
anywhere in the program. Note though that having a buffer with a `'static`
lifetime is _not_ the same as declaring a `static` variable. A buffer with a
`'static` lifetime may be safely referenced from anywhere, but must still be
passed to functions explicitly; it is not globally visible. A variable must be
declared with the `static` keyword to be globally visible.

There is some nuance here though. When returning a `&str` from a function, the
lifetime of the reference must be tied to either a single input lifetime, or
a method taking either `&self` or `&mut self`. In the first case, the lifetime
of the returned reference will be the lifetime of the single input reference.
In the second case, the lifetime will be the lifetime of the reference to
`self`.

If your function does not fit either of these cases, `rustc` will return an
error, as in the following example:

```rust
fn get_string() -> &str {
    "A string!"
}

fn main() {
    let string = get_string();
    println!("{}", string);
}
```

You can avoid the lifetime elision problems entirely by explicitly annotating
the function in one of the following ways:

```rust
fn get_string_static() -> &'static str {
    "A string!"
}

fn get_string_function_call<'a>() -> &'a str {
    "A string!"
}

fn main() {
    let string_1 = get_string_static();
    let string_2 = get_string_function_call();

    println!("{}", string_1);
    println!("{}", string_2);
}
```

So what do you do if you want to return a string from a function without
giving it a `'static` lifetime, or want to return a string whose contents
aren't known at compile time? You use the "owned" variety of string type, in
this case: [`String`][string].

### Why "Owned" Strings Exist

The "owned" varities of strings are allocated on the heap, and do not suffer the
same limitations of the "slice" varities. They may be freely moved around
without safety issues, handing out slices of their internal buffer as needed.
The "slice" varities do not have the same guarantees, and so can't be used as
freely.

If you want to avoid dealing with ownership and borrowing for strings, you
can just turn every "slice" variety into an "owned" variety via the
[`to_owned()`][to_owned] method (provided by the [`ToOwned`][toowned] trait,
which all "owned" varities of string types implement), and only work with the
"owned" varities, but doing so would incur performance penalties when you
unecessarily allocate string buffers on the heap, and would be considered poor
Rust style. Instead, it's best to develop an understanding of when the two
varities of strings are needed.

Another important thing to note is that because the "owned" varities of strings
abstract away the underlying buffer, they can grow or shrink, possibly
allocating a new underlying buffer and copying their contents to this new
buffer. The "slice" varities of strings cannot be resized, as they may not even
be on the heap.

The "slice" variety strings can only be accessed via what's called a "fat
pointer." This is because slices are "dynamically-sized types," meaning they
do not carry information about their own length. They are simply some
collection of contiguous memory. A "fat pointer" to a slice stores both a
pointer to the memory in question and the length of the data stored at that
memory location. This is all handled automatically by Rust, but it means that
the "slice" variety of strings are interacted with via references, rather than
being handled directly. For more detail about dynamically-sized types, check
out "The&nbsp;Rustonomicon," which [covers them in detail][rustonomicon].

## The String Type Pairs

The pairs of string types are differentiated from each other by the guarantees
provided by the underlying buffer.

[`String`][string] and [`str`][str]
: [`String`][string] and [`str`][str] are guaranteed to be valid UTF-8 encoded
  Unicode strings. If you're wondering why UTF-8 is the standard encoding for
  Rust strings, check out the [Rust FAQ's answer to that question][why-utf8].
[`CString`][cstring] and [`CStr`][cstr]
: [`CString`][cstring] and [`CStr`][cstr] are guaranteed to be compatible with C
  strings, and are usually used in FFI code.
[`OsString`][osstring] and [`OsStr`][osstr]
: [`OsString`][osstring] and [`OsStr`][osstr] are guaranteed to be
  platform-native strings (that is, they use the encoding of the current
  platform) that can be cheaply converted into [`String`][string] and
  [`str`][str] types. They are usually used when interacting directly with the
  operating system.
[`PathBuf`][pathbuf] and [`Path`][path]
: [`PathBuf`][pathbuf] and [`Path`][path] are wrappers around
  [`OsString`][osstring] and [`OsStr`][osstr] that provide convenient methods
  for operating on paths according to the rules of the current system. They are
  usually used when interacting with system paths.

## [`Cow`][cow]

There are times when working with strings in Rust that you want to cleanly
abstract over the two varities of string types. Maybe you want a container that
may hold either "owned" variety or "slice" variety strings, or you want to use
slices except for cases where an "owned" variety is absolutely necessary. In
these situations, use [`Cow`][cow].

[`Cow`][cow] (which stands for "Clone on Write") is a container that can take in
a "slice" variety of string type, and only convert that "slice" variety into an
"owned" variety when absolutely necessary (when something attempts to write to
the slice). This has the advantage of reducing the complexity of reasoning about
ownership, while also helping keep unecessary allocations down. It is not a
panacea, but it can be very helpful sometimes.

## Conclusion

That was a relatively quick explanation of string types in Rust. Hopefully that
helped to clarify a bit of why the different string types exist, and why they
are useful. As a final reference, here is a table of the different types:

| Guarantees    | "Slice" variety  | "Owned" variety        |
|:--------------|:-----------------|:-----------------------|
| UTF-8         | [`str`][str]     | [`String`][string]     | 
| C-compatible  | [`CStr`][cstr]   | [`CString`][cstring]   |
| OS-compatible | [`OsStr`][osstr] | [`OsString`][osstring] |
| System path   | [`Path`][path]   | [`PathBuf`][pathbuf]   |

---

Updated (3/28/2016):

- This post has been updated based on corrections from [CryZe92][cry] and
  [Ilogiq][ilo] on the Rust subreddit. Thanks to both of them for the help!

Updated (3/29/2016):

- Replaced the word "sort" with "variety" based on feedback from [rkangel][rka]
  on Hacker News.
- Clarified the wording of the unicode explanation, based on feedback from
  [nothrabannosir][not], also on Hacker News.
- Fixed a typo based on feedback from [respeccing][res] on Twitter.
- Reworded the transition into the explanation of "owned" string types, based
  on feedback from [bjzaba][bjz] from the Rust subreddit.

Updated (3/31/2016):

- Replaced "three" with "two," fixing an ommission from earlier changes.

[string]: http://doc.rust-lang.org/std/string/struct.String.html "Rustdoc for the String type"
[str]: http://doc.rust-lang.org/std/primitive.str.html "Rustdoc for the str type"
[cstring]: http://doc.rust-lang.org/std/ffi/struct.CString.html "Rustdoc for the CString type"
[cstr]: http://doc.rust-lang.org/std/ffi/struct.CStr.html "Rustdoc for the CStr type"
[osstring]: http://doc.rust-lang.org/std/ffi/struct.OsString.html "Rustdoc for the OsString type"
[osstr]: http://doc.rust-lang.org/std/ffi/struct.OsStr.html "Rustdoc for the OsStr type"
[pathbuf]: http://doc.rust-lang.org/std/path/struct.PathBuf.html "Rustdoc for the PathBuf type"
[path]: http://doc.rust-lang.org/std/path/struct.Path.html "Rustdoc for the Path type"
[cow]: http://doc.rust-lang.org/std/borrow/enum.Cow.html "Rustdoc for the Cow type"
[to_owned]: http://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned "Rustdoc for the to_owned function"
[toowned]: http://doc.rust-lang.org/std/borrow/trait.ToOwned.html "Rustdoc for the ToOwned trait"
[why-utf8]: https://www.rust-lang.org/faq.html#why-are-strings-utf-8 "Rust FAQ answer about why UTF-8 is the standard encoding for Rust strings"
[rustonomicon]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts "Rustonomicon section on dynamically-sized types"
[cry]: https://www.reddit.com/r/rust/comments/4c7wq2/string_types_in_rust/d1fsxd4 "Feedback from CryZe92 on the Rust subreddit"
[ilo]: https://www.reddit.com/r/rust/comments/4c7wq2/string_types_in_rust/d1fyvjo "Feedback from Ilogiq on the Rust subreddit"
[rka]: https://news.ycombinator.com/item?id=11380530 "Feedback from rkangel on Hacker News"
[not]: https://news.ycombinator.com/item?id=11379308 "Feedback from nothrabannosir on Hacker News"
[res]: https://twitter.com/respeccing/status/714551241615998976 "Feedback from respeccing on Twitter"
[bjz]: https://www.reddit.com/r/rust/comments/4c7wq2/string_types_in_rust/d1hbnmp "Feedback from bjzaba on the Rust subreddit"


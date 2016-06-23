---
layout: post
title: "Rust Modules and File Hierarchy"
date: 2016-05-26 21:10:10 -0400
comments: true
categories:  Rust
---

Rust's module system follows some fairly simple rules, but also seems to be a
tripping point for new users of the language. In this post, I am going to walk
through how the Rust module system works, and how it maps to the organization
of files on the file system.

<!-- more -->

Rust modules are how code is organized within a single crate (the term for
Rust packages). Every crate starts with a top-level module, traditionally
found in `src/lib.rs` for libraries, and `src/main.rs` for binaries.
Additional modules are declared with the `mod` keyword, while functions,
types, etc. are used from a module with the `use` keyword.

## Declaring Modules

Imagine the following Rust project structure (which is fairly standard):

```rust
- src/
  |- lib.rs
  |- foo.rs
  |- bar.rs
- Cargo.toml
```

In `src/lib.rs`, you would expect to find the following lines:

```rust
mod foo;
mod bar;
```

These lines tell the Rust compiler to look in the current directory for two
files called `foo` and `bar`, or for folders `foo` and `bar` with a file
called `mod.rs` in them. If it finds them, all is well. If it doesn't,
you'll get an error.

Further down in `src/lib.rs`, you may want to use something provided by one
of these modules. This can be done like so:

```rust
mod foo;
mod bar;

fn main() {
    // The `Foo` struct is in the `foo` module,
    // and being referenced through the module name.
    let f = foo::Foo::new();
}
```

Alternatively, if you don't want to prefix `Foo` with the module name, you can
`use` it:

```rust
mod foo;
mod bar;

use foo::Foo;

fn main() {
    // `Foo` has been imported into the current
    // namespace, and doesn't need the module
    // name prefix any more.
    let f = Foo::new();
}
```

## Nesting Modules

If you want to nest your modules, you can do so like this:

```
- src/
  |- lib.rs
  |- foo.rs
  |- bar/
     |- mod.rs
     |- baz.rs
- Cargo.toml
```

In `src/lib.rs`, your module declarations remain the same:

```rust
mod foo;
mod bar;
```

In `src/bar/mod.rs`, you can then declare the modules in the current folder:

```rust
mod baz;
```

## Public Modules

By default, declaration of a module is private. Meaning that users of the
crate won't be able to see the module and its contents. You can make a module
public by prefixing the module's declaration with `pub`. If we wanted the
`foo` and `bar` modules to be public, we would change the imports in
`src/lib.rs` to:

```rust
pub mod foo;
pub mod bar;
```

Now a user of the crate could access the internals of the `foo` and `bar`
modules like so (imaging the crate name is `blah`):

```rust
extern crate blah;

mod blah::foo;
mod blah::bar;

fn main() {
    // It's the same as before!
    lt f = foo::Foo::new();
}
```

You may have noticed the `extern crate` declaration. This tells the Rust
compiler to learn for a crate to link to, called "blah". If you're using
Cargo (which you probably should be!) then Cargo will automatically link
in whatever crates are listed in the `[dependencies]` section of your
`Cargo.toml` file.

## Re-exporting

When organizing your modules, you also have the option to re-export things
from another module, so that they are also available from the current one.

For example, if `src/bar/baz.rs` defined a `Baz` struct that you wanted to
be available from the top-level crate module, you could do it like this:

```rust
// In src/lib.rs
mod foo;
mod bar;

pub use bar::Baz;

// In src/bar/mod.rs
mod baz;

pub use baz::Baz;

// In src/bar/baz.rs
struct Baz {
    // ...
}

impl Baz {
    fn new() -> Baz {
        // ...
    }
}
```

Then, a user of your crate could access `Baz` like so:

```rust
extern crate blah;

use blah::Baz;

fn main() {
    // Pulled into the current module's namespace,
    // you can access `Baz` directly.
    let b0 = Baz::new();

    // This is also an option.
    let b1 = blah::Baz::new();

    // As is this!
    let b2 = blah::bar::Baz::new();
}
```

## Conclusion

Rust keeps a clean separation between declaring a module and `use`ing its
contents, and also keeps a tight integration between the organization of
modules in a project and their association with files and folders in your
project's source tree. There are other things not covered here (like the
`self` and `super` keywords, multi-part `use` declarations, and inline
modules) which are better covered [in the Rust book][book].

[book]: https://doc.rust-lang.org/book/crates-and-modules.html




![pulldown-cmark-frontmatter forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/pulldown-cmark-frontmatter.svg)](https://crates.io/crates/pulldown-cmark-frontmatter)
[![Live Build Status](https://img.shields.io/github/actions/workflow/status/khonsulabs/pulldown-cmark-frontmatter/rust.yml?branch=main)](https://github.com/khonsulabs/pulldown-cmark-frontmatter/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/pulldown-cmark-frontmatter/coverage/badge.svg)](https://khonsulabs.github.io/pulldown-cmark-frontmatter/coverage/)
[![Documentation](https://img.shields.io/badge/docs-main-informational)]($docs-base$)

*This crate was written by someone unaffiliated with the `pulldown-cmark`
crate.*

This crate makes it easy to parse frontmatter contained within Markdown
documents when using the [pulldown-cmark][pulldown-cmark] Markdown parser.

Unlike many other frontmatter styles, this crate enforces a basic document
format:

- Optional top-level (h1) heading
- Optional code block
- Remaining Markdown document

By utilizing a code block instead of other markers, most Markdown editing
software can more intelligently handle syntax highlighting, errors, and more.

The `FrontmatterExtractor` type will detect and return a plain-text
representation of a top-level heading, if it's the first element in the
document. The heading will still be returned when iterating over the
`pulldown_cmark::Event`s.

After the optional top-level heading, if a code block is encountered, it will be
returned as `Frontmatter::code_block`. Unlike the heading, *the frontmatter code
block will not appear in the iterated `Event`s*.

This repository includes [frontmatter-example.md][frontmatter-md]
which both the [HTML rendering example][html] and the [extractor
example][extractor] use.

## HTML Rendering Example

This example shows how to use this crate with `pulldown-cmark`'s html module. It
is included in the repository at [examples/html.rs][html].

```rust,ignore
$../examples/html.rs:readme$
```

The repository includes the [rendered html output][frontmatter-html]
to see what the produced HTML looks like.

## Extractor Example

This example extracts the frontmatter from a Markdown document without parsing
the entire document. It is included in the repository at
[examples/extractor.rs][extractor].

```rust,ignore
$../examples/extractor.rs:readme$
```

[pulldown-cmark]: https://github.com/raphlinus/pulldown-cmark
[html]: $src-base$/examples/html.rs
[frontmatter-html]: $src-base$/frontmatter-example.html
[frontmatter-md]: $src-base$/frontmatter-example.md
[extractor]: $src-base$/examples/extractor.rs

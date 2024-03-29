# pulldown-cmark-frontmatter


![pulldown-cmark-frontmatter forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/pulldown-cmark-frontmatter.svg)](https://crates.io/crates/pulldown-cmark-frontmatter)
[![Live Build Status](https://img.shields.io/github/actions/workflow/status/khonsulabs/pulldown-cmark-frontmatter/rust.yml?branch=main)](https://github.com/khonsulabs/pulldown-cmark-frontmatter/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/pulldown-cmark-frontmatter/coverage/badge.svg)](https://khonsulabs.github.io/pulldown-cmark-frontmatter/coverage/)
[![Documentation](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/pulldown-cmark-frontmatter/main/pulldown_cmark_frontmatter)

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
// This example renders the example Markdown to html using
// `pulldown_cmark::html`, while also extracting the frontmatter from
// Markdown.
let mut extractor = FrontmatterExtractor::new(pulldown_cmark::Parser::new(include_str!(
    "../frontmatter-example.md"
)));

// The only difference from using the FrontmatterExtractor and the regular
// pulldown_cmark::Parser is that you must pass a mutable reference to the
// extractor to be able to read the Frontmatter it extracts.
let mut rendered = String::new();
pulldown_cmark::html::push_html(&mut rendered, &mut extractor);
assert_eq!(rendered, include_str!("../frontmatter-example.html"));

let frontmatter = extractor.frontmatter.expect("frontmatter not detected");
assert_eq!(
    frontmatter.title.expect("title not detected"),
    "Frontmatter Example Document"
);
let code_block = frontmatter.code_block.expect("code block not detected");
assert_eq!(code_block.language.as_deref(), Some("toml"));
let attrs: ExampleAttributes = toml::from_str(&code_block.source).expect("invalid toml");
assert_eq!(attrs.author, "https://fosstodon.org/@ecton");
```

The repository includes the [rendered html output][frontmatter-html]
to see what the produced HTML looks like.

## Extractor Example

This example extracts the frontmatter from a Markdown document without parsing
the entire document. It is included in the repository at
[examples/extractor.rs][extractor].

```rust,ignore
// This example extracts the frontmatter from the Markdown,
// `FrontmatterExtractor::extract()` which stops parsing the Markdown
// document after the frontmatter extraction is complete.
let extractor = FrontmatterExtractor::from_markdown(include_str!("../frontmatter-example.md"));
let frontmatter = extractor.extract().expect("frontmatter not detected");
assert_eq!(
    frontmatter.title.expect("title not detected"),
    "Frontmatter Example Document"
);
let code_block = frontmatter.code_block.expect("code block not detected");
assert_eq!(code_block.language.as_deref(), Some("toml"));
let attrs: ExampleAttributes = toml::from_str(&code_block.source).expect("invalid toml");
assert_eq!(attrs.author, "https://fosstodon.org/@ecton");
```

[pulldown-cmark]: https://github.com/raphlinus/pulldown-cmark
[html]: https://github.com/khonsulabs/pulldown-cmark-frontmatter/blob/main/examples/html.rs
[frontmatter-html]: https://github.com/khonsulabs/pulldown-cmark-frontmatter/blob/main/frontmatter-example.html
[frontmatter-md]: https://github.com/khonsulabs/pulldown-cmark-frontmatter/blob/main/frontmatter-example.md
[extractor]: https://github.com/khonsulabs/pulldown-cmark-frontmatter/blob/main/examples/extractor.rs

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), is open-source.
This repository is available under the [MIT License](./LICENSE-MIT) or the
[Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).

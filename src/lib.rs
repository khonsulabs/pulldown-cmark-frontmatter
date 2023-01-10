#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#![allow(
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc
)]

use pulldown_cmark::{CodeBlockKind, CowStr, Event};

/// Extracts [`Frontmatter`] from any `Iterator<Item =
/// pulldown_cmark::Event<'_>>`.
///
/// This type implements `Iterator<Item = pulldown_cmark::Event<'_>>`, so it can
/// be used interchangeably with any Markdown processing code that previously
/// interacted with [`pulldown_cmark::Parser`].
///
/// This type's [`Event`] iterator will look for a top-level
/// heading (h1) and/or a code block at the start of the document. If either or
/// both are detected, [`FrontmatterExtractor::frontmatter`] will be populated
/// with the detected [`Frontmatter`].
///
/// If a code block is detected in the frontmatter, the code block's
/// [`Event`]s not be returned when iterating. The top-level
/// heading's events will be returned as they appear in the original iterator.
pub struct FrontmatterExtractor<'a, T>
where
    T: Iterator<Item = Event<'a>>,
{
    /// The detected frontmatter, if any.
    pub frontmatter: Option<Frontmatter<'a>>,
    source: T,
    state: DocumentAttributeParserState<'a>,
}

impl<'a, T> FrontmatterExtractor<'a, T>
where
    T: Iterator<Item = Event<'a>>,
{
    /// Returns a new instance that extracts frontmatter from the provided
    /// [`Event`] iterator.
    pub fn new(parser: T) -> Self {
        Self {
            source: parser,
            frontmatter: None,
            state: DocumentAttributeParserState::Parsing,
        }
    }

    fn frontmater_mut(&mut self) -> &mut Frontmatter<'a> {
        if self.frontmatter.is_none() {
            self.frontmatter = Some(Frontmatter {
                title: None,
                code_block: None,
            });
        }

        self.frontmatter.as_mut().expect("always initialized")
    }

    /// Scans the start of the document looking for [`Frontmatter`]. If
    /// frontmatter is detected, it will be returned.
    ///
    /// This function will not consume the original iterator completely. It will
    /// stop as soon as it is done detecting the frontmatter.
    pub fn extract(mut self) -> Option<Frontmatter<'a>> {
        while let Some(_) = self.next() {
            if matches!(self.state, DocumentAttributeParserState::InDocument) {
                break;
            }
        }

        self.frontmatter
    }
}

impl<'a, 'cb> FrontmatterExtractor<'a, pulldown_cmark::Parser<'a, 'cb>> {
    /// Returns an instance that parses `markdown` with the default
    /// [`pulldown_cmark::Parser`].
    #[must_use]
    pub fn from_markdown(markdown: &'a str) -> Self {
        Self::new(pulldown_cmark::Parser::new(markdown))
    }
}

impl<'a, T> Iterator for FrontmatterExtractor<'a, T>
where
    T: Iterator<Item = Event<'a>>,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.source.next()? {
                Event::Text(text) if self.state.in_document_title() => {
                    self.frontmater_mut().title_mut().push_str(&text);
                    return Some(Event::Text(text));
                }
                Event::Text(text) if self.state.in_code() => {
                    let language = match self.state.code_block_kind() {
                        CodeBlockKind::Indented => None,
                        CodeBlockKind::Fenced(language) => Some(language),
                    };
                    let frontmatter = self.frontmater_mut();
                    frontmatter.code_block = Some(CodeBlock {
                        source: text,
                        language,
                    });
                }
                Event::Start(pulldown_cmark::Tag::Heading(
                    pulldown_cmark::HeadingLevel::H1,
                    fragment,
                    classes,
                )) if !self.state.in_document() => {
                    self.state = DocumentAttributeParserState::InTitle;
                    return Some(Event::Start(pulldown_cmark::Tag::Heading(
                        pulldown_cmark::HeadingLevel::H1,
                        fragment,
                        classes,
                    )));
                }
                Event::End(pulldown_cmark::Tag::Heading(
                    pulldown_cmark::HeadingLevel::H1,
                    fragment,
                    classes,
                )) if !self.state.in_document() => {
                    self.state = DocumentAttributeParserState::Parsing;
                    return Some(Event::End(pulldown_cmark::Tag::Heading(
                        pulldown_cmark::HeadingLevel::H1,
                        fragment,
                        classes,
                    )));
                }
                Event::Start(pulldown_cmark::Tag::CodeBlock(kind)) if !self.state.in_document() => {
                    self.state = DocumentAttributeParserState::InAttributeCodeBlock(kind);
                }
                Event::End(pulldown_cmark::Tag::CodeBlock(_)) if !self.state.in_document() => {
                    self.state = DocumentAttributeParserState::InDocument;
                }
                other => {
                    if !self.state.in_document_title() {
                        self.state = DocumentAttributeParserState::InDocument;
                    }

                    return Some(other);
                }
            }
        }
    }
}

enum DocumentAttributeParserState<'a> {
    Parsing,
    InTitle,
    InAttributeCodeBlock(CodeBlockKind<'a>),
    InDocument,
}

impl<'a> DocumentAttributeParserState<'a> {
    pub fn in_document(&self) -> bool {
        matches!(self, Self::InDocument)
    }

    pub fn in_code(&self) -> bool {
        matches!(self, Self::InAttributeCodeBlock(_))
    }

    pub fn code_block_kind(&self) -> CodeBlockKind<'a> {
        if let Self::InAttributeCodeBlock(kind) = self {
            kind.clone()
        } else {
            CodeBlockKind::Indented
        }
    }

    pub fn in_document_title(&self) -> bool {
        matches!(self, Self::InTitle)
    }
}

/// Metadata stored within a Markdown document.
pub struct Frontmatter<'a> {
    /// The top-level heading's plain-text contents, if the document began with
    /// a top-level heading.
    pub title: Option<String>,
    /// The frontmatter code block, if detected.
    pub code_block: Option<CodeBlock<'a>>,
}

impl<'a> Frontmatter<'a> {
    fn title_mut(&mut self) -> &mut String {
        if self.title.is_none() {
            self.title = Some(String::new());
        }

        self.title.as_mut().expect("always initialized")
    }
}

/// A code block from a Markdown document's [`Frontmatter`].
pub struct CodeBlock<'a> {
    /// The contents of the code block.
    pub source: CowStr<'a>,
    /// The language of the code block, which is the identifier following the
    /// three backticks in a fenced Markdown code block.
    pub language: Option<CowStr<'a>>,
}

#[test]
fn attribute_parser_test() {
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct Attributes {
        hello: String,
    }
    let source = r#"# My **Document**

```toml
hello = "world"
```

This is regular text
"#;
    let mut parser = FrontmatterExtractor::from_markdown(source);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, &mut parser);
    assert_eq!(
        html,
        "<h1>My <strong>Document</strong></h1>\n<p>This is regular text</p>\n"
    );

    let frontmatter = parser.frontmatter.expect("frontmatter not detected");

    assert_eq!(frontmatter.title.as_deref(), Some("My Document"));

    let code_block = frontmatter.code_block.expect("code block not detected");
    assert_eq!(code_block.language, Some(CowStr::from("toml")));
    let deserialized: Attributes = toml::from_str(&code_block.source).unwrap();

    assert_eq!(deserialized.hello, "world");
}

#[test]
fn indented_parse_test() {
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct Attributes {
        hello: String,
    }
    let source = r#"# My **Document**

    hello = "world"

This is regular text
"#;
    let mut parser = FrontmatterExtractor::from_markdown(source);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, &mut parser);
    assert_eq!(
        html,
        "<h1>My <strong>Document</strong></h1>\n<p>This is regular text</p>\n"
    );

    let frontmatter = parser.frontmatter.expect("frontmatter not detected");

    assert_eq!(frontmatter.title.as_deref(), Some("My Document"));

    let code_block = frontmatter.code_block.expect("code block not detected");
    assert_eq!(code_block.language, None);
    let deserialized: Attributes = toml::from_str(&code_block.source).unwrap();

    assert_eq!(deserialized.hello, "world");
}

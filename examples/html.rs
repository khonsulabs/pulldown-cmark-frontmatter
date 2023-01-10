use pulldown_cmark_frontmatter::FrontmatterExtractor;

use crate::shared::ExampleAttributes;

mod shared;

fn main() {
    // begin rustme snippet: readme
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
    assert_eq!(attrs.author, "https://fosstodon.org/ecton");
    // end rustme snippet
}

#[test]
fn runs() {
    main()
}

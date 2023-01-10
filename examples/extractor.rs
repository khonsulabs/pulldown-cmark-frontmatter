use pulldown_cmark_frontmatter::FrontmatterExtractor;

use crate::shared::ExampleAttributes;

mod shared;

fn main() {
    // begin rustme snippet: readme
    // This example extracts the frontmatter from the Markdown,
    // `FrontmatterExtractor::extract()` which stops parsing the Markdown
    // document after the frontmatter extraction is complete.
    let extractor = FrontmatterExtractor::new(pulldown_cmark::Parser::new(include_str!(
        "../frontmatter-example.md"
    )));
    let frontmatter = extractor.extract().expect("frontmatter not detected");
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

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.0 (2024-09-16)

### Breaking Changes

- `pulldown-cmark` has been updated to `0.12`. Thank you to @sbuller for this
  work!

## 0.3.0 (2024-07-30)

### Breaking Changes

- `pulldown-cmark` has been updated to `0.11.0`. Thank you to @sbuller for this
  work!

## 0.2.0 (2023-09-21)

### Added

- `FrontmatterExtractor::extracted()` is a new function that returns true once
  the extractor has determined the frontmatter is fully extracted.
- `FrontmatterExtractor::extract_buffered()` is a new function that advances the
  underlying iterator until the frontmatter has been fully extracted. Unlike
  `FrontmatterExtractor::extract()`, this function buffers all events so that
  they can be returned via iteration.

## 0.1.1 (2023-03-10)

- Broken links to examples have been fixed

## 0.1.0 (2023-01-10)

- Initial release

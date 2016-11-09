# Markdown to html converter

This is a very simple program that converts markdown to html.
It simply uses the crate [heodown for Rust](https://github.com/hoedown/rust-hoedown).

To build, do the usual `cargo build --release`.

Easy to use it with pipes on Linux:

> $ cat myMarkdownFile.md | md2html > myMarkdownFile.html


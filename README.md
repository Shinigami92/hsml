<p>
  <a href="https://github.com/Shinigami92/hsml/actions/workflows/ci.yml">
    <img alt="Build Status" src="https://github.com/Shinigami92/hsml/actions/workflows/ci.yml/badge.svg?branch=main">
  </a>
  <a href="https://github.com/Shinigami92/hsml/blob/main/LICENSE">
    <img alt="License: MIT" src="https://img.shields.io/github/license/Shinigami92/hsml.svg">
  </a>
  <a href="https://www.paypal.com/donate?hosted_button_id=L7GY729FBKTZY" target="_blank">
    <img alt="Donate: PayPal" src="https://img.shields.io/badge/Donate-PayPal-blue.svg">
  </a>
</p>

# UNDER CONSTRUCTION

Right now there is no usable version of `hsml` available. I'm just working on it.

<img src="https://chronicle-brightspot.s3.amazonaws.com/6a/c4/00e4ab3143f7e0cf4d9fd33aa00b/constructocat2.jpg" width="400px" />

# HSML - Hyper Short Markup Language

`hsml` is a hyper short markup language that is inspired by [pug](https://pugjs.org) (aka jade).

## What is it?

- `hsml` is written in [Rust](https://www.rust-lang.org) and compiles to HTML.
- There will be a binary that can take CLI arguments to compile a `.hsml` file to a `.html` file, but also there will be some other arguments to e.g. format a `.hsml` file.
- There will be also a library that can parse a `.hsml` file and return an AST for it. It is planned that this AST can be used in the JS ecosystem as well, so tools like ESLint and Prettier can work with it.
- There are two major differences between `pug` and `hsml`
  - `hsml` will support TailwindCSS and similar CSS frameworks out of the box, even with arbitrary values like `.bg-[#1da1f2]` or `lg:[&:nth-child(3)]:hover:underline`
  - `hsml` will **not** support template engine syntax. It is _just_ an HTML preprocessor.

## Why doing it?

- I want to learn Rust
- I use `pug` for my projects but sadly `pug`'s goal mismatches my preferences and comes with a lot of overhead I don't need

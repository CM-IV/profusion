<div align="center"> <img src="https://raw.githubusercontent.com/profusion-rs/profusion/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">profusion</h1>
<div align="center">
 <strong>
   Generate type-checked Rust from your SQL
 </strong>
</div>

<br />

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/profusion_sync">
    <img src="https://img.shields.io/crates/v/profusion_sync.svg?style=flat-square"
    alt="Crates.io version" />
  </a>

  <!-- Book -->
  <a href="https://profusion-rs.netlify.app/book/index.html">
  <img src="https://img.shields.io/badge/book-latest-blue?logo=mdbook&style=flat-square" alt="book">
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/profusion_sync/latest/profusion_sync/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/profusion_sync?style=flat-square">
  </a>

  <!-- License -->
  <a href="https://github.com/profusion-rs/profusion#License">
    <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square" alt="License">
  </a>

  <!-- Chat -->
  <a href="https://discord.gg/nYwUmQDHBZ">
    <img src="https://img.shields.io/discord/987088069280825401?label=chat&logo=discord&style=flat-square" alt="Chat">
  </a>
</div>

---

**Note:** This crate is the *synchronous* client. You can find the *asynchronous* client [here](https://crates.io/crates/profusion_async).

This is a client crate for [profusion](https://crates.io/crates/profusion). This dependency provides
1. Internals required by the generated code.
2. Public items that you may find useful when working with profusion (you can find more info about these in the [docs](https://docs.rs/profusion_sync/latest/profusion_sync/)).

***You need to depend on this crate for profusion's generated code to work properly.***
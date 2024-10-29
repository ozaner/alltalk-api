# alltalk-api

This is a crate that wraps the [AllTalk](https://github.com/erew123/alltalk_tts) API for easy use in Rust projects.

## Installation
To use the latest version of `alltalk-api` add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
alltalk-api = { git = "https://github.com/ozaner/alltalk-api.git", tag = "2.0.0" }
```

The tag version corresponds to the version of AllTalk the crate matches.

### API Version Compatibility
This crate is designed to match the v2 AllTalk API, which is still in progress on the [Beta branch](https://github.com/erew123/alltalk_tts/tree/alltalkbeta). As such, AllTalk may have breaking changes that make this crate no longer compatible. The current commit hash of the beta this crate has been tested with is [`0c9034c`](https://github.com/erew123/alltalk_tts/commit/0c9034c1f32a65573c3c4df4f9c907cd32b19be4).

While it may work with newer versions, no guarantees are made.

## Example
To run an example program that plays streamed TTS audio straight from the terminal, run the following in the root directory of the repo:

```
cargo run --example stream-tts
```

This example assumes AllTalk is hosted on the default local endpoint, and that it supports streamed audio in the currently switched engine.

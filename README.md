# sponsor-block
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![# Issues](https://img.shields.io/github/issues/zedseven/sponsor-block-rs?logo=github)](https://github.com/zedseven/sponsor-block-rs/issues)
[![Crates.io](https://img.shields.io/crates/v/sponsor-block?logo=rust)](https://crates.io/crates/sponsor-block)
[![Crate Downloads](https://img.shields.io/crates/d/sponsor-block?logo=azure-artifacts)](https://crates.io/crates/sponsor-block)

A Rust wrapper for the [SponsorBlock](https://sponsor.ajay.app/) API, which you
can find complete documentation for [here](https://wiki.sponsor.ajay.app/w/API_Docs).

Uses SponsorBlock data licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)
from https://sponsor.ajay.app/.
Please see the [SponsorBlock Database and API License](https://github.com/ajayyy/SponsorBlock/wiki/Database-and-API-License)
for more information.

This library is still missing many features of the full API, but it contains many
segment-retrieval functions necessary for use of the service.

For library documentation, visit [docs.rs](https://docs.rs/sponsor-block).

## Example
The following is a short example of how you might fetch the segments for a
video:
```rust
use sponsor_block::{AcceptedCategories, Client};

const USER_ID: &str = "your local user id - it should be random and treated like a password";

let client = Client::new(USER_ID);
let video_segments = client
    .fetch_segments("7U-RbOKanYs", AcceptedCategories::all())
    .await
    .ok();

// Then do something with your video segments...
```

## Project License
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in *sponsor-block* by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

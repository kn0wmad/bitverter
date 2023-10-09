# Bitverter - A simple Bitcoin conversion tool

Bitverter is a simple tool written in Rust for doing cryptocurrency conversions.  The beta release supports denomination conversions only.

Work in progress, please check back or feel free to reach out with any questions or comments :)

## Building, Testing, and Usage

Currently Bitverter is only built for the Linux command line.  If you are even a little comfortable with this, it will be no problem to run.  Please stay tuned for a GUI/Web version.

1.  Install Rust using rustup (https://rustup.rs/)
2.  Open a terminal and clone the git repo

`git clone https://github.com/kn0wmad/bitverter.git`

3.  Navigate to the CLI program folder

`cd bitverter/bitverter_cli`

4.  Run the tests (optional)

`cargo test`

5.  Run the program

`cargo run`

## Bitverter Roadmap

**I have shifted focus, but do plan to revisit this project soon**

Currency Conversion library
- [x]   Currency conversion library to convert from btc to mbtc, bits, sats and vice versa
- [x]   Unit tests

Complete basic CLI functionality
- [x]   Terminal UI for conversions
- [x]   Beta release

### Future features
- [ ]   Add a match statement to require a valid denomination
- [ ]   Iced GUI alpha release
- [ ]   Convert to/from USD
- [ ]   Spanish (espanol) translation

Complete Rust web server
- [x]   Warp as server
- [ ]   Reqwest for APIs
- [ ]   Serde for parsing responses
- [ ]   Maud for HTML templating
- [ ]   Actually... refactor to Axum instead of Warp
- [ ]   Trunk?
- [ ]   Publish

### Moonshots
- [ ]   Multi-language support

* * *

## License

![BipCot NoGov License](/images/bipcot144x144.png)

This software is covered under a *modified* BipCot No-Government License.  Use and reuse is permitted by anyone, except governments and the agents thereof.

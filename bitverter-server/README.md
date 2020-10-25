# bitverter-server

Server version of bitverter.

## run

`cargo +nightly run -p bitverter-server`

## dev

`cargo +nightly watch -x '+nightly run -p bitverter-server'`

needs:

- <https://github.com/passcod/cargo-watch>

## docs

- general

  - <https://cheats.rs>
  - <https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html>

- dependencies

  - web framework: <https://actix.rs/docs/request/>
  - templating: <https://maud.lambda.xyz/web-frameworks.html>
  - make api requests: <https://github.com/seanmonstar/reqwest>
  - async/await: <https://github.com/async-rs/async-std>

- tools

  - cargo add, etc.: <https://github.com/killercup/cargo-edit>

- apis

  - <https://docs.nomics.com#tag/Currencies>

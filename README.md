# 1Password Export Tool (pre-alpha)

1Password Export Tool is a simple piece of software that allows for filtered exports of your 1Password data.

It is currently in its early stages of development, and can export most information from 1Password.

## How to use

NOTE: installation will eventually be done with a `cargo install`.

Using the export tool is extremely easy. Assuming you have (Rust)[https://rust-lang.org] installed, clone this repository and `cd` into it.

Run: `cargo run`.

Additionally, you want to make sure you have the (1Password CLI)[https://developer.1password.com/docs/cli/] installed. Sign in to it. The exporter does not currently support authentication aside from biometrics, so you will have to do this manually for now. This is likely to change in the future.

Assuming you set up everything correctly, you will see a loading screen. This means the exporter is retrieving your data for export.

After loading, you will see a screen that looks similar to this:

![Initial view](./screenshots/initial-page.png?raw=true)

## 1PUX format

Exports are made as a (1PUX)[https://support.1password.com/1pux-format/] data file. Document downloading support will be added soon.

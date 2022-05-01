# 1Password Export Tool (pre-alpha)

1Password Export Tool is a simple piece of software that allows for filtered exports of your 1Password data.

It is currently in its early stages of development, and can export most information from 1Password.

## Current limitations

There is some data with the CLI cannot currently obtain that is left out from the export. I will be adding a full list here.

Additionally, document exporting is not yet added. This will be added soon.

All account data is fetched when an account is ran, then filtered down and exported. I plan to make it only fetch partials, then fetch the remaining data during the export. This is another easy feature that will be added soon.

## How to use

NOTE: installation will eventually be done with a `cargo install`.

Using the export tool is extremely easy. Assuming you have [Rust](https://rust-lang.org) installed, clone this repository and `cd` into it.

Run: `cargo run <optional path>`.

If a path is provided, an entire export will be created and written to the path specified automatically. This is useful for exporting in an automated fashion. You would likely want to format your automated export like such: `<authenticate into op> && opexport <path> && <command to encrypt the file>`.

Additionally, you want to make sure you have the [1Password CLI](https://developer.1password.com/docs/cli/) installed. Sign in to it. The exporter does not currently support authentication aside from biometrics, so you will have to do this manually for now. This is likely to change in the future.

Assuming you set up everything correctly, you will see a loading screen. This means the exporter is retrieving your data for export.

After loading, you will be taken to the interactive menu where you can filter account information you want excluded from the export.

Navigate through this menu using the controls, type out the export path, then hit enter to write the export data JSON to your disk.

## 1PUX format

Exports are made as a [1PUX](https://support.1password.com/1pux-format/) data file. Document downloading support will be added soon.

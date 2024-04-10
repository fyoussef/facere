# Facere

[![](https://img.shields.io/crates/v/facere.svg)](https://crates.io/crates/facere)

Facere is a simple project to automatically generate files for projects that use TypeScript.
It is a simple tool, but designed to bring ease and efficiency to developers in their day-to-day projects.
It's built using [Clap](https://crates.io/crates/clap) and [Rust](https://www.rust-lang.org/).

## Installation

Install the Facere tool on your system using command below.

<b>Shell (Linux):</b>

```sh
curl -O -L https://github.com/fyoussef/facere/releases/tag/v0.1.8/facere-x86_64-unknown-linux-gnu.tar.gz
```

<b>Shell (MacOS):</b>

```sh
curl -O -L https://github.com/fyoussef/facere/releases/tag/v0.1.8/facere-aarch64-apple-darwin.tar.gz
```

In the case of other systems, I recommend that you download directly from the following address.
See if your system is in the list and download it <a href="https://github.com/fyoussef/facere/releases/tag/v0.1.8">here</a>.

## Create Typescript files

After installing facere, follow the steps below to create your TypeScript files.

Create typescript file using class template:

```sh
facere cl my/file
```

When creating the file, facere will check if the `src` directory exists in the directory where you are running the command. Otherwise, it will create the file and directories inside the `src` folder.

To see all available templates, run the command below.

```sh
facere -h
```

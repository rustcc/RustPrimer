# Preparation

## Rust Download the source code (for racer use)

### Downloaded from github

`Git clone https: // github.com / rust-lang / rust.git`

### From the official website to download the source code package

Download: `https: // static.rust-lang.org / dist / rustc-nightly-src.tar.gz`

## Racer
racer is rust auto-completion and syntax analysis tools provided by the rust of lovers, are used to provide basic functions complement and definition skip function. Itself written entirely in rust, complement the function has been more perfect.

We can get it through the following ways:

### 1. cargo automatic installation
After the rust 1.5 version, which comes with the package installation tools have supported the cargo cargo install command, which can help us to get the latest version of `racer` by a simple way.
In linux and unix systems, you can

`` `Bash
sudo / usr / local / bin / cargo install --git 'https://github.com/phildawes/racer.git'
NOTE: It is now updated to: cargo install racer
`` `

To install the latest version of the racer. (Note: windows version please add, I have not really practiced.)

### 2. compile and install

Actually I recommend conditional user installation this way, because he always some combat operations over harvest.

#### 2.1 Download Source

First, we need to download the source code racer

`` `
git clone https://github.com/phildawes/racer.git
`` `

#### 2.2 compile

Then, go to the directory and then compile

`` `
cd racer && cargo build --release
`` `

In this way, we will get racer binaries in `target / release / racer` directory

#### 2.3 Setting Environment Variables

Finally, in order to better support racer, we need [rust official website] (https://www.rust-lang.org/downloads.html) to download the source files, download the files you corresponding platform. Then extract it to a directory.

Then, linux and unix users please your `.bashrc` or` .zshrc` last add the following line (note with your source files unzipped path to replace the following documents):
`` `
export RUST_SRC_HOME = / path / to / your / rust / source /
export RUST_SRC_PATH = $ RUST_SRC_HOME / src
`` `

And Windows users in your environment variable to add two environment variables: `RUST_SRC_HOME` to` your source file extraction path `;` RUST_SRC_PATH` of `% RUST_SRC_HOME% \ src \`

### 3. Test

Please re-open the terminal and into the path before closing.
Execute the following code:
linux:
`` `
./target/release/racer complete std :: io :: B
`` `
windows:
`` `
target \ release \ racer complete std :: io :: B
`` `
You will be prompted racer, which means racer has completed execution.


## Install rustfmt

`Cargo install rustfmt`

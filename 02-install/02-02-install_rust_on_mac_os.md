#Rust For Mac-os

Rust supports major operating systems, Linux, Mac and windows.

Rust provides two ways to install mac users:

### 1, download the installation package:

Download the installation package, then you need to check the current operating system is 64 or 32, respectively, to download the corresponding installation package.
Check your operating system use the following command in the terminal:

`Uname -a`

! [Mac-os-inofrmatoin] (../ image / 01-get-mac-os-information.png)

As shown in FIG red part, if it proves to be ** x86_64 ** 64-bit system, you need to [download] (https://static.rust-lang.org/dist/rust-1.5.0-x86_64-apple- darwin.pkg) 64-bit installation package;
If you need ** x86-32 ** [Download] (https://static.rust-lang.org/dist/rust-1.5.0-i686-apple-darwin.pkg) 32-bit installation package

And installing conventional software, the installation package can be run directly.


* In writing this chapter, the latest stable version 1.5.0 *

### 2, a key installation command line:
Rust provides simple one-button installation, the command is as follows:

`Curl -sSf https://static.rust-lang.org/rustup.sh | sh`

* This process may require you to enter the password several times *

You just open your command line code above it. (Note that you may need a ladder, otherwise they will encounter similar * Could not resolve host: static.rust-lang.org * Error)

### 3. Verify the installation:
If any of you have completed the above steps, use the following command:

`Rustc --version`

If you see the following message indicating that your installation was successful:

`Rustc 1.5.0 (3d7cd77e4 2015-12-04)`

If there is no * rustc * command prompt, then review whether there is something wrong somewhere you operate, please come back to go over the document.

**note**

In addition to the stable version, Rust also provides Beta and Nightly versions, download the following address:
https://www.rust-lang.org/downloads.html

If you do not want to install Rust on your computer, but you still like to try rust, then there is an on-line environment: http: //play.rust-lang.org/

China University of Science and Technology of the source image contains [rust-static] (http://mirrors.ustc.edu.cn/rust-static/), ladders temporary problem students can try to download the compiler from here; in addition. There Crates source, see [description here] (https://servers.ustclug.org/2016/01/mirrors-add-rust-crates/).

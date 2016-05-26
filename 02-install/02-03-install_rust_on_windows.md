#Rust For Windows

Rust supports major operating systems, Linux, Mac and Windows.

Rust is installed on Windows and other software you install on windows the same.

### 1, download the installation package:

  [Download] (https://www.rust-lang.org/downloads.html)

  Rust provides installation package multiple versions and multiple platforms, you can download the corresponding, here we [1.6.0] (https://static.rust-lang.org/dist/rust-1.6.0- x86_64-pc-windows-gnu.msi) stable version, for example.

### 2, the installation:
Double-click to download the installation package, as shown below:

! [Mac-os-inofrmatoin] (../ image / 01-install-on-windows-1st.png)

By default, rust will be installed to all users, select "Advanced", the user can specify the installation path and installation. Then click "install" Wait (the middle can be a security prompt, click Allow to, if you installed the 360 ​​and the like, need to be careful to block write to the registry 360) a few minutes.

### 3. Verify the installation:

After installation is complete, run the windows command line and enter:

`Rustc --version`

** Rustc 1.6.0 ** to see the beginning, that you install a success.

**note**

China University of Science and Technology of the source image contains [rust-static] (http://mirrors.ustc.edu.cn/rust-static/), ladders temporary problem students can try to download the compiler from here; in addition. There Crates source, see [description here] (https://servers.ustclug.org/2016/01/mirrors-add-rust-crates/).

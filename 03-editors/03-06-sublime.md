# Sublime

Sublime Text is a very famous text editor, which itself has a powerful plug-in mechanism. You can get more good support when using Sublime Text editor rust codes you can configure various plug-ins.

This paper shows installed in the Windows environment preinstalled rust, if you have not installed rust, please reference book [installation chapter] (../ 02-install / 02-03-install_rust_on_windows.md) installation rust .

## Installation

### Sublime Text3 installation

Please [Sublime Text3 official website] to select the machine for the current version of Sublime Text version downloaded and installed (http://www.sublimetext.com/3) on.

### Rust Installation

Please rust official website [download page] (https://www.rust-lang.org/downloads.html) Download rust source code archive and extract the installation locally at a later configuration step we will use to this path. If domestic download speed is too slow, consider using USTC [Mirror] (http://mirrors.ustc.edu.cn/) Download Source Package rust.

### Rust download and compile the code hints widget racer

Specific installation and compile content please see the section of this chapter [Preparations] (../ 03-editors / 03-01-before.md), racer.exe Keep in mind that the compiled file path, in a later part of the configuration we will use it.

## Configuration

### Sublime Text3 related plug-in installation

#### Install Package Control

Sublime Text3 Before installing the plug-in will need to install Package Control, if your editor has been installed Package Control skip this paragraph rust mounted directly related plug-ins.

You can view the [Package Control official website] (https://packagecontrol.io/installation) learn how to install.
You can also use `ctrl directly in the editor + ~` shortcut key to start the console, paste the following code and press Enter to install.

`` `Shell

import urllib.request, os, hashlib; h = '2915d1851351e5ee549c20394736b442' + '8bc59f460fa1548d1514676163dafc88'; pf = 'Package Control.sublime-package'; ipp = sublime.installed_packages_path (); urllib.request.install_opener (urllib.request.build_opener ( urllib.request.ProxyHandler ())); by = urllib.request.urlopen ( 'http://packagecontrol.io/' + pf.replace ( '', '% 20')) read ();. dh = hashlib .sha256 (by) .hexdigest ();! print ( 'Error validating download (got% s instead of% s), please try manual install'% (dh, h)) if dh = h else open (os.path. join (ipp, pf), 'wb') .write (by)

`` `

#### Rust related plug-ins

Use shortcuts in the editor `ctrl + shift + p` start a command-line tool, enter the Install Package and press Enter to enter the plug-in installation, select or enter the name of the plug and press Enter to complete the installation of plug-ins.

Using the plug-in installed Rust \ (rust syntax highlighting \), RustAutoComplete \ (rust code hints and auto-completion plugins \).

The installation has not been completed, we need to be configured into the local racer.exe RustAutoComplete plugin. Open the top of the editor of the Preferences tab, and then click Preferences-> Package Settings-> RustAutoComplete-> Settings-User to open RustAutoComplete profile, configure the following information in the file and save it.

`` `Shell

{
  "Racer": "E: /soft/racer-master/target/release/racer.exe",
  "Search_paths": [ "E: /soft/rustc-1.7.0/src"]
}

`` `

Wherein racer is the absolute path compiled racer.exe program. search_paths is the absolute path rust source file src directory.

After restarting the editor widget to take effect.

## Quick compilation

Sublime itself supports a variety of compiler system, select Rust or Cargo in Build System Tools tab of a build system, select and use the keyboard shortcut `ctrl + B` to quickly compile the code.

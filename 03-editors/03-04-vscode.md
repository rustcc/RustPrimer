# VS Code Installation and Configuration

[VS Code] (https://code.visualstudio.com/) Microsoft out of a source code editor, adhering to good genes used in a Microsoft IDE field is a potential for considerable editor / IDE .

VScode currently Rust also have good support.



## Downloads VScode

Open official website https://code.visualstudio.com/ download editor.

## Dependent

As the first section, ready `racer`,` rust source code `,` rustfmt` these three things, and configure the appropriate environment variable is not mentioned here.

## Install Rust extension RustyCode

1. Open VScode editor;
2. Press Ctrl + p to open the command panel;
3. In the upper part of the editor emerged out of the input box, enter the `ext install Rusty`, will automatically search for available plug-in, after searching out, click on the installation;
4. Complete.

### Manually download extension

Since the domestic network access the Internet often pit father, if you experience VScode inside, not been downloaded situation can be resolved as follows (in Linux for example, others similar):

1. Open a terminal, the user back to the root directory. `Cd ~`;
2. `cd .vscode`;
3. Check the current directory has no `extensions` directory, and if not,` mkdir extensions` build a. If so, skip this step;
4. `cd extensions`;
5. `git clone https: // github.com / saviorisdead / RustyCode.git`;
6. After the download is complete, etc., (re) open VScode;
7. Ctrl + p to open the command panel, enter the `ext` to see if there is a reminder `Rusty Code`, if prompted, the installation was successful;
8.'re done.


## Custom Configuration

You can custom configure some things, open `vi .vscode / settings.json`, can be modified as follows:

`` `
{
    "Rust.racerPath": null, // Specifies path to Racer binary if it's not in PATH
    "Rust.rustLangSrcPath": null, // Specifies path to / src directory of local copy of Rust sources
    "Rust.rustfmtPath": null, // Specifies path to Rustfmt binary if it's not in PATH
    "Rust.cargoPath": null, // Specifies path to Cargo binary if it's not in PATH
    "Rust.formatOnSave": false, // Turn on / off autoformatting file on save (EXPERIMENTAL)
}
`` `

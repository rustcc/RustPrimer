# Rust Version Manager: multirust

multirust Rust is a simple version management tools.

Project home page is: <https://github.com/brson/multirust>

## Features

* Manage multiple install the official version of the Rust binaries.
* Configure Rust directory-based tool chain.
* Install and update release of channels from Rust: nightly, beta and stable.
* Receive notifications from publishers channel updates.
* Install from the official version of history nightly tool chain.
* Install by specifying the stable version.
* Install additional std for cross-compilation.
* Install the custom tool chains.
* Separate each installed Cargo metadata.
* Hash value check download.
* Verify the signature (if GPG exists).
* http.
* Depends only on the bash, curl and common unix tools.
* Support for Linux, OS X, Windows (via MSYS2).

## Installation

`` `
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
`` `

This command will compile and install multirust, the installation process may be prompted to enter your sudo password. He then download and install the stable version of the tool chain, when executed rustc, rustdoc and cargo, he will be configured as the default tool chain.

After you install the tool chain will be installed into `~ / .multirust / toolchains` on` * nix`.

## Uninstall

`` `
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh -s - --uninstall
`` `

## Usage

Will be installed after a multirust command, use the command comes more helpful tips, you can quickly locate the required function.

### Help

Run `multirust -h` you will get the following message:

`` `
❯ multirust -h
# Usage: multirust <command> [--verbose] [--version]
#
# Commands:
#
# Default Set the default toolchain
# Override Set the toolchain override for the current directory tree
# Update Install or update a given toolchain (for example, "stable", "beta", "nightly")
# Show-override Show information about the current override
# Show-default Show information about the current default
# List-overrides List all overrides
# List-toolchains List all installed toolchains
# Remove-override Remove an override, for current directory unless specified
# Remove-toolchain Uninstall a toolchain
# List-available-targets
# List targets available to install
# Add-target Add additional compilation targets to an existing toolchain
# Run Run a command in an environment configured for a toolchain
# Delete-data Delete all user metadata, including installed toolchains
# Upgrade-data Upgrade the ~ / .multirust directory from previous versions
# Doc Open the documentation for the currently active toolchain
# Which Report location of the currently active Rust tool.
# Help Show help for this command or subcommands
#
# Use `multirust help <command>` for detailed help.
#
`` `

When prompted, `multirust help <command>` to view the help subcommand, look at these basic help documentation is sufficient.

The following highlights some commonly used commands.

`Multirust default beta | stable | nightly` configure the default tool chain.

`Multirust override beta | stable | nightly` configure a directory and its subdirectories tool chain.

`Multirust show-default` current default tool chain information.

`Multirust show-override` tool chain display information of the current directory override.

`Multirust list-toolchains` Show all toolchain installed.

`Multirust list-overrides` displays all override tool chain.

`Multirust update` Update tool chain for all publishing channels.

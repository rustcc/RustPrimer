The cheese is good, we meet again. Before Chapter 5, we discussed some of the basic skills commonly used in cargo. Through the study of Chapter 5, we can basically solve the daily project development encountered most of the problems. But in fact, cargo provided to us the use of the function is not limited to this. I just want to say a word: cargo very good very strong, and far stronger than you think
This chapter will explore some of the details of cargo, which includes the following aspects:
- Project version declaration and management based on semantic version
- cargo toml description file configuration field for detailed reference

# Based on the semantic version of the project version of the statement and management
When we use the toml description file to configure the project, we often encounter project version declarations and management issues, such as:

`` `Toml
[Package]
Name = "libevent_sys"
Version = "0.1.0"

[Dependencies]
Libc = "0.2"

`` ``
The value of the version field in the package, and the value of the libc field in the dependencies section. These values ​​are written to the problem of semantic versioning. Semantic version control is a set of simple rules and conditions to constrain the version number of the configuration and growth. These rules are based on (but not limited to) the conventions that have been widely used by various closed, open source software. In simple terms, semantic version control follows the following rules:

- Version format: Main version number. Minor version number. Revision number, version number Increment rules are as follows:

1. major version number: When you do not modify the API,
2. Sub-version number: When you do a backward-compatible functionality added,
3. Revision number: When you do a down-compatible problem fix.

- The first version number and version of the compilation information can be added to the "major version number. Minor version number. Revision number" behind, as an extension.

For more detailed questions about semantic version control, we can refer to [here] (http://semver.org/lang/), I will not repeat them.

# Cargo's toml description file configuration field for detailed reference
## [package] paragraph
Han did not say, and directly on the example, we note that I explained in the Chinese example, personally feel that this comparison at a glance:
`` `Toml
[Package]
 # The name of the package, please use this name if you need to refer to this package somewhere else.
Name = "hello_world"

# Current version number, here follows the semver standard, which is the semantic version control standard.
Version = "0.1.0" # the current version, obeying semver

# Software all author list
Authors = ["you@example.com"]

# Very useful for a field, if you want to customize your own building workflow,
# In particular, when you want to call external tools to build other native languages ​​(C, C ++, D, etc.) when developing packages.
# At this point, the custom build process can use the rust language, written in the "build.rs" file.
Build = "build.rs"

# Explicitly declare which files in the package folder are excluded from the project's build process,
# Which files are included in the project's build process
Exclude = ["build / ** / *. O", "doc / ** / *. Html"]
Include = ["src / ** / *", "Cargo.toml"]

# Enable this field to prevent this error when an error occurs when the package is released to the public repository.
Publish = false

# A brief introduction to the package.
Description = "..."

# The following fields indicate more information about the package repository
Documentation = "..."
Homepage = "..."
Repository = "..."

# As the name suggests, this field points to the file is the legendary ReadMe,
# Also, the contents of this file will eventually be saved in the registry database.
Readme = "..."

# Used to sort and retrieve keywords.
Keywords = ["...", "..."]

# The license for the package must be a known standard license that has been listed in the cargo warehouse.
License = "..."

# File The non-standard license file corresponds to the file path.
License-file = "..."
`` ``

## depends on the detailed configuration
The most straightforward way to explore in the first five chapters, not here to repeat, for example:
`` `Toml
[Dependencies]
Hammer = "0.5.0"
Color = "> 0.6.0, <0.8.0"
`` ``

The platform-dependent dependency definition format is different, except that it needs to be defined under the [target] field. E.g:

`` `Toml
# Note that here cfg can use not, any, all other operators such as any combination of key-value pairs.
# And this usage only supports cargo 0.9.0 (rust 1.8.0) or later.
# If the windows platform, you need this dependency.
[Target.'cfg (windows) '. Dependencies]
Winhttp = "0.4.0"

[Target.'cfg (unix) 'dependencies]
Openssl = "1.0.1"

# If this is a 32-bit platform, you need this dependency.
[Target.'cfg (target_pointer_width = "32") '. Dependencies]
Native = {path = "native / i686"}

[Target.'cfg (target_pointer_width = "64") '. Dependencies]
Native = {path = "native / i686"}

# Another way is to list the full name of the platform description
[Target.x86_64-pc-windows-gnu.dependencies]
Winhttp = "0.4.0"
[Target.i686-unknown-linux-gnu.dependencies]
Openssl = "1.0.1"

# If you are using a custom platform, include the full path of the custom platform file in double quotation marks
[Target. "X86_64 / windows.json" .dependencies]
Winhttp = "0.4.0"
[Target. "I686 / linux.json" .dependencies]
Openssl = "1.0.1"
Native = {path = "native / i686"}
Openssl = "1.0.1"
Native = {path = "native / x86_64"}

# [Dev-dependencies] The format of the paragraph is equivalent to the [dependencies]
# The difference is that the dependencies of the [dependencies] statement are used to build the package,
# And [dev-dependencies] The declarations of paragraph declarations are used only for building tests and performance evaluations.
# In addition, the dependencies of the [dev-dependencies] paragraph declaration are not passed to other items that depend on the package
[Dev-dependencies]
Iron = "0.2"

`` ``

## custom compiler call mode template detailed parameters
Cargo built-in five compiler call template, respectively, dev, release, test, bench, doc, respectively, for the definition of different types of targets generated when the compiler parameters, if we want to change these compiler templates, you can define the corresponding field Values, for example (Note: the values ​​listed in the following example are the system defaults for this template field):

`` `Toml
# Develop the template, corresponding to the `cargo build` command
[Profile.dev]
Opt-level = 0 # Controls the compiler's --opt-level parameter, which is the optimization parameter
Debug = true # Controls whether the compiler turns on the `-g` parameter
Rpath = false # Controls the compiler's `-Crpath` parameter
Lto = false # Control the `-C lto` parameter, which affects the generation of executable and static libraries,
Debug-assertions = true # Controls whether the debug assertion is on
Codegen-units = 1 # Controls the `-C codegen-units` parameter of the compiler. Note that this value is ignored when `lto = true`

# Release the template, corresponding to the `cargo build --release` command
[Profile.release]
Opt-level = 3
Debug = false
Rpath = false
Lto = false
Debug-assertions = false
Codegen-units = 1

# Test the template, corresponding to the `cargo test` command
[Profile.test]
Opt-level = 0
Debug = true
Rpath = false
Lto = false
Debug-assertions = true
Codegen-units = 1

# Performance evaluation template, corresponding to the `cargo bench` command
[Profile.bench]
Opt-level = 3
Debug = false
Rpath = false
Lto = false
Debug-assertions = false
Codegen-units = 1

# Document template, corresponding to `cargo doc` command
[Profile.doc]
Opt-level = 0
Debug = true
Rpath = false
Lto = false
Debug-assertions = true
Codegen-units = 1

`` ``
It should be noted that when the compiler is called, only the template file located at the topmost package is valid, and the template definitions for the other subpackages or dependencies will be overwritten by the template of the top-level package.

## [features] paragraph
[Features] The fields in the paragraph are used for conditional compilation options or optional dependencies. E.g:

`` `Toml
[Package]
Name = "awesome"

[Features]
# This field sets the default selection list for optional dependencies,
# Note that the "session" here is not a package name,
# But another featrue field session
Default = ["jquery", "uglifier", "session"]

# Such as the value of the empty feature is generally used for conditional compilation,
# Similar to `# [cfg (feature =" go-faster ")]`.
Go-fast = []

# This feature relies on the bcrypt package,
# The advantage of this package is that you can add optional items to secure-password this feature in the future.
secure-password = ["bcrypt"]

# The session field here imports the session field in the feature section of the cookie package
Session = ["cookie / session"]

[Dependencies]
# The necessary dependency
Cookie = "1.2.0"
Oauth = "1.1.0"
Route-recognizer = "= 2.1.0"

# Optional dependencies
Jquery = {version = "1.0.2", optional = true}
Uglifier = {version = "1.5.3", optional = true}
Bcrypt = {version = "*", optional = true}
Civet = {version = "*", optional = true}
`` ``

If other packages depend on using the awesome package described above, you can write in its description file:
`` `Toml
[Dependencies.awesome]
Version = "1.3.5"
Default-features = false # Disable awesome's default features
Features = ["secure-password", "civet"] # use the features listed here
`` ``
The following rules apply when using features:
- The feature name can not conflict with the name of the package that appears in this description file
- In addition to the default feature, all other features are optional
- features can not be included with each other
- Development dependencies can not be included
The features groups can only rely on optional packages

An important use of features is that when the developer needs to release the package in the final, in the construction can be declared exposed to the end user's features, which can be achieved through the following order:

`` ``
$ Cargo build --release --features "shumway pdf"
`` ``
## About testing
When you run the cargo test command, cargo will do the following:
- compile and run the unit test in the package source code marked by # [cfg (test)]
- Compile and run the document test
- Compile and run the integration test
- compile examples

## Configure the build target
All of the fields, such as [[bin]], [lib], [[bench]], [[test]], and [[example]], provide a similar configuration to illustrate how the build target should be built. For example, the field values ​​in the [lib] paragraph are the default values ​​in the following example):

`` `Toml
[Lib]
# Library name, the default is the same as the project name
Name = "foo"

# This option is used only for the [lib] paragraph, which determines how the build target is to be built,
# Can take dylib, rlib, staticlib one of three values, that generate dynamic library, r library or static library.
Crate-type = ["dylib"]

The # path field declares the relative path of this build target relative to the cargo.toml file
Path = "src / lib.rs"

# Unit Test Switch Options
Test = true

# Document Test Switch Options
Doctest = true

# Performance evaluation switch options
Bench = true

# Document Generation Switch Options
Doc = true

# Whether to build a switch option for the compiler plugin
Plugin = false

# If set to false, `cargo test` will ignore the --test parameter passed to rustc.
Harness = true
`` ``

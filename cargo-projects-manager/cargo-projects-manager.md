ntroduction to
Once the time, the organization and management of the project code was a nightmare for the use of the apes that used the C ++ C ++ language. In order to solve the problem of `C / C ++ 'project management, the ape God tried all sorts of ways to develop a variety of project management tools from the beginning of` automake` to the later `cmake`,` qmake` And so on, but the results are not satisfactory, often to solve some problems, but the introduction of more problems, `c / C + +` ape who often fall into the language itself, but also master the complex construction tool syntax Dilemma. Coincidentally, `java` project code organization and management tools` ant` and `maven` also have the same problem. Complex project management configuration parameters, often let the ape know what to do.

As a modern language, `rust` naturally to abandon the Stone Age project code management methods and means. `Rust` project team for the ape to provide a super killer` cargo`, to solve the project code management brought about by the interference and confusion. Used a `node.js` ape, should be` node.js` in the artifact `npm`,` grunt`, `gulp` and other tools impressed. As a new generation of static language leader, `rust` official reference to the advantages of existing language management tools, so it produced a` cargo`.

In short, as a `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` `` ` The At the same time, with the `rust` language and its compiler` rustc` their own characteristics of a close combination, it can be said that both the language itself intimate love, but also `rust` ape intimate small cotton jacket, who knows who.
Nonsense is not to say, directly on the example and a variety of high-definition no horse map.
# Cargo entry
First of all, of course, still nonsense, to use cargo, naturally first to install cargo. There are three ways to install cargo. For the first two methods, see the installation method for rust, because the cargo tool is the official orthodox origin, which is of course included in the official distribution. The third method is to build from the source store of the [`cargo`] (https://github.com/rust-lang/cargo) project. Oh, My God. It is nonsense.

Well, suppose you have installed the cargo, we and I learn from the hands. Of course, the ape world, hands are generally stereotyped - that is `hello world` Dafa.
Enter in the terminal

`` `Bash
$ Cargo new hello_world --bin
`` ``

The above command uses ** cargo new ** in the current directory under the new cargo project management of the rust project, the project name is hello_world, - bin that the project will generate executable files. The specific project directory structure is as follows:

`` `Bash
$ Cd hello_world
$ Tree.
The
├ ─ ─ Cargo.toml
└ ─ ─ src
    └── main.rs

1 directory, 2 files
`` ``

We can enter the above command in the terminal, knock out the Enter key after you can see the results, or directly to the editor or file manager to observe.
Open the main.rs file, you can see, cargo new command for us automatically generated hello_world operation must be all the code:

`` `Rust
Fn main () {
    Println! ("Hello, world!");
}
`` ``

Well, impatient ape may have been unable to wait to take off their pants, well, we first to build and see how magical cargo, in the terminal input:

`` `Bash
$ Cargo build
`` ``

Wait a moment, cargo will automatically build for us all the necessary high-definition applications, for this hands, the buffer will not be more than 5 seconds, 12 seconds 88 players to hold back.

`` `Bash
$ Cargo run
    Running `target / debug / hello_world`
Hello, world!

`` ``

See what, see what, scared to have a wood, scared to have a wood. Okay, cargo is that simple.

Of course, that cargo the United States, not just simple so simple, cargo is simple, but very powerful. How strong is it? The It can be said that the basic development and management of the necessary means, cargo have. Very small and very strong, both strong and athletic, without horses, learning curve is almost zero.

# Based on the cargo of the rust project organizational structure
This time do not talk nonsense, the first high-definition no horse map:

[Cargo project organizational structure] (../ image / project-structure.png)

The default structure of the cargo described above is explained as follows:

##### `cargo.toml` and` cargo.lock` files are always located in the project root directory.
##### The source code is located in the `src` directory.
##### The default library entry file is `src / lib.rs`.
##### The default executable file entry file is `src / main.rs`.
##### Other optional executables are located in `src / bin / *. Rs` (where each rs file corresponds to an executable file).
##### External test source code files are located in the `tests` directory.
##### The sample program source code file is located in `examples`.
##### The benchmark source code file is located in the `benches` directory.

Well, we must bear in mind these default rules, it is best to follow this model to organize their own rust projects.

# Cargo.toml and cargo.lock
`Cargo.toml` and` cargo.lock` are the two files of the cargo project code management, and all the activities of the cargo tool are based on both files.

`Cargo.toml` is a cargo-specific project data description file. For the ape, the` cargo.toml` file stores all the information for the project, which is directly directed to the rust ape. If you want your own project to follow Expect the way to build, test and run, then, must be in a reasonable way to build 'cargo.toml'.

And the `cargo.lock` file is not directly for the apes, and the ape does not need to modify the file directly. Lock file is the cargo tool according to the same project toml file generated by the project depends on the detailed list of documents, so we generally do not need him, just need to face the `cargo.toml` file line on the line.

`` `Toml
[Package]
Name = "hello_world"
Version = "0.1.0"
Authors = ["fuying"]

[Dependencies]
`` ``
The toml file is made up of paragraphs such as [package] or [dependencies], each of which is made up of multiple fields. These paragraphs and fields describe the basic information of the project organization, such as the [package] paragraph in the above toml file Describes the `hello_world` project itself, including the project name (corresponding to the name field), the project version (corresponding to the version field), the author list (corresponding to the authors field), etc .; [dependencies] Depend on what the project is.

Let's take a look at the meaning of the paragraphs and fields used in the toml description file.

# Package paragraph
The [package] paragraph describes the names of the items that the software developer has defined for each item of the project, such as the [name] field. The [version] field defines the current version of the project, [authors] defines all of the items Author, of course, [package] paragraphs not only contain these fields, [package] paragraphs of the other optional fields see cargo parameter configuration section.

# Define project dependencies
The biggest advantage of using the cargo tool is the ability to manage the various dependencies of the project in a convenient, unified and flexible manner. This is also one of the important goals of using cargo to manage the project of rust. In the cargo's toml file description, the various dependencies of the project are described primarily through various dependency paragraphs. Toml in the commonly used dependencies include a few:
- based on rust official warehouse crates.io, through the version description to describe:
- Based on the project source code git repository address, through the URL to describe:
- Absolute path or relative path based on local project, described by Unix-like path:
These three forms of specific wording are as follows:

`` `Toml
[Dependencies]
Typemap = "0.3"
Plugin = "0.2 *"
Hammer = {version = "0.5.0"}
Color = {git = "https://github.com/bjz/color-rs"}
Geometry = {path = "crates / geometry"}
`` ``

In the above example, the behavior of the method of one of the 2-4, the fifth line for the method of writing, the first three acts of the method of writing three.
These three wordings are useful, and if the project requires the use of crates.io official warehouse to manage project dependencies, it is recommended to use the first method. If the project developer is more inclined to use the latest source code in the git repository, you can use method two. Method 2 is also often used when the dependency of an official repository is not passed when the alternatives are passed. Method 3 is mainly used for source code that is locally dependent.

# Define an integrated test case

Cargo Another important feature, the software development process is necessary and very important part of the test link, and through the code attribute declaration or toml file description to test the management. Where the unit test is described by the `# [test]` attribute before the test code part of the project code, and the integration test is usually described by the [[test]] paragraph in the toml file.
For example, assume that the integration test file is located under the tests folder, then toml can write like this:
`` `Toml
[[Test]]
Name = "testinit"
Path = "tests / testinit.rs"

[[Test]]
Name = "testtime"
Path = "tests / testtime.rs"

`` ``
In the above example, the name field defines the name of the integration test, and the path field defines the path of the integration test file relative to the toml file.
Look, define the integration test is so simple.
have to be aware of is:

- If the integration test entry is not defined in Cargo.toml, each rs file under the tests directory (excluding subdirectories) is treated as an integrated test portal.
- If the integrated test entry is defined in Cargo.toml, the rss defined are entries, and no default test entries are specified by default.

# Define project examples and executable programs
** We introduced the three functions commonly used in cargo project management, as well as two frequently used functions: description of example use cases and description of bin use cases. The description method is similar to the test case description method. However, this time the paragraph name '[[test]]' is replaced by '[[example]]' or '[[bin]]', respectively. E.g:**

`` `Toml
[[Example]]
Name = "timeout"
Path = "examples / timeout.rs"

[[Bin]]
Name = "bin1"
Path = "bin / bin1.rs"

`` ``
The examples and bins declared in the '[[example]]' and '[[bin]]' paragraphs need to be run with 'cargo run --example NAME' or 'cargo run --bin NAME', where NAME corresponds to You define the name in the name field.

# Build, clean up, update and install
It is an important aspect to understand the writing of the toml description file. Another important aspect is that the cargo tool itself is a useful tool for our program ape. If you are interested, you can enter the 'cargo --help' in the terminal. The most commonly used command for development is 'cargo build', which is used to build the project. In addition, the 'cargo clean' command clears all content in the target folder; 'cargo update' retrieves and updates information about the dependencies based on the toml descriptor file and writes the lock file, such as changes to the dependency version, etc. Etc; 'cargo install' can be used for actual production deployments. These commands are very useful in real development deployments.

** cargo For more detailed usage see ['28. Cargo parameter configuration '] (../ cargo-detailed-cfg / cargo-detailed-cfg.md) **
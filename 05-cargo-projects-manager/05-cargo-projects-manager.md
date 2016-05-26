# Cargo Profile
Once upon a time, for the use of used to `C / C ++` language of apes, the organization and management of the project code is absolutely a nightmare. In order to solve management problems `C / C ++` project, ape gods tried different solutions, has developed a myriad of project management tools, from the beginning to the later `automake`` cmake`, `qmake` etc., but the results are not satisfactory, often to solve some problems, but introduces more problems, `C / C ++` apes can often get caught in the grasp of the language itself, it must also master complex syntax builder dilemma. Similarly, project management tools and code organization `ant` and` maven` `java` also the same problem. Complex project management configuration parameters allow apes often overwhelmed.

As a modern language, `rust` natural to abandon the ways and means Stone Age project code management. `Rust` project offers super big kill` cargo` ape for you to solve the interference caused by the project code management and confusion. Used `node.js` ape who should` node.js` the artifact `npm`,` grunt`, `gulp` tools such impressive. As a leader in a new generation of static languages, `rust` official language with reference to the advantages of the existing management tools, thus a` cargo`.

All in all, words and short, as `rust` organization code management tool,` cargo` provides a range of tools, from the establishment of the project, to build the test run until the deployment of project management `rust` provide as complete It means. Meanwhile, `rust` language and compiler` rustc` various characteristics itself closely, it can be said is both intimate love of language itself, but also the `rust` apes intimate small cotton-padded jacket, with who knows who.
Nonsense is not to say, directly on the examples and various HD horseless FIG.
# Cargo entry
First, of course, nonsense, to use cargo, naturally you must first install the cargo. There are three ways to install cargo, the first two methods, see rust installation method, because cargo is the official tool orthodox background, of course, included in the official distribution package. The third method from [ `cargo`] (https://github.com/rust-lang/cargo) project source repository to build. Oh, My God. Indeed nonsense.

Well, assuming that you have installed the cargo, join me to learn about hands-style. Of course, the ape world, hands-style generally stereotyped - that is, `hello world` Dafa.
In a terminal

`` `Bash
$ Cargo new hello_world --bin
`` `

Use these commands ** cargo new ** new directory in the current project-based cargo management rust project, the project name for hello_world, - bin indicates that the project will generate an executable file. Generate specific project directory structure is as follows:

`` `Bash
$ Cd hello_world
$ Tree.
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
`` `

You can enter these commands in a terminal, then knocked the Enter key to see the above results, or go directly to the editor or File Manager to observe can.
Open main.rs file, you can see, cargo new command automatically generates all the code necessary to run hello_world for us:

`` `Rust
fn main () {
    println ( "Hello, world!")!;
}
`` `

Well, they may have been impatient ape can not wait to take your pants off, okay, let's build and see how amazing cargo, enter in a terminal:

`` `Bash
$ Cargo build
`` `

All wait a moment, cargo will automatically apply needed for us to build a good definition, for this hands-style, the buffer will not be more than 5 seconds, 12 seconds 88 players to be choked off.

`` `Bash
$ Cargo run
    Running `target / debug / hello_world`
Hello, world!

`` `

What to see, what to see, scared urine there are wood, with wood have scared urine. Well, cargo is so simple.

Of course, the United States said the cargo, and not just a simple simple, cargo is simple, but very powerful. How powerful? ? It can be said, basically rust development and management tools required, cargo there. Very small and very powerful, both have strong moral integrity, without horses, the learning curve is almost zero.

Item # rust-based cargo organizational structure
This is not to say nonsense, the first high-definition No Horses:

! [Cargo project organization] (../ image / 05-project-structure.png)

Cargo above the default project structure is explained as follows:

##### `Cargo.toml` and` cargo.lock` project file is always located in the root directory.
##### Source code is located under the `src` directory.
##### Default library file entry is `src / lib.rs`.
##### Default executable file entry is `src / main.rs`.
##### Other optional executable file located in `src / bin / *. Rs` (where each file corresponds rs an executable file).
##### External test source code files are located in `tests` directory.
##### Sample source code files are located in `examples`.
##### Benchmark source files are located under the `benches` directory.

Well, we must keep in mind these default rules, according to this model to best organize their rust project.

# Cargo.toml and cargo.lock
`Cargo.toml` and` cargo.lock` two files are the core code management project cargo, all cargo activity tools are based on these two documents.

`Cargo.toml` is cargo-specific project data description file for apes are concerned,` cargo.toml` file stores all information about the project, which is directly facing the rust apes, apes if you want your project in accordance with rust desirable way to build, test and run, then you must build 'cargo.toml' in a reasonable manner.

The `cargo.lock` file is not directly facing the apes, apes do not need to go directly to modify this file. lock files are cargo tools rely on a detailed list of files based on the same project file toml generated project, so we generally do not take care of him just in front of `cargo.toml` file line and on the line.

`` `Toml
[Package]
name = "hello_world"
version = "0.1.0"
authors = [ "fuying"]

[Dependencies]
`` `
toml file is generated by such [package] or [dependencies] such paragraphs, each paragraph has a plurality of fields, and these fields paragraphs describe the basic information on project organization, such as the aforementioned toml file [package] Paragraph `hello_world` project described some of the information itself, including the project name (corresponding to the name field), project (corresponding to the version field), the list of authors (corresponding to the authors field) and the like; [dependencies] paragraphs describe the project` hello_world` It depends what project.

Let us look at common sense toml profile paragraphs, and fields.

# Package paragraph
[Package] paragraphs describe software developers for various metadata of the project description, such as [name] field defines the name of the project, [version] field defines the project's current version, [authors] defines all of the project oF course, [package] paragraph contains not only these fields, [package] other optional fields see paragraph cargo parameters section.

# Define project dependencies
Use cargo biggest advantage is that the tool can be convenient, unified and flexible management of the various project dependencies. This is the use of cargo to rust of the project one of the important goals of management. In the cargo of toml file descriptions, mainly to describe various dependencies of the project through various dependencies paragraphs. toml commonly used include passages about several dependency:
- Based on official rust warehouse crates.io, described by the release notes:
- Project-based source git repository address, described by URL:
- Absolute or relative path to the local project-based, Unix-like path through the model to describe:
These three forms specific wording as follows:

`` `Toml
[Dependencies]
typemap = "0.3"
plugin = "0.2 *"
hammer = {version = "0.5.0"}
color = {git = "https://github.com/bjz/color-rs"}
geometry = {path = "crates / geometry"}
`` `

The above example, the wording of a method of behavior 2-4, 5th behavior Method Two wording, the sixth behavioral approach wording III.
These three have written useful, if the project requires crates.io official repository to manage project dependencies, it is recommended to use the first method. If the project developers prefer to use git repository latest source code, you can use the second method. Method two is also often used as an alternative dependencies compiled without the official repository when. Method three is mainly used in the local source code dependencies.

#define Integration test cases

Another important function of cargo, ie the software development process is necessary and very important part of integration testing, and the test to be managed by the code attribute declaration or toml file description. Among them, mainly through the unit test before the test code section of the project code with `# [test]` attributes to describe, and integration testing is usually described by toml file [[test]] paragraph.
For example, assume that the integration test files are in the file folder tests, the toml would write:
`` `Toml
[[Test]]
name = "testinit"
path = "tests / testinit.rs"

[[Test]]
name = "testtime"
path = "tests / testtime.rs"

`` `
The above example, name field defines the name of the integration testing, path field defines the integration testing with respect to this document toml file path.
See the definition of integration testing is so simple.
have to be aware of is:

- If integration testing entry is not defined in Cargo.toml years, then each rs file tests directory (not including subdirectories) under the integration testing is deemed entrance.
- If Cargo.toml in the definition of integration testing entry, then those rs entrance is defined not specify any default integration test entrance.

# Define the project examples and executable programs
** Above us the cargo project management commonly used in three functions, there are two frequently used functions: example use case description and use case descriptions bin. Which describes a method and test method described in similar cases. However, this time the name of the paragraph '[[test]]' are replaced with: '[[example]]' or '[[bin]]'. E.g:**

`` `Toml
[[Example]]
name = "timeout"
path = "examples / timeout.rs"

[[Bin]]
name = "bin1"
path = "bin / bin1.rs"

`` `
For '[[example]]' and '[[bin]]' paragraph statement examples and bins, through 'cargo run --example NAME' or 'cargo run --bin NAME' run, which corresponds to the NAME your name in the name field definition.

# Construction, cleaning, and installing updates
Understand the wording toml description files is an important aspect. Another important aspect is the variety of useful tools cargo itself provides tools for our program ape. If you're interested, you've entered in the terminal 'cargo --help' to view. Wherein the development of the most commonly used command is 'cargo build', for building projects. In addition, 'cargo clean' command to clean up the target folder all the contents; 'cargo update' to re-retrieve and update their various dependencies according toml description file and write lock file, for example, rely on updated versions of items and other changes and so on; 'cargo install' can be used in the actual production deployment. These commands are in the actual development and deployment in both is very useful.

** Cargo more detailed usage, see ['28. Cargo configuration parameters'] (../ 28-cargo-detailed-cfg / 28-01-cargo-detailed-cfg.md) **

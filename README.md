# RustPrimer
Introductory Primer for Rust
(English version)

## Google Translated.
Straight Copy Pasta Son

<small> @archae0pteryx

## [Table of Contents]()

### 1: :[Getting aquainted](./01-1st-glance/README.md) <small>(daogangtang, tiansiyuan)</small>
### 2: :[Installation](./02-install/02-00-preface.md) <small>(marvin-min)
- [Linux](./02-install/02-01-install_rust_on_linux.md)
- [Mac](./02-install/02-02-install_rust_on_mac_os.md)
- [Windows](./02-install02-03-install_rust_on_windows.md)
- [Version Managers](./02-install/02-04-multirust.md)

### 3: :[Text Editors](./03-editors/03-00-preface.md)
- [Configuration](./03-editors/03-01-before.md) <small>(Wayslog)
- [VIM](./ 03-editors/03-02-vim.md) <small> (Wayslog)
- [Emacs](./ 03-editors/03-03-emacs.md) <small> (Tiansiyuan)
- [VSCode](./03-editors/03-04-vscode.md) <small> (daogangtang)
- [Atom](./ 03-editors/03-05-atom.md) <small> (wayslog)
- [Sublime](./ 03-editors/03-06-sublime.md) <small> (domty)
- [Visual Studio](./ 03-editors/03-07-visualstudio.md) (marvinguo)
- [Spacemacs](./ 03-editors/03-10-spacemacs.md) <small> (wayslog)

### 4: :[Rust -> One Hour QuickStart](./04-quickstart/ 04-00-intro.md)
- [Your First Rust Programe](./04-quickstart/ 04-01-hello-world.md)
- [Simple Math](./04-quickstart/04-02-basic-math.md)
- [Cheat Sheet](./04-quickstart/04-03-cheet-sheet.md)

### 5: :[Cargo Project Manager](./05-cargo-projects-manager/ 05-cargo-projects-manager.md) <small>(Fuyingfuying)
### 6: :[Basic Rust Structure](./06-flow/06-00-preface.md) <small> (Daogangtang)
- [Workflow](./06-flow/06-01-comment.md)
- [Conditionals](./06-flow/06-02-condition.md)
- [Loops](./06-flow/06-03-repeatition.md)

### 7: :[Types, Operators and Strings](07-type/ 07-00-preface.md) <small> (Wayslog)
- [Base Types](07-type/07-01-types.md)
- [Composite Types](07-type/07-02-compound-types.md)
- [String Class](07-type/07-03-strings.md)
- [Base Operators and String Formatting](07-type/ 07-04-operator-and-format.md)

### 8: :[Functions](./08-function/08-00-overview.md) <small> (qdao)
- [Function Parameters](./08-function/08-01-arguement.md)
- [Function Return Values](./ 08-function/ 08-02-return_value.md)
- [Statements and Expressions](08-function/ 08-03-statement_expression.md)
- [Higher-Order Functions](08-function/ 08-04-high_order_function.md)

### 9: :[Pattern Matching](09-match/09-00-overview.md) <small> (wayslog)
- [Matching Keywords](09-match /09-01-match.md)
- [Mode](09-match/09-02-pattern.md)

### 10: :[Trait (Feature)](10-trait/10-00-overview.md) <small> (JohnSmithX)
- [Trait Keyword](10-trait/10-01-trait.md)
- [Trait Object](10-trait/10-02-trait-object.md)

### 11: :[Generics](11-generics/11-01-generics.md) <small> (stormgbs)
### 12: :[Variability, Ownership, Leases and Lifetimes Oh! My](12-ownership-system/12-00-ownership_system.md) <small> (stormgbs)
- [Ownership](12-ownership-system/12-01-ownership.md)
- [Citation and Borrowing](12-ownership-system/ 12-02-borrowing_references.md)
- [Lifecycles](12-ownership-system/12-03-lifetimes.md)

### 13: :[Closure](13-closure/13-00-overview.md) <small> (qdao)
- [Closure Syntax](13-closure/13-01-syntax.md)
- [Closure Schieve](13-closure/13-02-implementation.md)
- [Closures as Parameters and Return Values](13-closure/ 13-03-as_argument_return_value.md)

### 14: :[Collection Types (Collections)](14-collections/ 14-00-overview.md) <small> (wayslog)
- [Dynamic Array](14-collections/14-01-vec.md)
- [Hash Tables](14-collections/14-02-hashmap.md)

### 15: :[Iterators](15-iterator/15-00-overview.md) <small> (wayslog)
- [Iterations, Adapters, Consumers](15-iterator/ 15-01-iterator.md)

### 16: :[Modules and Packages: Preface](16-modules/ 16-00-preface.md) <small> (daogangtang)
- [Module (module) and Packages (crate)](16-modules/ 16-01-module.md)
- [Prelude](16-modules/ 16-02-prelude.md)

### 17: :[Options, Results, and Error Handling](17-error-handling/17-01-option-result.md) <small> (JohnSmithX)
### 18: :[Macro Systems](18-macro/18-01-macro.md) <small> (tennix)
### 19: :[The Heap, Stack, and... The  Box](./19-heap-stack/heap-stack.md) <small> (tennix)

### 20: :[Some Smart Pointers](./20-rcarc/20-00-preface.md) <small> (daogangtang)
- [Rc, Arc](./20-rcarc/20-01-rcarc.md)
- [Mutex, RwLock](./ 20-rcarc/20-02-mutex.md)
[Cell, RefCell](./20-rcarc/20-03-cell.md)

### 21: :[Type System of Several Common Traits](./ 21-intoborrow/21-00-preface.md) <small> (daogangtang)
- [Into / From Its Application in String and & str System Conversion On](./21-intoborrow/21-01-into.md)
- [AsRef, AsMut](./21-intoborrow/21-02-asref.md)
- [Borrow, BorrowMut, ToOwned](./21-intoborrow/21-03-borrow.md)
- [Deref and Deref coercions](./21-intoborrow/21-04-deref.md)
- [Cow on a String and Its Application & str](./ 21-intoborrow / 21-05-cow.md)

### 22: :[Send and Sync](./22-marker/21-01-sendsync.md) <small> (daogangtang)
### 23: :[Concurrent, Parallel, Multi-Threaded Programming](./23-concurrency-parallel-threads/ 23-00-preface.md) <small> (anzhihun)
- [Threads](./23-concurrency-parallel-threads/ 24-01-thread.md)
- [Messaging](./23-concurrency-parallel-threads/ 24-02-message-passing.md)
- [Shared Memory](./23-concurrency-parallel-threads/ 24-03-share-memory.md)
- [Synchronization](./23-concurrency-parallel-threads/ 24-04-synchronize.md)
- [Oarallel](./23-concurrency-parallel-threads/ 24-05-parallel.md)

### 24: :[Unsafe Raw Pointer](24-unsafety-rawpointer/ 24-00-preface.md) <small> (JohnSmithX)
- [Unsafe](24-unsafety-rawpointer/24-01-unsafety.md)
- [Raw Pointers](24-unsafety-rawpointer/24-02-raw-pointer.md)

### 25: :[FFI](25-ffi/25-00-preface.md) <small> (42)
- [Rust Calling ffi Function](25-ffi/ 25-01-calling-ffi-functions.md)
- [The Rust Compiled into Libraries](25-ffi/ 25-02-compiling-rust-to-lib.md)

### 26: :[Operator Overloading](26-operator-overload/ 26-01-operator.md) <small> (wayslog)
### 27: :[Properties and Compiling Parameters](27-attr-and-compiler-args/27-00-preface.md) <small> (elton)
- [Properties](./27-attr-and-compiler-args/ 27-01-attributes.md)
- [Compiler Arguments](27-attr-and-compiler-args/ 27-02-rustc-options.md)

### 28: :[Cargo Parameter Configurations](28-cargo-detailed-cfg/ 28-01-cargo-detailed-cfg.md) <small> (fuyingfuying)
### 29: :[Test and Evaluation](29-testing / 29-00-preface.md) <small> (daogangtang)
- [Test (testing)](29-testing/29-01-threearchtest.md)
- [evaluation (benchmark)](29-testing/29-02-bench.md)

### 30: :[Code Style](30-coding-style/30-01-style.md) <small> (tiansiyuan)
### 31: :[Any](31-any/31-01-any.md) <small> (wayslog)
### 32: :[Security (safety)](32-safety/32-01-safety.md) <small> (daogangtang)
### 33: :[Common Data Structures to Achieve](33-data-struct / 33-00-preface.md) <small> (Naupio)
- [Stack Structure](33-data-struct / 33-01-stack.md)
- [Queue](33-data-struct / 33-02-queue.md)
- [PQ](33-data-struct / 33-03-priority_queue.md)
- [Binary](33-data-struct / 33-04-binary_tree.md)
- [List](33-data-struct / 33-05-linked_list.md)
- [Fig Structure](33-data-struct / 33-06-graph.md)

### 34: :[Standard Library Presentation](34-std/ 34-00-overview.md) <small> (wayslog)
- [System Commands: Call grep](34-std/34-01-process.md)
- [Directory Operations: simple grep](34-std / 34-02-fs-and-path.md)
- [Network Module: W Echo](34-std/34-03-net.md)

### 35: :[Actual Articles](35-action/35-00-preface.md) <small> (wangyu190810)
- [Combat: JSON Processing](35-action/json_data / readme.md)
- [Combat: Web Application Development Entry](35-action / mysite / readme.md)
- [Combat: Use Postgresql Database](35-action/db/readme.md)

## Copyright Regulations & License Information
<small> (Where applicable)

This book uses the `CC BY-SA 3.0` agreement, please indicate the address.


## ChangeLog

1. March 31, 2016, to complete the first draft. V1.0 release version

1. 2016 年3月31日，初稿完成。发布 v1.0 版。

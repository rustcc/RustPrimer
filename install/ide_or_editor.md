# IDE或编辑器支持

Rust目前的IDE/编辑器支持仍然不是特别完善，但已经基本可用。

Rust官方的编辑器支持从[Racer](https://github.com/racer-rust/racer)到RLS(https://github.com/rust-lang/rls)，再到现在的[Rust Analyzer](https://github.com/rust-analyzer/rust-analyzer)，都采取核心逻辑与编辑器分离的设计。即单独实现“自动补全”、“代码跳转”等功能，进而适配多个编辑器，以提供更广泛的编辑器支持。

另外，著名开发工具研发商Jetbrain也很早就以插件形式对Rust的开发提供了支持。

## Intellij Rust

Jetbrain暂时并没有为其出单独的IDE，但提供了官方开发的[Intellij Rust](https://github.com/intellij-rust/intellij-rust)插件。

Intellij Rust已经发布到Jetbran的[插件市场](https://plugins.jetbrains.com/plugin/8182-rust)，其安装仅需要打开 `Settings > Plugins > Marketplace`后搜索 `Rust` 并安装对应插件即可。

Intellij Rust支持Jetbrain 旗下主流IDE。但在不同的IDE上，其实际表现有细微差别。目前（2020.3）支持最好的是CLion，且相对其他平台，Clion平台上的Intellij Rust也有细微的性能优势。对于不想付费的用户，可以通过Intellij IDEA Ultimate安装使用。

PS: 目前（2020.3）Intellij Rust提供宏展开功能，对使用宏生成的代码提供一定支持。其中包含一个实验性的宏展开引擎，默认不开启，手动开启后可以获得更好的支持。

## Rust Analyzer

Rust Analyzer是由原Intellij Rust开发组成员主要参与开发的一个Rust编译器前端，是RLS-2.0计划的一部分，意在为Rust提供更好的IDE支持。

目前(2020.3)，Rust Analyzer仍然处于实验阶段，但被普遍认为相比RLS 1.0有着更好的稳定性和更丰富的功能。Rust Analyzer支持多个编辑器，其中支持最好的是VScode。

### VScode

对于VScode用户，在[插件市场](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)安装名为rust-analyzer的插件后，第一次打开一个包含Cargo.toml的文件夹，将会自动下载合适版本的LSP Server实现并启用。

### 其他编辑器

由于Rust-Analyzer仍然处于实验性阶段，安装方式可能发生变化，其他编辑器的配置方法请参考[Rust Analyzer官方文档](https://rust-analyzer.github.io/manual.html#installation)，并推荐关注[Rust-Analyzer官网的Changelog](https://rust-analyzer.github.io/thisweek)页面，在页面下方提供了RSS形式的订阅支持。

## 二者的比较

目前（2020.3），Intellij Rust有着更完善的功能，但也被相当一部分开发者反馈存在严重的卡顿问题。Rust Analyzer的功能相对较弱，但在快速迭代阶段，且暂时没有太多关于性能上的问题报告。

二者对于宏的支持都有限，但Intellij Rust表现的相对较好，对于大部分`macro_rules!`形式的宏生成的代码都可以有比较好的支持，其对`lazy_static`等广泛使用库也有额外支持。

可以考虑搭配AI只能补全工具[tabnine](https://tabnine.com/)使用，`tabnine`由于使用Rust开发，作者声称对Rust代码提供永久免费的全功能支持。但`tabnine`被一些人认为会占用太多的内存（一般1到3G）。
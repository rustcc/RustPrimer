筒子们好，我们又见面了。之前第5章，我们一起探讨了cargo的一些常用的基本技能。通过第5章的学习，大家基本能解决日常项目开发中遇到的大多数问题。但实际上，cargo提供给我们所使用的功能不仅限于此。我只想说一个字：cargo很好很强大，而且远比你想象的强大。
本章将深入探讨cargo的一些细节问题，这包括以下几个方面：
- 基于语义化版本的项目版本声明与管理
- cargo的toml描述文件配置字段详细参考

# 基于语义化版本的项目版本声明与管理
我们在使用toml描述文件对项目进行配置时，经常会遇到项目版本声明及管理的问题，比如：

```toml
[package]
name = "libevent_sys"
version = "0.1.0"

[dependencies]
libc = "0.2"

```
这里package段落中的version字段的值，以及dependencies段落中的libc字段的值，这些值的写法，都涉及到语义化版本控制的问题。语义化版本控制是用一组简单的规则及条件来约束版本号的配置和增长。这些规则是根据（但不局限于）已经被各种封闭、开放源码软件所广泛使用的惯例所设计。简单来说，语义化版本控制遵循下面这些规则：

- 版本格式：主版本号.次版本号.修订号，版本号递增规则如下：

1. 主版本号：当你做了不兼容的 API 修改，
2. 次版本号：当你做了向下兼容的功能性新增，
3. 修订号：当你做了向下兼容的问题修正。

- 先行版本号及版本编译信息可以加到“主版本号.次版本号.修订号”的后面，作为延伸。

关于语义化版本控制的具体细节问题，大家可以参考[这里](http://semver.org/lang/zh-CN/)，我不再赘述。

# cargo的toml描述文件配置字段详细参考
## [package]段落
啥也不多说了，直接上例子，大家注意我在例子中的中文解释，个人觉得这样比较一目了然：
```toml
[package]
 # 软件包名称，如果需要在别的地方引用此软件包，请用此名称。
name = "hello_world"

# 当前版本号，这里遵循semver标准，也就是语义化版本控制标准。
version = "0.1.0"    # the current version, obeying semver

# 软件所有作者列表
authors = ["you@example.com"]

# 非常有用的一个字段，如果要自定义自己的构建工作流，
# 尤其是要调用外部工具来构建其他本地语言（C、C++、D等）开发的软件包时。
# 这时，自定义的构建流程可以使用rust语言，写在"build.rs"文件中。
build = "build.rs"

# 显示声明软件包文件夹内哪些文件被排除在项目的构建流程之外，
# 哪些文件包含在项目的构建流程中
exclude = ["build/**/*.o", "doc/**/*.html"]
include = ["src/**/*", "Cargo.toml"]

# 当软件包在向公共仓库发布时出现错误时，使能此字段可以阻止此错误。
publish = false

# 关于软件包的一个简短介绍。
description = "..."

# 下面这些字段标明了软件包仓库的更多信息
documentation = "..."
homepage = "..."
repository = "..."

# 顾名思义，此字段指向的文件就是传说中的ReadMe，
# 并且，此文件的内容最终会保存在注册表数据库中。
readme = "..."

# 用于分类和检索的关键词。
keywords = ["...", "..."]

# 软件包的许可证，必须是cargo仓库已列出的已知的标准许可证。
license = "..."

# 软件包的非标许可证书对应的文件路径。
license-file = "..."
```

## 依赖的详细配置
最直接的方式在之前第五章探讨过，这里不在赘述，例如这样：
```toml
[dependencies]
hammer = "0.5.0"
color = "> 0.6.0, < 0.8.0"
```

与平台相关的依赖定义格式不变，不同的是需要定义在[target]字段下。例如：

```toml
# 注意，此处的cfg可以使用not、any、all等操作符任意组合键值对。
# 并且此用法仅支持cargo 0.9.0（rust 1.8.0）以上版本。
# 如果是windows平台，则需要此依赖。
[target.'cfg(windows)'.dependencies]
winhttp = "0.4.0"

[target.'cfg(unix)'.dependencies]
openssl = "1.0.1"

#如果是32位平台，则需要此依赖。
[target.'cfg(target_pointer_width = "32")'.dependencies]
native = { path = "native/i686" }

[target.'cfg(target_pointer_width = "64")'.dependencies]
native = { path = "native/i686" }

# 另一种写法就是列出平台的全称描述
[target.x86_64-pc-windows-gnu.dependencies]
winhttp = "0.4.0"
[target.i686-unknown-linux-gnu.dependencies]
openssl = "1.0.1"

# 如果使用自定义平台，请将自定义平台文件的完整路径用双引号包含
[target."x86_64/windows.json".dependencies]
winhttp = "0.4.0"
[target."i686/linux.json".dependencies]
openssl = "1.0.1"
native = { path = "native/i686" }
openssl = "1.0.1"
native = { path = "native/x86_64" }

# [dev-dependencies]段落的格式等同于[dependencies]段落，
# 不同之处在于，[dependencies]段落声明的依赖用于构建软件包，
# 而[dev-dependencies]段落声明的依赖仅用于构建测试和性能评估。
# 此外，[dev-dependencies]段落声明的依赖不会传递给其他依赖本软件包的项目
[dev-dependencies]
iron = "0.2"

```

## 自定义编译器调用方式模板详细参数
cargo内置五种编译器调用模板，分别为dev、release、test、bench、doc，分别用于定义不同类型生成目标时的编译器参数，如果我们自己想改变这些编译模板，可以自己定义相应字段的值，例如（注意：下述例子中列出的值均为此模板字段对应的系统默认值）：

```toml
# 开发模板, 对应`cargo build`命令
[profile.dev]
opt-level = 0  # 控制编译器的 --opt-level 参数，也就是优化参数
debug = true   # 控制编译器是否开启 `-g` 参数
rpath = false  # 控制编译器的 `-C rpath` 参数
lto = false    # 控制`-C lto` 参数，此参数影响可执行文件和静态库的生成，
debug-assertions = true  # 控制调试断言是否开启
codegen-units = 1 # 控制编译器的 `-C codegen-units` 参数。注意，当`lto = true`时，此字段值被忽略

# 发布模板, 对应`cargo build --release`命令
[profile.release]
opt-level = 3
debug = false
rpath = false
lto = false
debug-assertions = false
codegen-units = 1

# 测试模板，对应`cargo test`命令
[profile.test]
opt-level = 0
debug = true
rpath = false
lto = false
debug-assertions = true
codegen-units = 1

# 性能评估模板，对应`cargo bench`命令
[profile.bench]
opt-level = 3
debug = false
rpath = false
lto = false
debug-assertions = false
codegen-units = 1

# 文档模板，对应`cargo doc`命令
[profile.doc]
opt-level = 0
debug = true
rpath = false
lto = false
debug-assertions = true
codegen-units = 1

```
需要注意的是，当调用编译器时，只有位于调用最顶层的软件包的模板文件有效，其他的子软件包或者依赖软件包的模板定义将被顶层软件包的模板覆盖。

## [features]段落
[features]段落中的字段被用于条件编译选项或者是可选依赖。例如：

```toml
[package]
name = "awesome"

[features]
# 此字段设置了可选依赖的默认选择列表，
# 注意这里的"session"并非一个软件包名称，
# 而是另一个featrue字段session
default = ["jquery", "uglifier", "session"]

# 类似这样的值为空的feature一般用于条件编译，
# 类似于`#[cfg(feature = "go-faster")]`。
go-faster = []

# 此feature依赖于bcrypt软件包，
# 这样封装的好处是未来可以对secure-password此feature增加可选项目。
secure-password = ["bcrypt"]

# 此处的session字段导入了cookie软件包中的feature段落中的session字段
session = ["cookie/session"]

[dependencies]
# 必要的依赖
cookie = "1.2.0"
oauth = "1.1.0"
route-recognizer = "=2.1.0"

# 可选依赖
jquery = { version = "1.0.2", optional = true }
uglifier = { version = "1.5.3", optional = true }
bcrypt = { version = "*", optional = true }
civet = { version = "*", optional = true }
```

如果其他软件包要依赖使用上述awesome软件包，可以在其描述文件中这样写：
```toml
[dependencies.awesome]
version = "1.3.5"
default-features = false # 禁用awesome 的默认features
features = ["secure-password", "civet"] # 使用此处列举的各项features
```
使用features时需要遵循以下规则：
- feature名称在本描述文件中不能与出现的软件包名称冲突
- 除了default feature，其他所有的features均是可选的
- features不能相互循环包含
- 开发依赖包不能包含在内
- features组只能依赖于可选软件包

features的一个重要用途就是，当开发者需要对软件包进行最终的发布时，在进行构建时可以声明暴露给终端用户的features，这可以通过下述命令实现：

```
$ cargo build --release --features "shumway pdf"
```
## 关于测试
当运行cargo test命令时，cargo将会按做以下事情：
- 编译并运行软件包源代码中被#[cfg(test)] 所标志的单元测试
- 编译并运行文档测试
- 编译并运行集成测试
- 编译examples

## 配置构建目标
所有的诸如[[bin]], [lib], [[bench]], [[test]]以及 [[example]]等字段，均提供了类似的配置，以说明构建目标应该怎样被构建。例如（下述例子中[lib]段落中各字段值均为默认值）：

```toml
[lib]
# 库名称，默认与项目名称相同
name = "foo"

# 此选项仅用于[lib]`段落，其决定构建目标的构建方式，
# 可以取`dylib, `rlib, `staticlib 三种值之一，表示生成动态库、r库或者静态库。
crate-type = ["dylib"]

# path字段声明了此构建目标相对于cargo.toml文件的相对路径
path = "src/lib.rs"

# 单元测试开关选项
test = true

# 文档测试开关选项
doctest = true

# 性能评估开关选项
bench = true

# 文档生成开关选项
doc = true

# 是否构建为编译器插件的开关选项
plugin = false

# 如果设置为false，`cargo test`将会忽略传递给rustc的--test参数。
harness = true
```
##

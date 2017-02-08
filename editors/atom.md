# Atom
本文是rust的Atom编辑器配置。
横向对比一下，不得不说，Atom无论在易用性还是界面上都比前辈们要好的很多，对于Rust的配置，也是基本上可以做到开箱即用。
虽然本文独占一小节，但是其实能写的东西也就了了。

- [自行配置](#自行配置)
- [使用tokamak](#tokamak)

## 自行配置

## 准备工作

首先，你需要一个可执行的rustc编译器，一个cargo程序，一个已经编译好的racer程序和一份已经解压好的rust源码。
我们假定你已经将这三个程序安装完毕，并且能够自由的从命令行里调用他们。

另外，本文不讲解如何安装Atom，需要新安装的同学请自行前往[项目主页](https://github.com/atom/atom)安装。

ps:无论是windows用户还是*nix用户都需要将以上三个程序加入你的PATH(Windows下叫Path)环境变量里。

## 需要安装的插件包

打开Atom，按Ctrl+Shift+p，搜索preference，打开Atom的配置中心，选择install选项卡。

依次安装`rust-api-docs-helper`/`racer`/`language-rust`/`linter-rust`/`linter`。

这里要单独说的一个就是linter，这是一个基础的lint组件包，atom的很多以linter为前缀的包都会依赖这个包，但是Atom并不会为我们自动的安装，因此需要我们自己去安装。

## 一点配置

以上，我们安装好了几个组件包，但是不要着急去打开一个Rust文件。你可能还需要一点点的配置。这里，我们在配置中心里打开`Packages`选项卡，在`Installed Packages`里搜索racer，并点击其`Setting`。

这里需要将racer的可执行文件的绝对路径填入`Path to the Racer executable`里。同时，我们还需要将rust源码文件夹下的src目录加入到`Path to the Rust source code directory`里。

## 完成安装

好了，就是这么简单。你现在可以打开任意一个rust文件就会发现源码高亮已经默认打开了，编辑一下，racer也能自动补全，*如果不能*，尝试一下用`F3`键来显式地呼出racer的补全。

## tokamak

[tokamak](https://github.com/vertexclique/tokamak) 是一个使 atom 摇身一变为 rust IDE 的 atom 插件. 安装后 atom 即具有语法高亮, 代码补全与 Lint 等功能, 而且还有个不错的界面, 看起来确实像个 IDE. 你可以在 atom 中搜索 tokamak 并安装它.

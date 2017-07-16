# VS Code 安装配置

[VS Code](https://code.visualstudio.com/) 是微软出的一款开源代码编辑器，秉承了微软在IDE领域的一惯优秀基因，是一款潜力相当大的编辑器/IDE。

VScode 目前也对 Rust 也有良好的支持。



## 下载 VScode

请打开官网 https://code.visualstudio.com/ 下载编辑器。

## 依赖

如本章第一节所述，准备好 `racer`，`rust 源代码`，`rustfmt`，`rls` 这四样东西，并且配置好相应的环境变量，此不赘述。

## 安装 Rust 扩展 Rust

1. 打开 VScode 编辑器；
2. 按 Ctrl + p 打开命令面板；
3. 在编辑器中上部浮现出的输入框中，输入 `ext install vscode-rust`，会自动搜索可用的插件，搜索出来后，点击进行安装；
4. 使用`VScode`打开任意一个`.rs`文件，插件首次启动会自动引导用户完成配置。

注:推荐使用RLS模式，即使用[Rust Langular Server](https://github.com/rust-lang-nursery/rls)提供各项功能支持

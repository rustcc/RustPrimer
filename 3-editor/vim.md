# VIM/GVIM安装配置

本文是VIM的Rust支持配置，在阅读本文之前，我们假定你已经拥有了一个可执行的rustc程序，并编译好了racer。

## 1.1 使用vundel

vundel是vim的一个插件管理工具，基本上算是本类当中最为易用的了。
首先我们需要安装它

```bash
mkdir -p  ~/.vim/bundle/
git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim
```
## 1.2 启用rust支持
首先，你需要下载rust-lang的源码文件，并将其解压到一个路径下。

然后，在你的`~/.vimrc`文件里添加如下内容

```vim
set nocompatible              " be iMproved, required
filetype off                  " required
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

Plugin 'VundleVim/Vundle.vim'
Plugin 'racer-rust/vim-racer'
Plugin 'rust-lang/rust.vim'

call vundle#end()            " required

let g:rustfmt_autosave = 1
set hidden
"" 需要注意的是下面这两行配置;
"" 这一行指的是标注出来你编译的racer的路径
let g:racer_cmd = "<path-to-racer>/target/release/racer"
"" 这一行是将你下载的rust源码路径填进去，这里注意要填的是源码路径下的src，而不是直接填到你解压的路径下。
"" 例如你将源码解压到了`/home/wayslog/tools/rust-1.5/`路径下，
"" 那么你下面的值应该为`/home/wayslog/tools/rust-1.5/src`.
let $RUST_SRC_PATH="<path-to-rust-srcdir>/src/"

```
然后我们在终端中执行
```bash
vim +PluginInstall +qall
```
就能正常安装并且使用了。

包括如下功能：

  1. 基本的c-x c-o补全
  2. 语法着色
  3. gd跳转到定义（不好用）

总体来看支持度并不高。
## 2. 额外的
Q1. 为什么没有自动补全

A1. 事实上我是一直在用YCM来进行补全的，但是Racer并没有被YCM支持，而且似乎有人去YCM下提ISSUE，但是被作者拒绝了，于是就呵呵了。

Q2. 颜色好搓

A2. 我推荐一个配色，也是我自己用的 [molokai](https://github.com/tomasr/molokai)

更详细内容可以参见我的[vimrc配置](https://github.com/wayslog/dotfiles/blob/master/_vimrc)，当然，我这个用的是比较老的版本的vundle，仅供参考。

Have a nice Rust !

# vim/GVim安装配置

本节介绍vim的Rust支持配置，在阅读本节之前，我们假定你已经拥有了一个可执行的rustc程序，并编译好了racer。

## 我的vim截图

应邀而加

![此处应该有截图](../images/editor-vim-wayslog.png)

## 使用vundle

vundle是vim的一个插件管理工具，基本上算是本类当中最为易用的了。
首先我们需要安装它

### linux or OS X

```bash
mkdir -p  ~/.vim/bundle/
git clone https://github.com/VundleVim/Vundle.vim.git ~/.vim/bundle/Vundle.vim
```

### windows

1. 首先找到你的gvim的安装路径，然后在路径下找到vimfiles文件夹
2. 在这个文件夹中将vundle库克隆到vimfiles/bundle/目录下的Vundle.vim文件夹中

## 启用rust支持

### 下载源码

首先，你需要下载rust-lang的源码文件，并将其解压到一个路径下。
这个源码文件我们可以从[rust官网](https://www.rust-lang.org/downloads.html)下载到，请下载你对应平台的文件。
然后将其解压到一个目录下，并找到其源码文件中的`src`目录。
比如我们解压源码包到`C:\\rust-source\`，那么我们需要的路径就是`C:\\rust-source\src`，记好这个路径，我们将在下一步用到它。

### 修改vim配置

首先找到你的vimrc配置文件，然后在其中添加如下配置

```vim
set nocompatible
filetype off
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

Plugin 'VundleVim/Vundle.vim'
Plugin 'racer-rust/vim-racer'
Plugin 'rust-lang/rust.vim'

call vundle#end()

filetype on
```

然后为了让配置生效，我们重启我们的(g)vim，然后在vim里执行如下命令

```
:PluginInstall
```

这里vundle会自动的去仓库里拉取我们需要的文件，这里主要是vim-racer和rust.vim两个库。

### 更多的配置

为了让我们的vim能正常的使用，我们还需要在vimrc配置文件里加入一系列配置，

```vim
"" 开启rust的自动reformat的功能
let g:rustfmt_autosave = 1

"" 手动补全和定义跳转
set hidden
"" 这一行指的是你编译出来的racer所在的路径
let g:racer_cmd = "<path-to-racer>/target/release/racer"
"" 这里填写的就是我们在1.2.1中让你记住的目录
let $RUST_SRC_PATH="<path-to-rust-srcdir>/src/"
```

#### 使用 YouCompleteMe

YouCompleteMe 是 vim 下的智能补全插件, 支持 C-family, Python, Rust 等的语法补全, 整合了多种插件, 功能强大. Linux 各发行版的官方源里基本都有软件包, 可直接安装. 如果有需要进行编译安装的话, 可参考[官方教程](https://github.com/Valloric/YouCompleteMe#installation)

让 YCM 支持 Rust 需要在安装 YCM 过程中执行 ./install.py 时加上 --racer-completer, 并在 .vimrc 中添加如下设置

```
let g:ycm_rust_src_path="<path-to-rust-srcdir>/src/"
"" 一些方便的快捷键
""" 在 Normal 模式下, 敲 <leader>jd 跳转到定义或声明(支持跨文件)
nnoremap <leader>jd :YcmCompleter GoToDefinitionElseDeclaration<CR>
""" 在 Insert 模式下, 敲 <leader>; 补全
inoremap <leader>; <C-x><C-o>
```

## 总结

经过不多的配置，我们得到了如下功能：

  1. 基本的c-x c-o补全 (使用 YCM 后, 能做到自动补全)
  2. 语法着色
  3. gd跳转到定义

总体来看支持度并不高。

![此处应该有第二张截图](../images/editor-vim-welldone.png)

### 额外的
Q1. 颜色好挫

A1. 我推荐一个配色，也是我自己用的 [molokai](https://github.com/tomasr/molokai)

更详细内容可以参见我的[vimrc配置](https://github.com/wayslog/dotfiles/blob/master/_vimrc)，当然，我这个用的是比较老的版本的vundle，仅供参考。

Have a nice Rust !

# VIM/GVIM安装配置

本文是VIM的Rust支持配置，在阅读本文之前，我们假定你已经拥有了一个可执行的rustc程序，并编译好了racer。

## 0 我的vim截图

应邀而加

![此处应该有截图](https://github.com/wayslog/RustPrimer/blob/editor/image/03-editor-vim-wayslog.png)

## 1.1 使用vundle

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

## 1.2 启用rust支持

### 1.2.1 下载源码

首先，你需要下载rust-lang的源码文件，并将其解压到一个路径下。
这个源码文件我们可以从[rust官网](https://www.rust-lang.org/downloads.html)下载到，请下载你对应平台的文件。
然后将其解压到一个目录下，并找到其源码文件中的`src`目录。
比如我们解压源码包到`C:\\rust-source\`，那么我们需要的路径就是`C:\\rust-source\src`，记好这个路径，我们将在下一步用到它。

### 1.2.2 修改vim配置

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

### 1.2.3 更多的配置

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

## 1.3.1 总结

经过不多的配置，我们的到了包括如下功能：

  1. 基本的c-x c-o补全
  2. 语法着色
  3. gd跳转到定义（不好用）

总体来看支持度并不高。

![此处应该有第二张截图](https://github.com/wayslog/RustPrimer/blob/editor/image/03-editor-vim-welldone.png)

### 1.3.1 额外的
Q1. 为什么没有自动补全

A1. 事实上我是一直在用YCM来进行补全的，但是racer并没有被YCM支持，而且似乎有人去YCM下提ISSUE，但是被作者拒绝了，于是就呵呵了。

Q2. 颜色好搓

A2. 我推荐一个配色，也是我自己用的 [molokai](https://github.com/tomasr/molokai)

更详细内容可以参见我的[vimrc配置](https://github.com/wayslog/dotfiles/blob/master/_vimrc)，当然，我这个用的是比较老的版本的vundle，仅供参考。

Have a nice Rust !

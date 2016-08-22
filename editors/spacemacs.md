# Spacemacs

spacemacs，是一个给vimer的Emacs。
## 简介
spacemacs是一个专门给那些习惯vim的操作，同时又向往emacs的扩展能力的人。它非常适合我这种折腾过vim，配置过emacs的人，但同时也欢迎任何没有基础的新人使用。简单来说，它是一个开箱即用的Emacs！这对一个比很多人年龄都大的软件来说是一件极其不容易的事情。

## 安装
由于笔者自己在linux平台，并没有windows平台的经验，所以在这里便不献丑了，期待各位补充。另外，windows平台真的需要么，斜眼瞅向了Visual Studio。

### Emacs安装

在*nix系统中，都不一定会默认安装了Emacs，就算安装了，也不一定是最新的版本。在这里，我强烈建议各位卸载掉系统自带的Emacs，因为你不知道系统给你安装的是个什么奇怪的存在，最遭心的，我碰见过只提供阉割版Emacs的linux发行版。

建议各位自己去emacs项目主页下载Emacs-24.5（本书写作时的最新版）极其以上版本，然后下载下来源码。至于Emacs的安装也非常简单，linux平台老三步。

```bash
./configure
make
sudo make install
```

什么？你没有make？没有GCC？缺少依赖？
请安装它们……

### Spacemacs安装

前面说了,Spacemacs就是个Emacs的配置文件库，因此我们可以通过非常简单的方式安装它：

```bash
git clone https://github.com/syl20bnr/spacemacs ~/.emacs.d
mv ~/.emacs ~/_emacs.backup
cd ~/.emacs.d
echo $(git describe --tags $(git rev-list --tags --max-count=1)) | xargs git checkout
```

其中，后三行是笔者加的，这里必须要吐槽一下的是，Spacemacs的master分支实际上是极其落后而且有错误的！其目前的release都是从develop分支上打的tag。

因此，一！定！不！要！用！主！分！支！

最后，之所以要加最后一行，这是笔者安装的时候的release的一个小bug，没有这个文件的话,emacs并不会顺利的完成初始化。

好了，配置文件我们已经搞定了，接下来，启动你的emacs，spacemacs会自动的去网上下载你需要的插件安装包。另外，能自备梯子的最好，因为你要下的东西不大，但是这个网络确实比较捉急。

### 前期准备

为了让Spacemacs支持Rust，我们还需要一点小小的配置。首先，请参照[前期准备](../editors/before.md)，安装好你的racer。

在这里，强烈建议将racer的环境变量加入到系统变量中(通常他们在`/etc/profile/`里进行配置)并重新启动系统，因为真的有很多人直接点击emacs的图标启动它的，这样做很可能导致emacs并不继承自己的环境变量，这是很令人无奈的。

## 完成配置

### 修改标准的Spacemacs配置。

Spacemacs文档中提供了一份标准的spacemacs[配置文件](https://github.com/syl20bnr/spacemacs/blob/master/core/templates/.spacemacs.template)，你可以将其加入到你自己的`~/.spacemacs`文件中。

这里，我们需要修改的是其关于自定义插件的部分：

```lisp
(defun dotspacemacs/layers ()
  "Configuration Layers declaration.
You should not put any user code in this function besides modifying the variable
values."
  (setq-default
   ;; Base distribution to use. This is a layer contained in the directory
   ;; `+distribution'. For now available distributions are `spacemacs-base'
   ;; or `spacemacs'. (default 'spacemacs)
   dotspacemacs-distribution 'spacemacs
   ;; List of additional paths where to look for configuration layers.
   ;; Paths must have a trailing slash (i.e. `~/.mycontribs/')
   dotspacemacs-configuration-layer-path '()
   ;; List of configuration layers to load. If it is the symbol `all' instead
   ;; of a list then all discovered layers will be installed.
   dotspacemacs-configuration-layers
   '(
     ;; ----------------------------------------------------------------
     ;; Example of useful layers you may want to use right away.
     ;; Uncomment some layer names and press <SPC f e R> (Vim style) or
     ;; <M-m f e R> (Emacs style) to install them.
     ;; ----------------------------------------------------------------
     auto-completion
     better-defaults
     git
     spell-checking
     syntax-checking
     version-control
     rust
     )
   ;; List of additional packages that will be installed without being
   ;; wrapped in a layer. If you need some configuration for these
   ;; packages then consider to create a layer, you can also put the
   ;; configuration in `dotspacemacs/config'.
   dotspacemacs-additional-packages '()
   ;; A list of packages and/or extensions that will not be install and loaded.
   dotspacemacs-excluded-packages '()
   ;; If non-nil spacemacs will delete any orphan packages, i.e. packages that
   ;; are declared in a layer which is not a member of
   ;; the list `dotspacemacs-configuration-layers'. (default t)
   dotspacemacs-delete-orphan-packages t))

;; ...
;; 以下配置文件内容省略
;; ...
```

注意`dotspacemacs-configuration-layers`的配置和标准配置文件的不同。

将配置文件保存，然后重启你的emacs，当然，我们也可以按`SPC f e R`来完成重载配置文件的目的，然后你会发现emacs会开始下一轮下载，稍等其完成。

在上一步中，我们已经完成了对Racer的环境变量的配置，所以，现在你的Spacemacs已经配置完成了！这种简便的配置形式，几乎能和Atom抗衡了。

### 按键绑定
如下，spacemacs默认提供了几种按键绑定，但是，笔者并不觉得这些很好用，还是更喜欢用命令行。

| Key Binding | Description                       |
|-------------|-----------------------------------|
| ~SPC m c c~ | compile project with Cargo        |
| ~SPC m c t~ | run tests with Cargo              |
| ~SPC m c d~ | generate documentation with Cargo |
| ~SPC m c x~ | execute the project with Cargo    |

## 尝试

现在开始，我们可以打开一个Cargo项目，并且去使用它了。你会惊讶的发现racer/flycheck/company这三个插件配合在一起的时候是那么的和谐简单。

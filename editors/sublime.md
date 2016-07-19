# Sublime

Sublime Text是一款非常有名的文本编辑器，其本身也具备强大的插件机制。通过配置各种插件可以在使用Sublime Text编辑rust代码时获得更加良好的支持。

本文主要展示在已经预装rust的Windows环境下的安装，如果您还没有安装rust，请先参照本书的[安装章节](../install/install_rust_on_windows.md)安装rust。

## 安装

### Sublime Text3安装

请在 [Sublime Text3官网](http://www.sublimetext.com/3)上选择适合当前机器版本的Sublime Text版本进行下载和安装。

### rust的安装

请在rust官网的[下载页面](https://www.rust-lang.org/downloads.html)下载rust的源代码压缩包并在本地解压缩安装，在稍后的配置环节我们将会用到这个路径。如果国内下载速度过慢，可以考虑使用中科大的[镜像](http://mirrors.ustc.edu.cn/)下载rust源码包。

### 下载Rust并编译代码提示插件racer

具体安装和编译内容请查看本章第一节的[安装准备](../editors/before.md)，请牢记编译后的racer.exe文件路径，在稍后的配置环节中我们将用到它。

## 配置

### Sublime Text3相关插件安装

#### 安装Package Control

Sublime Text3在安装各种插件前需要先安装Package Control，如果您的编辑器已安装Package Control请跳过本段直接安装rust相关插件。

您可以查看[Package Control官网](https://packagecontrol.io/installation)学习如何安装。
也可以直接在编辑器中使用 `ctrl+~` 快捷键启动控制台，粘贴以下代码并回车进行安装。

```shell

import urllib.request,os,hashlib; h = '2915d1851351e5ee549c20394736b442' + '8bc59f460fa1548d1514676163dafc88'; pf = 'Package Control.sublime-package'; ipp = sublime.installed_packages_path(); urllib.request.install_opener( urllib.request.build_opener( urllib.request.ProxyHandler()) ); by = urllib.request.urlopen( 'http://packagecontrol.io/' + pf.replace(' ', '%20')).read(); dh = hashlib.sha256(by).hexdigest(); print('Error validating download (got %s instead of %s), please try manual install' % (dh, h)) if dh != h else open(os.path.join( ipp, pf), 'wb' ).write(by)

```

#### rust相关插件

在编辑器下使用快捷键 `ctrl+shift+p` 启动命令行工具，输入Install Package按回车进入插件安装，选择或输入插件名称并回车即可完成插件的安装。

使用上述方式安装Rust插件\(rust语法高亮\)、RustAutoComplete\(rust代码提示和自动补全插件\)。

此时安装尚未完成，我们需要将本地的 racer.exe配置进RustAutoComplete插件中。打开编辑器顶端的Preferences选项卡，依次 Preferences->Package Settings->RustAutoComplete->Settings-User 来打开 RustAutoComplete 的配置文件，在文件中配置以下信息并保存。

```shell
{
  "racer": "E:/soft/racer-master/target/release/racer.exe",
  "search_paths": [    "E:/soft/rustc-1.7.0/src"   ]
}
```

其中racer是编译后的racer.exe程序的绝对路径。search_paths是rust源码文件下src目录的绝对路径。

编辑器重启后插件即可生效。

## 快速编译

Sublime本身支持多种编译系统，在Tools选项卡下的Build System中选择Rust或者Cargo作为编译系统，选中后使用快捷键 `ctrl+B` 即可对代码进行快速编译。


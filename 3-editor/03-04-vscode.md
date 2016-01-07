# VS Code 安装配置

[VS Code](https://code.visualstudio.com/) 是微软出的一款开源代码编辑器，秉承了微软在IDE领域的一惯优秀基因，是一款潜力相当大的编辑器/IDE。

VScode 目前也对 Rust 也有良好的支持。



## 下载 VScode

请打开官网 https://code.visualstudio.com/ 下载编辑器。

## 依赖

如本章第一节所述，准备好 `racer`，`rust 源代码`，`rustfmt` 这三样东西，并且配置好相应的环境变量，此不赘述。

## 安装 Rust 扩展 RustyCode

1. 打开 VScode 编辑器；
2. 按 Ctrl + p 打开命令面板；
3. 在编辑器中上部浮现出的输入框中，输入 `ext install Rusty`，会自动搜索可用的插件，搜索出来后，点击进行安装；
4. 完成。

### 手动下载扩展

由于国内网络访问外网上经常坑爹，如果您遇到在VScode里面，下载不下来的情况，可以用如下方式解决（以 Linux 举例，其它类同）：

1. 打开终端，回到用户根目录。`cd ~`；
2. `cd .vscode`;
3. 检查当前目录下有没有 `extensions` 目录，如果没有，`mkdir extensions` 建一个。如果有，直接跳过本步骤；
4. `cd extensions`；
5. `git clone https://github.com/saviorisdead/RustyCode.git`;
6. 等下载完成后，（重新）打开 VScode；
7. Ctrl + p 打开命令面板，输入 `ext ` 查看是否有提醒 `Rusty Code`，如果提示，表示安装成功；
8. 大功告成。


## 自定义配置

你可以自定义配置一些东西，打开 `vi .vscode/settings.json`，可以修改如下配置：

```
{
    "rust.racerPath": null, // Specifies path to Racer binary if it's not in PATH
    "rust.rustLangSrcPath": null, // Specifies path to /src directory of local copy of Rust sources
    "rust.rustfmtPath": null, // Specifies path to Rustfmt binary if it's not in PATH
    "rust.cargoPath": null, // Specifies path to Cargo binary if it's not in PATH
    "rust.formatOnSave": false, // Turn on/off autoformatting file on save (EXPERIMENTAL)
}
```




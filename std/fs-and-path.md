# 目录操作:简单grep

上一节我们实现了通过`Command`调用subprocess。这一节，我们将通过自己的代码去实现一个简单的grep。当然了，这种基础的工具你是能找到源码的，而我们的实现也并不像真正的grep那样注重效率，本节的主要作用就在于演示标准库API的使用。

首先，我们需要对当前目录进行递归，遍历，每当查找到文件的时候，我们回调一个函数。

于是，我们就有了这么个函数：

```rust
use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;

fn visit_dirs(dir: &Path, pattern: &String, cb: &Fn(&DirEntry, &String)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), pattern, cb));
            } else {
                cb(&entry, pattern);
            }
        }
    }else{
        let entry = try!(try!(fs::read_dir(dir)).next().unwrap());
        cb(&entry, pattern);
    }
    Ok(())
}

```

我们有了这样的一个函数，有同学可能觉得这代码眼熟。这不是标准库里的例子改了一下么？

.

.

.

是啊！

好了，继续，我们需要读取每个查到的文件，同时判断每一行里有没有所查找的内容。
我们用一个BufferIO去读取各个文件，同时用String的自带方法来判断内容是否存在。

```rust
fn call_back(de: &DirEntry, pt: &String) {
    let mut f = File::open(de.path()).unwrap();
    let mut buf = io::BufReader::new(f);
    for line in io::BufRead::lines(buf) {
        let line = line.unwrap_or("".to_string());
        if line.contains(pt) {
            println!("{}", &line);
        }
    }
}
```

最后，我们将整个函数调用起来，如下：

```rust
use std::env::args;
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;

fn visit_dirs(dir: &Path, pattern: &String, cb: &Fn(&DirEntry, &String)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), pattern, cb));
            } else {
                cb(&entry, pattern);
            }
        }
    }else{
        let entry = try!(try!(fs::read_dir(dir)).next().unwrap());
        cb(&entry, pattern);
    }
    Ok(())
}

fn call_back(de: &DirEntry, pt: &String) {
    let mut f = File::open(de.path()).unwrap();
    let mut buf = io::BufReader::new(f);
    for line in io::BufRead::lines(buf) {
        let line = line.unwrap_or("".to_string());
        if line.contains(pt) {
            println!("{}", &line);
        }
    }
}

// 实现调用grep命令搜索文件
fn main() {
    let mut arg_iter = args();
    arg_iter.next();
    // panic if there is no one
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt =  arg_iter.next().unwrap_or("./".to_string());
    let pt = Path::new(&pt);
    visit_dirs(&pt, &pattern, &call_back).unwrap();
}

```

调用如下：

```
➜  demo git:(master) ✗ ./target/debug/demo "fn main()" ../
fn main() {
fn main() { }
fn main() {
    pub fn main() {
    pub fn main() {}
fn main() {
    pub fn main() {
    pub fn main() {}
```

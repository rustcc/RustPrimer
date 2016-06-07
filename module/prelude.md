
# Prelude

Rust 的标准库，有一个 `prelude` 子模块，这里面包含了默认导入（std 库是默认导入的，然后 std 库中的 prelude 下面的东西也是默认导入的）的所有符号。

大体上有下面一些内容：

```rust
std::marker::{Copy, Send, Sized, Sync}
std::ops::{Drop, Fn, FnMut, FnOnce}
std::mem::drop
std::boxed::Box
std::borrow::ToOwned
std::clone::Clone
std::cmp::{PartialEq, PartialOrd, Eq, Ord}
std::convert::{AsRef, AsMut, Into, From}
std::default::Default
std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}
std::option::Option::{self, Some, None}
std::result::Result::{self, Ok, Err}
std::slice::SliceConcatExt
std::string::{String, ToString}
std::vec::Vec
```

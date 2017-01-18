# 输入与输出

输入与输出可以说是一个实用程序的最基本要求，没有输入输出的程序是没有什么卵用的。虽然输入输出被函数式编程语言鄙称为副作用，但正是这个副作用才赋予了程序实用性，君不见某著名函数式语言之父称他主导设计的函数式语言"[is useless](https://www.youtube.com/watch?v=iSmkqocn0oQ)"。这章我们就来谈谈输入输出副作用。

## 读写 Trait

输入最基本的功能是读(Read)，输出最基本的功能是写(Write)。标准库里面把怎么读和怎么写抽象出来归到了 `Read` 和 `Write` 两个接口里面，实现了 `Read` 接口的叫 reader，而实现了 `Write` 的叫 writer。Rust里面的 Trait 比其它语言里面的接口更好的一个地方是 Trait 可以带默认实现，比如用户定义的 reader 只需要实现 `read` 一个方法就可以调用 `Read` trait 里面的任意其它方法，而 writer 也只需要实现 `write` 和 `flush` 两个方法。

Read 和 Write 这两个 Trait 都有定义了好多方法，具体可以参考标准库 API 文档中的[Read](http://doc.rust-lang.org/stable/std/io/trait.Read.html) 和 [Write](http://doc.rust-lang.org/stable/std/io/trait.Write.html)

Read 由于每调用一次 `read` 方法都会调用一次系统API与内核交互，效率比较低，如果给 reader 增加一个 buffer，在调用时 `read` 方法时多读一些数据放在 buffer 里面，下次调用 `read` 方法时就有可能只需要从 buffer 里面取数据而不用调用系统API了，从而减少了系统调用次数提高了读取效率，这就是所谓的 `BufRead` Trait。一个普通的 reader 通过 `io::BufReader::new(reader)` 或者 `io::BufReader::with_capacity(bufSize, reader)` 就可以得到一个 BufReader 了，显然这两个创建 BufReader 的函数一个是使用默认大小的 buffer 一个可以指定 buffer 大小。BufReader 比较常用的两个方法是按行读： `read_line(&mut self, buf: &mut String) -> Result<usize>` 和 `lines(&mut self) -> Lines<Self>`，从函数签名上就可以大概猜出函数的用法所以就不啰嗦了，需要注意的是后者返回的是一个迭代器。详细说明直接看 API 文档中的[BufRead](http://doc.rust-lang.org/stable/std/io/trait.BufRead.html)

同样有 `BufWriter` 只不过由于其除了底层加了 buffer 之外并没有增加新的写方法，所以并没有专门的 `BufWrite` Trait，可以通过 `io::BufWriter::new(writer)` 或 `io::BufWriter::with_capacity(bufSize, writer)` 创建 `BufWriter`。

输入与输出接口有了，我们接下来看看实际应用中最常用的两类 reader 和 writer：标准输入/输出，文件输入/输出

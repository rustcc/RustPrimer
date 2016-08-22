# FFI


FFI([Foreign Function Interface](https://en.wikipedia.org/wiki/Foreign_function_interface))是用来与其它语言交互的接口，在有些语言里面称为语言绑定(language bindings)，Java 里面一般称为 JNI(Java Native Interface) 或 JNA(Java Native Access)。由于现实中很多程序是由不同编程语言写的，必然会涉及到跨语言调用，比如 A 语言写的函数如果想在 B 语言里面调用，这时一般有两种解决方案：一种是将函数做成一个服务，通过进程间通信([IPC](https://en.wikipedia.org/wiki/Inter-process_communication))或网络协议通信([RPC](https://en.wikipedia.org/wiki/Remote_procedure_call), [RESTful](https://en.wikipedia.org/wiki/Representational_state_transfer)等)；另一种就是直接通过 FFI 调用。前者需要至少两个独立的进程才能实现，而后者直接将其它语言的接口内嵌到本语言中，所以调用效率比前者高。

当前的系统编程领域大部分被 C/C++ 占领，而 Rust 定位为系统编程语言，少不了与现有的 C/C++ 代码交互，另外为了给那些"慢"脚本语言调用，Rust 必然得对 FFI 有完善的支持，本章我们就来谈谈 Rust 的 FFI 系统。

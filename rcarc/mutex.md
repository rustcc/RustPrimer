# Mutex 与 RwLock

## Mutex

`Mutex` 意为互斥对象，用来保护共享数据。Mutex 有下面几个特征：

1. `Mutex` 会等待获取锁令牌(token)，在等待过程中，会阻塞线程。直到锁令牌得到。同时只有一个线程的 `Mutex` 对象获取到锁；
2. `Mutex` 通过 `.lock()` 或 `.try_lock()` 来尝试得到锁令牌，被保护的对象，必须通过这两个方法返回的 `RAII` 守卫来调用，不能直接操作；
3. 当 `RAII` 守卫作用域结束后，锁会自动解开；
4. 在多线程中，`Mutex` 一般和 `Arc` 配合使用。

示例：

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

const N: usize = 10;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
let data = Arc::new(Mutex::new(0));

let (tx, rx) = channel();
for _ in 0..10 {
    let (data, tx) = (data.clone(), tx.clone());
    thread::spawn(move || {
        // The shared state can only be accessed once the lock is held.
        // Our non-atomic increment is safe because we're the only thread
        // which can access the shared state when the lock is held.
        //
        // We unwrap() the return value to assert that we are not expecting
        // threads to ever fail while holding the lock.
        let mut data = data.lock().unwrap();
        *data += 1;
        if *data == N {
            tx.send(()).unwrap();
        }
        // the lock is unlocked here when `data` goes out of scope.
    });
}

rx.recv().unwrap();
```

### `lock` 与 `try_lock` 的区别

`.lock()` 方法，会等待锁令牌，等待的时候，会阻塞当前线程。而 `.try_lock()` 方法，只是做一次尝试操作，不会阻塞当前线程。

当 `.try_lock()` 没有获取到锁令牌时，会返回 `Err`。因此，如果要使用 `.try_lock()`，需要对返回值做仔细处理（比如，在一个循环检查中）。


__点评__：Rust 的 Mutex 设计成一个对象，不同于 C 语言中的自旋锁用两条分开的语句的实现，更安全，更美观，也更好管理。


## RwLock

`RwLock` 翻译成 `读写锁`。它的特点是：

1. 同时允许多个读，最多只能有一个写；
2. 读和写不能同时存在；

比如：

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write().unwrap();
    *w += 1;
    assert_eq!(*w, 6);
} // write lock is dropped here
```

### 读写锁的方法

1. `.read()`
2. `.try_read()`
3. `.write()`
4. `.try_write()`

注意需要对 `.try_read()` 和 `.try_write()` 的返回值进行判断。

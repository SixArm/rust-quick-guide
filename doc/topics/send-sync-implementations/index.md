# Send & Sync - implementations

<https://stackoverflow.com/questions/68704717/>

The `Send` trait and `Sync` trait show up frequently in multi-threaded projects. Here are implementations that are especially good to know.

`&T`: since immutable references can be copied, the ability to send one to another thread would let you perform immutable access from several threads in parallel. Thus `&T` can only be `Send` if `T` is `Sync`. There is no need for `T` to be `Send` as an `&T` doesn't allow mutable access.

`&mut T`: mutable references can't be copied, so sending them to other threads doesn't allow access from several threads in parallel, thus `&mut T` can be `Send` even if `T` is not `Sync`. Of course, `T` must still be `Send`.

`MutexGuard`: destroying a `MutexGuard` on another thread is unsound, so it can't be `Send`. However if the value inside may be immutably accessed from several threads in parallel, then such an immutable access would also be safe on the `MutexGuard` itself.

`SyncWrapper`: an immutable reference to a `SyncWrapper<T>` does not allow you to perform any actions at all; it is always safe to be `Sync`.

`Rc<T>`: if you have two clones of the same `Rc<T>`, then it would be a data race to access them from different threads in parallel. This rules out both `Send` and `Sync`, since both of them would allow immutable access from other threads, and that other thread could use that to call `.clone()` remotely and obtain an `Rc<T>` on the other thread.

`Arc<T>`: this mostly behaves like `&T`. It can be cloned, so sending it to other threads requires `T: Sync`. However, it also requires `T: Send` as the last `Arc<T>` might be dropped on a different thread than where `T` was created, which you can't do without `Send`.

`RefCell<T>`: this type can never be `Sync` because you can modify the value inside with only an immutable reference, and this would be a data race if you could do it from several threads in parallel. There's no problem with `RefCell<T>` being `Send` provided that `T` is.

# Design patterns: singleton

The "singleton" design pattern ensures that only one instance of a particular object is ever created. This can be implemented using a static variable or a lazy static variable.

Example:

```rust
struct Singleton;

impl Singleton {
    fn instance() -> &'static Self {
        static mut INSTANCE:
            *const Singleton = 0 as *const Singleton;
        static ONCE: Once = Once::new();
        unsafe {
            ONCE.call_once(|| {
                let singleton = Singleton {};
                INSTANCE = mem::transmute(Box::new(singleton));
            });

            &*INSTANCE
        }
    }
}
```

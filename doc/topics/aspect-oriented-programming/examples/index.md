# Aspect-oriented programming - example

Example:

```rust
aspect!(LoggingAspect, {
    fn before(&self, args: &MethodArguments) {
        println!("before {}", args.method_name());
    }
    fn after_returning(
        &self, args: &MethodArguments, result: &Any
    ) {
        println!("{} result {:?}", args.method_name(), result);
    }
    fn after_throwing(
        &self, args: &MethodArguments, err: &Error
    ) {
        println!("{} err {:?}", args.method_name(), err);
    }
});

struct UserService {}

impl UserService {
    fn get_user(&self, id: u32) -> Result<User, Error> {
        if id == 1 {
            return Ok(User { id, name: "Alice" })
        }
        Err(Error::new(ErrorKind::Other, "Not Found"))
    }
}

fn main() {
    let logging = LoggingAspect::new();
    let user_service = UserService::new();
    let proxy = aspect::Proxy::new(user_service, &logging);
    let result = proxy.get_user(1); // Call intercepted method
    match result {
        Ok(user) => println!("User: {:?}", user),
        Err(err) => println!("Error: {:?}", err),
    }
}
```

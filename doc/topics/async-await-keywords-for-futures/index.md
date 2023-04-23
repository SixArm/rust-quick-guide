# async/await keywords for futures

Rust provides support for asynchronous programming through its `async`/`await` syntax. The `async` keyword defines a function that can be suspended and resumed later. The `await` keyword pauses execution of an `async` function until a condition is met.

When a function is declared with the `async` keyword, it becomes an asynchronous function. This means that the function can be paused at any point using the await keyword and resumed later when the awaited value becomes available. The async function returns a `Future` type that represents the result of the computation.

Example of an async function that returns a future:

```rust
async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```

The example's `fetch` function is defined with the `async` keyword. The function uses the `reqwest` crate to make an HTTP request. The first `await` waits for the response. The second `await` waits for the body text.

Example of `await` that waits for a future:

```rust
async fn do_something() -> i32 {
    let future = get_result_async();
    let result = await!(future);
    result + 1
}
```

The example's `await!` pauses execution of the `do_something` function until `get_result_async` is completed. Once the future completes, the result is returned and the task is resumed. The value of the result is then incremented by 1 and returned as the final result.
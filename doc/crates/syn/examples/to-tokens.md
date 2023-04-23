# syn crate - example to_tokens

<!-- projects/crates/syn -->

Suppose we have a `struct` with two fields:

```rust
struct Person {
    name: String,
    age: u32,
}
```

We can use the Syn crate to generate a code block that implements the `ToTokens` trait for our `Person` struct, which will allow us to convert any instance of `Person` into a token stream that can be used, for example, macros:

```rust
extern crate syn;

impl syn::ToTokens for Person {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let age = &self.age;
        let expanded = quote! {
            Person {
                name: String::from(#name),
                age: #age,
            }
        };
        expanded.to_tokens(tokens);
    }
}
```

Here we implement the `to_tokens` method to generate a token stream that represents the `Person` struct with the provided values. This code uses the `quote` macro to generate the token stream, which in turn uses the `#` symbol to interpolate variables into the stream.

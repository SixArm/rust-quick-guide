# str type versus String type

In Rust, both `str` and `String` are used to represent textual data, always using Unicode UTF-8 encoded u8 bytes. But they have some differences in terms of how they are stored and accessed.

* Type: `str` is a primitive type. `String` is a standard library type.

* Memory: `str` is immutable, and can be stored in the program's binary, or stack, or heap. `String` is mutable, growable, and heap-allocated.

* Ownership: `&str` is a slice that borrows ownership from another string or a static string literal. In contrast, `String` owns the string data it contains.

* Lifetime: `str` has a static lifetime (i.e., it lives as long as the program runs) in case of string literals, or a borrowed lifetime (i.e., it lives as long as the reference it was borrowed from) in case of borrowed slices. `String` has a dynamic lifetime (i.e., it lives as long as there is a reference to it).

* Usage: `&str` is usually used for function arguments and return types, or for string literals, while `String` is typically used when you need to create or modify a string at runtime.

* Coercion: a `&String` can be coerced to a `&str`, such as when a `&String` arg is passed to a function signature with a `&str` parameter.

* Indexing: Indexing by bytes is different than counting by characters, because `str` and `String` both store Unicode characters, using the UTF-8 variable-width encoding format, which means one character can take up more than one byte.

# String types for UTF-8, C, OS, and paths

Rust provides different string types for different kinds of text.

`str` and `String`: a Unicode UTF-8 value sequence. A `str` is slice-like and immutable. A `String` is owned, mutable, and growable.

`CStr` and `CString`: a C-style null-terminated char byte sequence. A `CStr` is slice-like and immutable. A `CString` is owned, mutable, and growable.

`OsStr` and `OsString`: a platform-specific operating system string. A `OsStr` is slice-like and immutable. A `OsString` is owned, mutable, and growable.

`Path` and `PathBuf`: a platform-specific file path string. A `Path` is slice-like and immutable. A `PathBuf` is owned, mutable, and growable.

Converting between a platform-specific type (`OsStr`, `OsString`, `Path`, `PathBuf`) and a platform-independent type (`str`, `String`) may require lossy conversion, or handling conversion errors.

Examples of string types:

```rust
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

let a: &str = "foo";
let b: String = String::from("foo");

let c: &CStr =  CStr::from_bytes_with_nul(b"foo\0")
    .expect("Error in CStr::from_bytes_with_nul");
let d: CString = CString::from_vec_with_nul(b"foo\0".to_vec())
    .expect("Error in CString::from_vec_with_nul");

let e: &OsStr = OsStr::new("foo");
let f: OsString = OsString::from("foo");

let g: &Path = Path::new("foo");
let h: PathBuf = PathBuf::from("foo");
```

use std::ffi::{CStr, CString, OsStr, OsString};
use std::path::{Path, PathBuf};

fn main() {
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

    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        a, b, c, d, e, f, g, h
    )
}

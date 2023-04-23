//! Tell tonic-build to compile your protobufs when you build your Rust project. 
//!
//! You can configure this build process in a number of ways, however we will not
//! get into the details in this introductory tutorial. 
//!
//! Please see the tonic-build documentation for details on configuration.
//!
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
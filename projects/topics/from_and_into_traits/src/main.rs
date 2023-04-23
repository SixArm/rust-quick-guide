struct MyStruct(pub i32);

// Convert from i32
impl From<i32> for MyStruct {
    fn from(val: i32) -> Self {
        MyStruct(val)
    }
}

// Convert into i32
impl Into<i32> for MyStruct {
    fn into(self) -> i32 {
        self.0
    }
}

fn main() {
    let my_struct = MyStruct::from(1);
    println!("mystruct.0 is {}", &my_struct.0);
    let i: i32 = my_struct.into();
    println!("i is {:?}", i);
}

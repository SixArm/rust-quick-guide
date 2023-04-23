trait MyTrait {
    fn my_method(&self);
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn my_method(&self) {
        println!("hello");
    }
}

fn foo<T: MyTrait>(item: T) {
    item.my_method();
}

fn main() {
    let s = MyStruct{};
    foo(s)
}

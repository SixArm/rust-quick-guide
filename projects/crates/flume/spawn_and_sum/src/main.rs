use std::thread;

fn main() {
    let (tx, rx) = flume::unbounded();
    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        })
    });
    let received: u32 = rx.iter().sum();
    assert_eq!((0..10).sum::<u32>(), received);
    println!("{}", received);
}

use assertables::*;

fn main() {

    // For values
    assert_lt!(1, 2); // less than
    assert_gt!(2, 1); // greater than

    // For strings
    assert_starts_with!("hello world", "hello");
    assert_not_starts_with!("hello world", "foo");

    // For sets
    assert_set_superset!([1, 2, 3], [1, 2]);
    assert_set_disjoint!([1, 2, 3], [4, 5, 6]);

    // All assertions passed
    println!("Success");

}

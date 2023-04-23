# What are Rust Guideposts' projects?

<https://github.com/sixarm/rust-guideposts>

Rust Guideposts provides sample projects. These projects are small Rust programs that you can read, build, and run. Each project demonstrates one quick topic summary, or demonstrates one crate. The projects are in the Rust Guideposts repository, in the `projects` directory.

Some of the projects for topics are:

* `from_and_into_traits`
* `closures_for_iterators`
* `test_driven_development`
* `pass_by_value_or_reference`
* `the_borrow_checker`

Some of the projects for crates are:

* `assertables/values_strings_sets`
* `csv/read_spreadsheet_file`
* `reqwest/make_http_request`
* `serde/parse_json_data`
* `sqlx/create_table_insert_into_select`

Example command to run a project:

```sh
cd projects/topics/hello_world
cargo run
```

Many of the projects also include a simple integration test:

```sh
cd projects/topics/hello_world
cargo test
```

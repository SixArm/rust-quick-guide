# Rust Guideposts

Rust Guideposts are quick topic summaries to help people learn about the Rust programming language, ecosystem, concepts, crates, and more. You can try any topic, in any order, at any time, akin to a frequently asked questions resource.

Rust Guideposts work best as an adjunct to a comprehensive Rust book, such as the The Rust Programming Language. The book is valuable to read, cover to cover, for its excellent technical explanations.

We welcome constructive advice, new topic ideas, pull requests, open issues, and the like. See CONTRIBUTING.md. Rust Guideposts are continually evolving, with ongoing additions, corrections, and optimizations.


## Start here


### Introduction

<!-- [](doc/book/sections/introduction) -->

* [What are Rust Guideposts?](doc/topics/what-are-rust-guideposts)

* [What are Rust Guideposts' projects?](doc/topics/what-are-rust-guideposts-projects)

* [Who is this for?](doc/topics/who-is-this-for)

* [How can I contribute?](doc/topics/how-can-i-contribute)


### Welcome to Rust

<!-- [](doc/book/sections/welcome-to-rust) -->

* [What is a Rust "Hello World" program?](doc/topics/what-is-a-rust-hello-world-program) + [project](projects/topics/hello_world)

* [What is a Rust "FizzBuzz" program?](doc/topics/what-is-a-rust-fizzbuzz-program) + [project](projects/topics/fizzbuzz/fizzbuzz_using_if_else)

* [What is a Rust "Fibonacci" program?](doc/topics/what-is-a-rust-fibonacci-program) + [project](projects/topics/fibonacci/fibonacci_using_recursion)


### Learning

<!-- [](doc/book/sections/learning) -->

* [What makes Rust good?](doc/topics/what-makes-rust-good)

* [What is the Rust ecosystem?](doc/topics/what-is-the-rust-ecosystem)

* [Who might benefit from learning Rust?](doc/topics/who-might-benefit-from-learning-rust)

* [What are good ways to learn Rust?](doc/topics/what-are-good-ways-to-learn-rust)

* [What are good projects to learn Rust?](doc/topics/what-are-good-projects-to-learn-rust)


### Caveats

<!-- [](doc/book/sections/caveats) -->

* [Is Rust a good first language?](doc/topics/is-rust-a-good-first-language)

* [What are the hardest parts of Rust?](doc/topics/what-are-the-hardest-parts-of-rust)

* [Rust stable versus Rust nightly](doc/topics/rust-stable-versus-rust-nightly)

* [What is Rust missing?](doc/topics/what-is-rust-missing)

* [What are some non-goals of Rust?](doc/topics/what-are-some-non-goals-of-rust)

* [Why do companies not use Rust?](doc/topics/why-do-companies-not-use-rust)


## What makes Rust special?

<!-- [](doc/book/sections/what-makes-rust-special) -->

* [The borrow checker](doc/topics/the-borrow-checker) + [example](doc/topics/the-borrow-checker/examples)

* [Channels for thread communication](doc/topics/channels-for-thread-communication)

* [Concurrency and parallelism](doc/topics/concurrency-and-parallelism)

* [Error messages](doc/topics/error-messages)

* [Foreign Function Interface (FFI)](doc/topics/foreign-function-interface)

* [Futures for asynchronous operations](doc/topics/futures-for-asynchronous-operations)

* [Monomorphisation of generic code](doc/topics/monomorphisation-of-generic-code)

* [Resource Acquisition Is Initialization (RAII)](doc/topics/resource-acquisition-is-initialization)

* [Unsafe code](doc/topics/unsafe-code)

* [WebAssembly (WASM)](doc/topics/webassembly-wasm) + [example](doc/topics/webassembly-wasm/examples)

* [Zero-cost abstractions](doc/topics/zero-cost-abstractions) + [example](doc/topics/zero-cost-abstractions/examples)


## Types, Traits, Keywords, Macros


### Type guideposts

<!-- [](doc/book/sections/types) -->

* [Scalar types](doc/topics/scalar-types)

* [Compound types](doc/topics/compound-types)

* [Sum types and product types](doc/topics/sum-types-and-product-types)

* [str type versus String type](doc/topics/str-type-versus-string-type)

* [String types for UTF-8, C, OS, and paths](doc/topics/string-types)

* [Box type for a smart pointer](doc/topics/box-type-for-a-smart-pointer)

* [Cow type for clone-on-write](doc/topics/cow-type-for-clone-on-write)

* [RefCell type for dynamic borrowing](doc/topics/refcell-type-for-dynamic-borrowing)

* [Rc type for single-thread sharing](doc/topics/rc-type-for-single-thread-sharing)

* [Arc type for multi-thread sharing](doc/topics/arc-type-for-multi-thread-sharing)

* [Pin type for memory location](doc/topics/pin-type-for-memory-location) + [example](doc/topics/pin-type-for-memory-location/examples)


### Trait guideposts

<!-- [](doc/book/sections/traits) -->

* [Copy trait and Clone trait for duplicating values](doc/topics/copy-trait-and-clone-trait-for-duplication)

* [Debug trait for debugging and printing](doc/topics/debug-trait-for-debugging-and-printing)

* [Display trait for formatting](doc/topics/display-trait-for-formatting)

* [dyn trait for dynamic dispatch](doc/topics/dyn-trait-for-dynamic-dispatch)

* [Eq, PartialEq, Ord, PartialOrd traits](doc/topics/eq-partialeq-ord-partialord-traits)

* [From and Into traits for conversions](doc/topics/from-and-into-traits-for-conversions)

* [Send trait for sending among threads](doc/topics/send-trait-for-sending-among-threads)

* [Sync trait for syncing among threads](doc/topics/sync-trait-for-syncing-among-threads)

* [Send & Sync - implementations](doc/topics/send-sync-implementations)

* [Sealed traits](doc/topics/sealed-traits)


## Keywords guideposts

<!-- [](doc/book/sections/keywords) -->

* [enum keyword for enumerations](doc/topics/enum-keyword-for-enumerations)

* [struct keyword for structures](doc/topics/struct-keyword-for-structures)

* [union keyword for multi-type memory](doc/topics/union-keyword-for-multi-type-memory)

* [match keyword for control flow](doc/topics/match-keyword-for-control-flow)

* [mod keyword for module namespaces](doc/topics/mod-keyword-for-module-namespaces)

* [mod keyword for nested hierarchies](doc/topics/mod-keyword-for-nested-hierarchies)

* [nested-or-pattern for matching](doc/topics/nested-or-pattern-for-matching)
  
* [async/await keywords for futures](doc/topics/async-await-keywords-for-futures)

* [trait keyword for polymorphism](doc/topics/trait-keyword-for-polymorphism)


### Macro guideposts

<!-- [](doc/book/sections/macros) -->

* [println! macro for printing output](doc/topics/println-macro)

* [assert! macro and friends for testing](doc/topics/assert-macro)

* [regex! macro for lazy static optimization](doc/topics/regex-macro)

* [catch_unwind! macro for handling panic](doc/topics/catch-unwind-macro)

* [macro_rules! macro for declarative macros](doc/topics/macro-rules-macro)


## Coding


### Syntax

<!-- [](doc/book/sections/syntax) -->

* [Annotations for compiler directives](doc/topics/annotations-for-compiler-directives)

* [Destructuring into components](doc/topics/destructuring-into-components)

* [Iterators for traversing collections](doc/topics/iterators-for-traversing-collections)

* [Closures for anonymous functions](doc/topics/closures-for-anonymous-functions)

* [Macros for metaprogramming](doc/topics/macros-for-metaprogramming)

* [Panic and how to handle it with a hook](doc/topics/panic-and-how-to-handle-it-with-a-hook)

* [Pass by value or reference](doc/topics/pass-by-value-or-reference)

* [Range syntax for a sequence of values](doc/topics/range-syntax-for-a-sequence-of-values)

* [Tuples for ordered collections](doc/topics/tuples-for-ordered-collections)


### Memory

<!-- [](doc/book/sections/memory) -->

* [Memory lifetimes](doc/topics/memory-lifetimes)

* [Implicit lifetimes](doc/topics/implicit-lifetimes)

* [Explicit lifetimes](doc/topics/explicit-lifetimes)

* [Memory on the stack or the heap](doc/topics/memory-on-the-stack-or-the-heap)

* [Memory ownership and borrowing](doc/topics/memory-ownership-and-borrowing) + [example](doc/topics/memory-ownership-and-borrowing/examples)

* [Mutability and immutability](doc/topics/mutability-and-immutability)

* [No garbage collection](doc/topics/no-garbage-collection)

* [Borrow splitting](doc/topics/borrow-splitting)


### Testing

<!-- [](doc/book/sections/testing) -->

* [Test framework](doc/topics/test-framework)

* [Test assertions](doc/topics/test-assertions)

* [Unit testing](doc/topics/unit-testing)

* [Integration testing](doc/topics/integration-testing)

* [Documentation testing](doc/topics/documentation-testing)

* [Documentation testing annotations](doc/topics/documentation-testing-annotations)
  
* [Source-based code coverage](doc/topics/source-based-code-coverage)

* [Test-driven development (TDD)](doc/topics/test-driven-development-tdd) + [example](doc/topics/test-driven-development-tdd/examples) + [projecct](projects/topics/test_driven_development)


### Examples

<!-- [](doc/book/sections/examples) -->

* [Access a database with rusqlite](doc/topics/acccess-a-database-with-rusqlite) + [project](projects/topics/access_a_database_with_rusqlite)

<!-- * [Aspect-oriented programming (AOP)](!doc/topics/aspect-oriented-programming) + [example](!doc/topics/aspect-oriented-programming/examples) + [project](!projects/topics/aspect-oriented-programming) -->

* [Benchmark run times with Bencher](doc/topics/benchmark-times-with-bencher) + [example](doc/topics/benchmark-times-with-bencher/examples) + [project](projects/topics/benchmark_times_with_bencher)

* [Concurrency with std::thread](!doc/topics/concurrency-with-std-thread)

* [Liskov substitution principle (LSP)](doc/topics/liskov-substitution-principle) + [example](doc/topics/liskov-substitution-principle/examples)

* [List directories recursively with walkdir](doc/topics/list-directories-recursively-with-walkdir) + [project](projects/topics/list_directories_with_walkdir)

* [Make HTTP request with reqwest](doc/topics/make-http-request-with-reqwest) + [project](projects/topics/make_http_request_with_reqwest)

* [Memoize a function with cached](doc/topics/memoize-a-function-with-cached) + [project](projects/topics/memoize_a_function_with_cached)

* [Parallel iterate with search-](doc/topics/parallel-iterate-with-rayon) + [project](projects/topics/parallel_iterate_with_rayon)

* [Parse JSON data with Serde](doc/topics/parse-json-data-with-serde) + [project](projects/topics/parse_json_data_with_serde)

* [Read a spreadsheet with CSV](doc/topics/read-a-spreadsheet-with-csv) + [project](projects/topics/read_a_spreadsheet_with_csv)

* [Run a terminal program with cursive](doc/topics/run-a-terminal-program-with-cursive) + [project](projects/topics/run_a_terminal_program_with_cursive)

* [Search text file lines with regex](doc/topics/search-text-file-lines-with-regex) + [project](projects/topics/search_text_file_lines_with_regex)


## Tooling & tactics


### Tooling we use often

<!-- [](doc/book/sections/tooling) -->

* [rustup command-line tool](doc/topics/rustup-command-line-tool)

* [Cargo package manager and crates](doc/topics/cargo-package-manager-and-crates)

* [cargo-install-favorites for many tools](doc/topics/cargo-install-favorites)

* [Blessed recommendations](doc/topics/blessed-recommendations)

* [Clippy linting](doc/topics/clippy-linting)

* [Helix text editor for writing code](doc/topics/helix-text-editor/)

* [Rustfmt for code formatting](doc/topics/rustfmt-for-code-formatting) + [examples](doc/topics/rustfmt-for-code-formatting/examples)

* [Rust mdBook for documentation](doc/topics/rust-mdbook-for-documentation)

* [Cross-compiling for multiple platforms](doc/topics/cross-compiling-for-multiple-platforms)

* [Rhai script](doc/topics/rhai-script)



### Tooling concepts

<!-- [](doc/book/sections/tooling-concepts) -->

* [Abstract syntax tree (AST)](doc/topics/abstract-syntax-tree-ast)

* [Tree-sitter parsing library](doc/topics/tree-sitter-parsing-library)

* [Language Server Protocol (LSP)](doc/topics/language-server-protocol-lsp)

* [Static analysis for error detection](doc/topics/static-analysis-for-error-detection)

* [Debugger for diagnostics](doc/topics/debugger-for-diagnostics) + [example](doc/topics/debugger-for-diagnostics/examples) and


### Design patterns

<!-- [](doc/book/sections/design-patterns) -->

* [Design patterns: introduction](doc/topics/design-patterns-introduction)

* [Design patterns: adapter](doc/topics/design-patterns-adapter)

* [Design patterns: builder](doc/topics/design-patterns-builder)

* [Design patterns: observer](doc/topics/design-patterns-observer)

* [Design patterns: singleton](doc/topics/design-patterns-singleton)


## Crates


### Crates we like for many of our programs

<!-- [](doc/book/sections/crates/many) -->

* [Assertables crate for assert macro tests](doc/crates/assertables) + [project](projects/crates/assertables/values_strings_sets)

* [cached crate for memoization](doc/crates/cached) + [project](projects/crates/cached/memoize)

* [log crate for logging messages](doc/crates/log) + [project](projects/crates/log/info_warn_error_debug)

* [itertools crate for iterator extras](doc/crates/itertools) + [project](projects/crates/itertools/demo)

* [num crate for number types and traits](doc/crates/num) + [project](!projects/crates/num/?)

* [once_cell crate for lazy global variables](doc/crates/once-cell) + [project](projects/crates/once_cell/lazy_static_regex)

* [syn crate for syntax analysis](doc/crates/syn) <!-- + [project](!projects/crates/syn) -->

* [regex crate for regular expressions](doc/crates/regex) + [project](projects/crates/regex/demo)

* [Serde crate for serialize/deserialize](doc/crates/serde) + [project](projects/crates/serde/parse_json_data)

* [Strum crate for enums](doc/crates/strum) + [project](projects/crates/strum/demo)

* [rand crate for random numbers](doc/crates/rand) + [project](projects/crates/rand/rng_gen_range)


### Crates we like for command line interfaces

<!-- [](doc/book/sections/crates/command-line-interfaces) -->

* [CLAP crate for commands](doc/crates/clap) + [example](doc/crates/clap/examples) + [project](projects/crates/clap/command_builder)

* [Textwrap crate for text wrapping](doc/crates/textwrap) + [example](doc/crates/textwrap/examples) + [project](projects/crates/textwrap/fill_wrap)

* [Cursive crate for text user interfaces](doc/crates/cursive) + [example](doc/crates/cursive/examples) + [project](!projects/crates/cursive/hello_world)

* [TUI crate for text user interfaces](doc/crates/tui) + [example](doc/crates/tui/examples) + [project](projects/crates/tui/terminal_draw_block)

* [walkdir crate for traversing directories](doc/crates/walkdir) + [project](projects/crates/walkdir/list_entries)


### Crates we like for development workflows

<!-- [](doc/book/sections/crates/development) -->

* [cargo-cache crate for caching builds](doc/crates/cargo-cache)

* [cargo-crev for community-driven trust](doc/crates/cargo-crev)

* [cargo-dist crate for distribution archives](doc/crates/cargo-dist)

* [cargo-release crate for publishing](doc/crates/cargo-release) + [examples](doc/crates/cargo-release/examples)

* [cargo-make crate for task runners](doc/crates/cargo-make) + [examples](doc/crates/cargo-make/examples)

* [Criterion crate for benchmarks](doc/crates/criterion) + [example](doc/crates/criterion/examples)


### Crates we like for concurrency and parallelism

<!-- [](doc/book/sections/crates/concurrency-and-parallelism) -->

* [Crossbeam crate for concurrency](doc/crates/crossbeam) <!-- + [project](!projects/crates/crossbeam) -->

* [epoll for event polling](doc/crates/epoll) + [example](doc/crates/epoll/examples) + [project](projects/crates/epoll/tcp_listener)

* [Flume crate for channels](doc/crates/flume) + [project](projects/crates/flume/spawn_and_sum)

* [parking_lot crate for synchronization](doc/crates/parking-lot) <!-- + [project](!projects/crates/parking-lot) -->

* [Rayon crate for parallelism](doc/crates/rayon) <!-- + [project](!projects/crates/rayon) -->


### Crates we like for data

<!-- [](doc/book/sections/crates/data) -->

* [arrow-csv crate for loading CSV to Arrow](doc/crates/arrow-csv) <!-- + [project](!projects/crates/arrow-csv) -->

* [CSV crate for comma-separated values](doc/crates/csv) + [project](projects/crates/csv/read_a_spreadsheet_file)

* [Diesel crate for object-relational mapping](doc/crates/diesel) + [example](doc/crates/diesel/examples) + [project](projects/crates/diesel/hello_world_with_sqlite)

* [Polars crate for data analysis](doc/crates/polars) <!-- + [project](!projects/crates/polars) -->

* [Rusqlite crate for SQLite databases](doc/crates/rusqlite) + [project](projects/crates/rusqlite/create_table_insert_into_select_from)

* [sqlx crate for SQL databases](doc/crates/sqlx) + [example](doc/crates/sqlx/examples) + [project](projects/crates/sqlx/create_table_insert_into_select)


### Crates we like for web applications

<!-- [](doc/book/sections/crates/web) -->

* [axum crate for web services](doc/crates/axum) + [example](doc/crates/axum/examples) + [project](projects/crates/axum/hello_world)

* [Hyper crate for HTTP clients/servers](doc/crates/hyper) + [project](projects/crates/hyper/hello_world)

* [prost crate for protocol buffers](doc/crates/prost) + [example](doc/crates/prost/examples) + [project](projects/crates/prost/serialize_deserialize_protobuf)

* [reqwest crate for HTTP requests](doc/crates/reqwest) + [project](projects/crates/reqwest/make_http_request)

* [Sycamore crate for reactive front-end](doc/crates/sycamore) + [example](doc/crates/sycamore/examples) <!-- + [project](!projects/crates/sycamore) -->

* [Tokio crate for async/concurrency](doc/crates/tokio) + [example](doc/crates/tokio/examples) + [project](!projects/crates/tokio/http_server)

* [tonic crate for gRPC](doc/crates/tonic) + [example server](doc/crates/tonic/examples/server) + [example client](doc/crates/tonic/examples/client) + [project](projects/crates/tonic/hello_world)

* [Yew crate for client-side web apps](doc/crates/yew) + [example](doc/crates/yew/examples) <!-- + [project](!projects/crates/yew) -->


### Crates we like for graphics & games

<!-- [](doc/book/sections/crates/graphics) -->

* [gtk4 crate for GTK GUIs](doc/crates/gtk4) + [example](doc/crates/gtk4/examples) + [project](!projects/crates/gtk4/hello_world) -->

* [egui crate for pure Rust GUIs](doc/crates/egui) + [project](projects/crates/egui/hello_world_with_eframe)

* [Bevy crate for game programming](doc/crates/bevy) + [example](doc/crates/bevy/examples) <!-- + [project](!projects/crates/bevy/hello_world) -->

* [macroquad crate for simple games](doc/crates/macroquad) + [project](projects/crates/macroquad/hello_world)


### Rust going forward

<!-- [](doc/book/sections/forward) -->

* [Rust governance](doc/topics/rust-governance)

* [The Rust Foundation](doc/topics/the-rust-foundation)

* [The Rust RFC process](doc/topics/the-rust-rfc-process)

* [The Rust roadmap](doc/topics/the-rust-roadmap)


### Backmatter

<!-- [](doc/book/sections/backmatter) -->

* [About the author](doc/topics/about-the-author)

* [About the ebook PDF](doc/topics/about-the-ebook-pdf)

* [About related projects](doc/topics/about-related-projects)


## Tracking

* Project: rust-guideposts
* Version: 1.0.0
* Created: 2023-03-08T20:29:33Z
* Updated: 2023-04-10T15:23:16Z
* License: CC-BY-NC-SA-4.0
* Website: <https://github.com/sixarm/rust-guideposts>
* Contact: Joel Parker Henderson (joel@sixarm.com)

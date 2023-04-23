# cargo-release crate - examples

The cargo-release crate provides many features and functions, including these examples.

Release Management: The `cargo-release` crate provides a range of tools for managing the release process, including the ability to automatically generate a new version number based on a specified release type (e.g. major, minor, or patch), update the changelog and version number in your crate's Cargo.toml file, tag the release in Git, and publish the crate to crates.io:

```bash
cargo release --dry-run  # preview the release process
cargo release            # perform the release
```

Pre-Release Management: The `cargo-release` crate also provides tools for managing pre-releases, including the ability to create and publish pre-release versions of your crate (e.g. 0.2.0-alpha.1), and to promote pre-release versions to stable releases:

```bash
cargo release --pre-release  # create a pre-release version
cargo release --continue     # promote a pre-release to stable
```

Customization: The `cargo-release` crate is highly configurable, allowing you to customize the release process to suit your needs. For example, you can specify which branches to release from, configure the changelog format and location, and specify additional steps to perform during the release process:

```toml
[release]
branches = ["main"]
changelog = "docs/CHANGELOG.md"
pre-release = false

[release.steps.post]
# ... additional steps to perform after the release ...
```

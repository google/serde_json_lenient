# Release Process

* Update version in `Cargo.toml` and `src/lib.rs`
* Commit with message "Release x.y.z"
* Tag `vx.y.z`
* `cargo publish`
* Push changes to the repo (directly, without a PR), including tags
* Create a new release from the pushed tag at https://github.com/google/serde_json_lenient/tags, summarizing the changes and linking to PRs.

# Rust Sycamore Halfmoon framework

Rust based Sycamore web app example using the halfmoon CSS framework.

## Usage of Trunk

For a more thorough explanation of Trunk and its features, please head over to the [repository](https://github.com/thedodd/trunk).

### Running

Just click on the button above to start a workspace. Trunk will automatically be installed and a dev server will automatically be started.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

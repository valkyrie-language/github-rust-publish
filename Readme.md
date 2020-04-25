

### Configuration file query order

```sh
name: Github Rust Publish

on:
  push:
    branches: [ master, dev ]
  pull_request:
    branches: [ master, dev ]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - name: Publish Rust Binary
        uses: valkyrie-language/github-rust-publish@v0.0.0
        with:
          config: .config/Publish.toml
```
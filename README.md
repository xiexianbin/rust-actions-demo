# rust-actions-demo

[![build-test](https://github.com/xiexianbin/rust-actions-demo/actions/workflows/workflow.yaml/badge.svg)](https://github.com/xiexianbin/rust-actions-demo/actions/workflows/workflow.yaml)
[![crates.io](https://img.shields.io/crates/v/x_demo.svg)](https://crates.io/crates/x_demo)

rust project demo/template with github actions.

## usage

- release

```
// create git tag and binary
git tag v0.1.0
git push origin --tags

// publish to https://crates.io/crates/<xxx>
cargo publish
```

- download
```
curl -Lfs -o main https://github.com/xiexianbin/rust-actions-demo/releases/latest/download/demo  # -{linux|darwin|windows}
chmod +x demo
./demo
```

or

```shell
cargo install x_demo
```

- 其他示例

https://github.com/xiexianbin/rust-httpserver

# rust-actions-demo

[![build-test](https://github.com/xiexianbin/rust-actions-demo/actions/workflows/workflow.yaml/badge.svg)](https://github.com/xiexianbin/rust-actions-demo/actions/workflows/workflow.yaml)

rust project demo/template with github actions.

## usage

- release

```
git tag v0.1.0
git push origin --tags
```

- download
```
curl -Lfs -o main https://github.com/xiexianbin/rust-actions-demo/releases/latest/download/demo  # -{linux|darwin|windows}
chmod +x demo
./demo
```

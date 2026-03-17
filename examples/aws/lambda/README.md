# Build AWS Lambda layer

This guide explains how to build and deploy a custom AWS Lambda layer for the khmercut-rs Rust library, making it available for your Lambda functions.

## 🛠️ Prerequisites

- Setup AWS CLI and configure it with your credentials and profile. e.g. `angkor`
- Create a S3 bucket called `sls-assets` in the `ap-southeast-1` region.


```shell 

## 🚀 Usage

Deploy a layer to AWS:

```sh
# Default (aarch64, version 0.2.0)
make

# Specify architecture (e.g., x86_64)
make ARCH=x86_64

# Specify version
make VERSION=0.2.1

# Specify both architecture and version
make ARCH=x86_64 VERSION=0.2.1
```

## 📚 Layers

| Layer                                                      | version |
|------------------------------------------------------------|---------|
| arn:aws:lambda:ap-southeast-1:AWS_ACCOUNT_ID:layer:khmercut:VERSION| VERSION      |
| arn:aws:lambda:ap-southeast-1:692859920286:layer:khmercut:3        | 3            |


## 🔗 References

- [serverless-rust](https://www.serverless.com/plugins/serverless-rust)

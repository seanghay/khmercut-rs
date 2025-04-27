# Build AWS Lambda layer

This guide explains how to build and deploy a custom AWS Lambda layer for the khmercut-rs Rust library, making it available for your Lambda functions.

## 🛠️ Prerequisites

- Setup AWS CLI and configure it with your credentials and profile. e.g. `angkor`
- Create a S3 bucket called `sls-assets` in the `ap-southeast-1` region.


```shell 

## 🚀 Usage

Deploy a layer to aws

```shell
make
```

## 📚 Layers

| Layer                                                      | version |
|------------------------------------------------------------|---------|
| arn:aws:lambda:ap-southeast-1:AWS_ACCOUNT_ID:layer:khmercut:VERSION| VERSION      |


## 🔗 References

- [serverless-rust](https://www.serverless.com/plugins/serverless-rust)

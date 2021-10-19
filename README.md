# AWS Lambda and Rust - Pokemon names

Send your name's initial and get a matching Pokemon name.
AWS Lambda API written in Rust

## How it works:
You make a POST request with a body like this, where "C" is your name's first letter:
```
{
  "command": "C"
}
```

then a AWS Lambda function written in Rust will get you a Pokemon name in the response.
This is just a quick lambda proof of concept, it works and can be expanded to get full stats and that kind of stuff.

## How to run:
To deploy as a lambda function needs to be manually built for Amazon Linux, so the build needs a `x86_64-unknown-linux-musl` [platform target](https://github.com/awslabs/aws-lambda-rust-runtime)

Once compiled the executable needs to be renamed to 'bootstrap' and add it to a zip file. Then it can be uploaded using the AWS CLI or the UI.
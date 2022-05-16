A simple example with CDK V2 Python and Rust.

## Install & Deploy
Install AWS CDK CLI
```bash
npm install -g aws-cdk
```
Install pip dependencies (I suggest the use of a virtual env)
```bash
pip install -U -r requirements.txt
```
Build code with
```bash
rustup target add x86_64-unknown-linux-musl (only the first time)
./cdk-build.sh
```
Deploy Lambda
```bash
cdk deploy LambdaRustTestStack [--profile <local-aws-profile>]
```

## Test Lambda
The request and response event formats follow the same schema as the Amazon 
[API Gateway payload format version 2.0](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format).
For more information about AWS Lambda Function-Url, see [AWS Lambda doc](https://docs.aws.amazon.com/lambda/latest/dg/lambda-urls.html).
### Locally (SAM)
```bash
./cdk-build.sh
cdk synth --no-staging > template.yml
sam local invoke LambdaRustTestXXXXXXX --event tests/lambda-function-url-input-example.json
```
### Function url via curl (TODO code handler)
Get function url from deploy Output
#### Input
```bash
curl -X POST -H "Content-Type: application/json" -d '{"message": "I love you, Nana", "name": "Hachiko" }' <function-url>
```
#### Output
```json
{
  "response": "Hello, Hachiko! Your message is: I love you, Nana"
}
```

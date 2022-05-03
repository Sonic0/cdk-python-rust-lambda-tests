A simple example with CDK V2 Python and Rust.

## Install & Deploy
Install AWS CDK CLI
```bash
npm install -g aws-cdk
```
Install pip dependencies
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
### AWS Console (Test event)
#### Input
```json
{
  "message": "vagabond",
  "firstName": "Hachiko"
}
```
#### Output
```json
{
  "response": "Hello, vagabond! Your name is Hachiko"
}
```
### AWS CLI
```bash
aws lambda invoke --function-name 'rust-test' --cli-binary-format raw-in-base64-out --payload '{"message": "CIAO", "firstName": "Andrea"}' out.json
```
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
curl -X POST -H "Content-Type: application/json" -d '{"message": "dude", "firstName": "Andrea" }' <function-url>
```
#### Output
```json
{
  "response": "Hello, dude! Your name is Andrea"
}
```

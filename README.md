A simple example with CDK V2 Python and Rust.

### Install & Deploy
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
./cdk-deploy.sh
```
Deploy Lambda
```bash
cdk deploy LambdaRustTestStack [--profile <local-aws-profile>]
```

### Test Lambda via AWS Console
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

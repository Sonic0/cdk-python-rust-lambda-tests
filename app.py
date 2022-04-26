#!/usr/bin/env python3
import os

import aws_cdk as cdk

from cdk_lambda_rust.lambda_stack import CdkPythonRustLambdaTestsStack

my_env = cdk.Environment(account=os.getenv('CDK_DEFAULT_ACCOUNT'), region=os.getenv('CDK_DEFAULT_REGION'))

app = cdk.App()

CdkPythonRustLambdaTestsStack(
    app, "LambdaRustTestStack",
    env=my_env
)

app.synth()

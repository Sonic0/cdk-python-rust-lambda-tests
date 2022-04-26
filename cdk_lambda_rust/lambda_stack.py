from aws_cdk import (
    Stack,
    Duration,
    RemovalPolicy,
    Environment,
    aws_lambda as lambda_,
    aws_logs as logs
)
from constructs import Construct


class CdkPythonRustLambdaTestsStack(Stack):

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        aws_lambda_code = lambda_.Code.from_asset('cdk_lambda_rust/lambda.zip')

        aws_lambda = lambda_.Function(
            self, 'LambdaRustTest',
            handler="not.required",
            code=aws_lambda_code,
            runtime=lambda_.Runtime.PROVIDED_AL2,
            description='Deploying a Rust function on Lambda using the custom runtime',
            function_name='rust-test',
            log_retention=logs.RetentionDays.ONE_DAY,
            timeout=Duration.seconds(30)
        )
        # aws_lambda.grant_invoke()
        aws_lambda.apply_removal_policy(RemovalPolicy.DESTROY)

        # Set Lambda Evironment variables
        aws_lambda.add_environment("region", self.region)
        # aws_lambda.add_environment("debug", str(self._resource_config.debug))

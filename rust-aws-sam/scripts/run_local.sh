unzip -o \
    target/lambda/release/rust_lambda.zip \
    -d /tmp/lambda && \
nerdctl run \
    -i -e DOCKER_LAMBDA_USE_STDIN=1 \
    --rm \
    -v /tmp/lambda:/var/task \
    lambci/lambda:provided

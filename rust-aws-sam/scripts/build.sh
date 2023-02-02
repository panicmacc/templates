LAMBDA_ARCH="linux/x86_64"
RUST_TARGET="x86_64-unknown-linux-gnu"
RUST_VERSION="latest"
PROJECT_NAME="rust-sam"

al2build() {
	docker run --rm \
	  --platform ${LAMBDA_ARCH} \
		--user "$(id -u)":"$(id -g)" \
		-v "${PWD}":/usr/src/myapp \
		-w /usr/src/myapp \
		rust:${RUST_VERSION} \
		cargo build --release --target ${RUST_TARGET}
}

zipRustLambda() {
	mkdir -p ./build
	cp ./target/${RUST_TARGET}/release/${PROJECT_NAME} \
		./build/bootstrap 
		#&& zip lambda.zip bootstrap \
		#&& rm bootstrap
}

deploySamApp() {
	if [ -f "./samconfig.toml" ]; then
		sam deploy
	else
		sam deploy --guided
	fi
}

set -e
al2build
zipRustLambda
deploySamApp

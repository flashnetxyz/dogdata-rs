BUILD_PATH = "target"

.PHONY: coverage-codecov
coverage-codecov:
	SQLX_OFFLINE=true cargo llvm-cov nextest
	SQLX_OFFLINE=true cargo llvm-cov --workspace --codecov --output-path ./codecov.json
	SQLX_OFFLINE=true cargo llvm-cov --workspace --cobertura --output-path ./cobertura.xml

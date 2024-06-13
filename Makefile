.PHONY: test

test:
	@for version in 3.8 3.9 3.10 3.11 3.12; do \
		LD_LIBRARY_PATH=$$HOME/.rye/py/cpython@3.8.18/lib:$$HOME/.rye/py/cpython@3.9.18/lib:$$HOME/.rye/py/cpython@3.10.13/lib:$$HOME/.rye/py/cpython@3.11.8/lib:$$HOME/.rye/py/cpython@3.12.2/lib:$$LD_LIBRARY_PATH \
		RUSTFLAGS="-L $$HOME/.rye/py/cpython@3.8.18/lib -L $$HOME/.rye/py/cpython@3.9.18/lib -L $$HOME/.rye/py/cpython@3.10.13/lib -L $$HOME/.rye/py/cpython@3.11.8/lib -L $$HOME/.rye/py/cpython@3.12.2/lib" \
		PYO3_CROSS_PYTHON_VERSION=$$version \
		cargo test -r || exit 1; \
	done

fetch:
	@for version in 3.9 3.10 3.11 3.12; do \
		echo "Fetching Python $$version"; \
		rye toolchain fetch $$version || exit 1; \
	done
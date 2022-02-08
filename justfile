
# Run the application.
run *args:
    cargo run -- {{args}}

# Run a debug build.
build:
    cargo build

# Run a release build.
release:
    cargo build --release

# Clean up build artifacts.
clean:
    cargo clean

# Format the project.
format:
    cargo fix
    cargo fmt

# Run tests.
test:
    cargo test

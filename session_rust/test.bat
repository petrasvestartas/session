@echo off

REM Change directory to the directory containing this script
cd %~dp0

echo Formatting Rust code...
cargo fmt --all

echo Running clippy...
cargo clippy --all-targets --all-features --release -- -D warnings
if %errorlevel% neq 0 (
    echo Clippy failed! Fix warnings above.
    exit /b %errorlevel%
)

echo Running Rust tests...
cargo test --release

echo Running demo binary (release)...
cargo run --release

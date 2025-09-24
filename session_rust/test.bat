@echo off

REM Change directory to the directory containing this script
cd %~dp0

echo Formatting Rust code...
cargo fmt --all

echo Running Rust tests...
cargo test

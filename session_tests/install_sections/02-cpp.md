# C++

## Install & download

**Windows** — [CMake](https://cmake.org/download/) 3.20+ and
[Visual Studio](https://visualstudio.microsoft.com/downloads/) 2019+ (Desktop C++):

```bash
winget install Kitware.CMake Microsoft.VisualStudio.2022.BuildTools
```

**macOS:**

```bash
xcode-select --install
brew install cmake
```

**Linux:**

```bash
sudo apt install -y build-essential cmake
```

## Build & run tests

**Windows:**

```bash
cd session_cpp
cmake -B build
cmake --build build --config Release
.\build\Release\point_minitest.exe
```

**macOS / Linux:**

```bash
cd session_cpp
cmake -B build
cmake --build build -j
./build/point_minitest
```

<template>
  <div class="install-view">
    <main class="install-main">
      <table>
        <thead>
          <tr>
            <th>
              <a href="https://github.com/petrasvestartas/session_cpp" target="_blank" class="lang-link">
                <svg class="lang-icon" viewBox="0 0 24 24" fill="currentColor" title="C++">
                  <path d="M20.66 7a1.51 1.51 0 0 0-.55-.57l-7.34-4.24a1.67 1.67 0 0 0-1.54 0L3.89 6.43a1.51 1.51 0 0 0-.55.57 1.6 1.6 0 0 0-.22.76v8.48a1.6 1.6 0 0 0 .22.76 1.51 1.51 0 0 0 .55.57l7.34 4.24a1.67 1.67 0 0 0 1.54 0l7.34-4.24a1.51 1.51 0 0 0 .55-.57 1.6 1.6 0 0 0 .22-.76V7.76a1.6 1.6 0 0 0-.22-.76zM12 17.92A5.92 5.92 0 1 1 17.13 9L16 9.71l-.36.2-1 .61A3 3 0 0 0 9 12a2.88 2.88 0 0 0 .4 1.48 3 3 0 0 0 5.13 0l2.6 1.52A5.94 5.94 0 0 1 12 17.92zm5.92-5.59h-.66V13h-.65v-.66H16v-.66h.66V11h.65v.66h.66zm2.47 0h-.66V13h-.66v-.66h-.65v-.66h.65V11h.66v.66h.66z"/>
                </svg>
              </a>
            </th>
            <th>
              <a href="https://github.com/petrasvestartas/session_py" target="_blank" class="lang-link">
                <i class="fa-brands fa-python lang-icon" title="Python"></i>
              </a>
            </th>
            <th>
              <a href="https://github.com/petrasvestartas/session_rust" target="_blank" class="lang-link">
                <i class="fa-brands fa-rust lang-icon" title="Rust"></i>
              </a>
            </th>
          </tr>
        </thead>
        <tbody>
          <!-- Windows Section -->
          <tr class="row-header">
            <td colspan="3"><strong>Windows</strong></td>
          </tr>
          <tr>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Prerequisites:</strong></p>
                <ul>
                  <li><a href="https://cmake.org/download/" target="_blank">CMake</a> 3.20+</li>
                  <li><a href="https://visualstudio.microsoft.com/downloads/" target="_blank">Visual Studio</a> 2019+</li>
                </ul>
                <p><strong>Build:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_cpp\nmkdir build\ncd build\ncmake ..\ncmake --build . --config Release')" title="Copy code"></button>
                  <pre><code>cd session_cpp
mkdir build
cd build
cmake ..
cmake --build . --config Release</code></pre>
                </div>
                <p><strong>Run tests:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('build\\Release\\tests.exe')" title="Copy code"></button>
                  <pre><code>build\Release\tests.exe</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Register the demo target in <code>session_cpp\CMakeLists.txt</code>:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('# Temp demo executable (user example)\nadd_executable(temp_demo temp/demo.cpp)\ntarget_link_libraries(temp_demo PRIVATE session_core)\nset_target_properties(temp_demo PROPERTIES\n    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/temp\n    RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/temp\n)')" title="Copy code"></button>
                  <pre><code># Temp demo executable (user example)
add_executable(temp_demo temp/demo.cpp)
target_link_libraries(temp_demo PRIVATE session_core)
set_target_properties(temp_demo PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/temp
    RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/temp
)</code></pre>
                </div>
                <p>Create <code>session_cpp\temp\demo.cpp</code>:</p>
                <div class="code-shell">
                  <pre><code>#include "point.h"
#include &lt;iostream&gt;

int main() {
    session_cpp::Point p(1.0, 2.0, 3.0);
    std::cout &lt;&lt; p[0] &lt;&lt; ", " &lt;&lt; p[1] &lt;&lt; ", " &lt;&lt; p[2] &lt;&lt; "\n";
    return 0;
}</code></pre>
                </div>
                <p>Build and run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_cpp\\build\ncmake --build . --config Release --target temp_demo\n.\\temp\\temp_demo.exe')" title="Copy code"></button>
                  <pre><code>cd session_cpp\build
cmake --build . --config Release --target temp_demo
.\temp\temp_demo.exe</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Prerequisites:</strong></p>
                <ul>
                  <li><a href="https://www.python.org/downloads/" target="_blank">Python</a> 3.10+</li>
                </ul>
                <p><strong>Install uv:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('pip install uv')" title="Copy code"></button>
                  <pre><code>pip install uv</code></pre>
                </div>
                <p><strong>Create environment:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd path\\to\\session\nuv venv uvsession')" title="Copy code"></button>
                  <pre><code>cd path\to\session
uv venv uvsession</code></pre>
                </div>
                <p><strong>Activate:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd uvsession\\Scripts\nactivate.bat')" title="Copy code"></button>
                  <pre><code>cd uvsession\Scripts
activate.bat</code></pre>
                </div>
                <p><strong>Install package:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('uv pip install -e session_py')" title="Copy code"></button>
                  <pre><code>uv pip install -e session_py</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Create <code>temp\demo.py</code>:</p>
                <div class="code-shell">
                  <pre><code>from session_py import Point

p = Point(1.0, 2.0, 3.0)
print(repr(p))</code></pre>
                </div>
                <p>Run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd temp\npython demo.py')" title="Copy code"></button>
                  <pre><code>cd temp
python demo.py</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Install:</strong></p>
                <ul>
                  <li>Download <a href="https://rustup.rs/" target="_blank">rustup-init.exe</a></li>
                </ul>
                <p><strong>Build:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_rust\ncargo build --release --features protobuf')" title="Copy code"></button>
                  <pre><code>cd session_rust
cargo build --release --features protobuf</code></pre>
                </div>
                <p><strong>Run tests:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cargo test --release')" title="Copy code"></button>
                  <pre><code>cargo test --release</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Create <code>session_rust\temp\demo.rs</code>:</p>
                <div class="code-shell">
                  <pre><code>use session_rust::Point;

fn main() {
    let p = Point::new(1.0, 2.0, 3.0);
    println!("{p:?}");
}</code></pre>
                </div>
                <p>Run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_rust\ncargo run --bin temp_demo')" title="Copy code"></button>
                  <pre><code>cd session_rust
cargo run --bin temp_demo</code></pre>
                </div>
              </div>
            </td>
          </tr>

          <!-- macOS Section -->
          <tr class="row-header">
            <td colspan="3"><strong>macOS</strong></td>
          </tr>
          <tr>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Prerequisites:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('xcode-select --install\nbrew install cmake')" title="Copy code"></button>
                  <pre><code>xcode-select --install
brew install cmake</code></pre>
                </div>
                <p><strong>Build:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_cpp\nmkdir build\ncd build\ncmake ..\nmake -j$(sysctl -n hw.ncpu)')" title="Copy code"></button>
                  <pre><code>cd session_cpp
mkdir build
cd build
cmake ..
make -j$(sysctl -n hw.ncpu)</code></pre>
                </div>
                <p><strong>Run tests:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('./build/tests')" title="Copy code"></button>
                  <pre><code>./build/tests</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Register the demo target in <code>session_cpp/CMakeLists.txt</code>:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('# Temp demo executable (user example)\nadd_executable(temp_demo temp/demo.cpp)\ntarget_link_libraries(temp_demo PRIVATE session_core)\nset_target_properties(temp_demo PROPERTIES\n    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/temp\n    RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/temp\n)')" title="Copy code"></button>
                  <pre><code># Temp demo executable (user example)
add_executable(temp_demo temp/demo.cpp)
target_link_libraries(temp_demo PRIVATE session_core)
set_target_properties(temp_demo PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/temp
    RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/temp
)</code></pre>
                </div>
                <p>Create <code>session_cpp/temp/demo.cpp</code>:</p>
                <div class="code-shell">
                  <pre><code>#include "point.h"
#include &lt;iostream&gt;

int main() {
    session_cpp::Point p(1.0, 2.0, 3.0);
    std::cout &lt;&lt; p[0] &lt;&lt; ", " &lt;&lt; p[1] &lt;&lt; ", " &lt;&lt; p[2] &lt;&lt; "\n";
    return 0;
}</code></pre>
                </div>
                <p>Build and run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_cpp/build\ncmake --build . --target temp_demo\n./temp/temp_demo')" title="Copy code"></button>
                  <pre><code>cd session_cpp/build
cmake --build . --target temp_demo
./temp/temp_demo</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Prerequisites:</strong></p>
                <ul>
                  <li><a href="https://www.python.org/downloads/" target="_blank">Python</a> 3.10+</li>
                </ul>
                <p><strong>Install uv:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('curl -LsSf https://astral.sh/uv/install.sh | sh')" title="Copy code"></button>
                  <pre><code>curl -LsSf https://astral.sh/uv/install.sh | sh</code></pre>
                </div>
                <p><strong>Create environment:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd path/to/session\nuv venv uvsession')" title="Copy code"></button>
                  <pre><code>cd path/to/session
uv venv uvsession</code></pre>
                </div>
                <p><strong>Activate:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('source uvsession/bin/activate')" title="Copy code"></button>
                  <pre><code>source uvsession/bin/activate</code></pre>
                </div>
                <p><strong>Install package:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('uv pip install -e session_py')" title="Copy code"></button>
                  <pre><code>uv pip install -e session_py</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Create <code>temp/demo.py</code>:</p>
                <div class="code-shell">
                  <pre><code>from session_py import Point

p = Point(1.0, 2.0, 3.0)
print(repr(p))</code></pre>
                </div>
                <p>Run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd temp\npython3 demo.py')" title="Copy code"></button>
                  <pre><code>cd temp
python3 demo.py</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Install:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh')" title="Copy code"></button>
                  <pre><code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
                </div>
                <p><strong>Build:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_rust\ncargo build --release --features protobuf')" title="Copy code"></button>
                  <pre><code>cd session_rust
cargo build --release --features protobuf</code></pre>
                </div>
                <p><strong>Run tests:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cargo test --release')" title="Copy code"></button>
                  <pre><code>cargo test --release</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Create <code>session_rust/temp/demo.rs</code>:</p>
                <div class="code-shell">
                  <pre><code>use session_rust::Point;

fn main() {
    let p = Point::new(1.0, 2.0, 3.0);
    println!("{p:?}");
}</code></pre>
                </div>
                <p>Run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_rust\ncargo run --bin temp_demo')" title="Copy code"></button>
                  <pre><code>cd session_rust
cargo run --bin temp_demo</code></pre>
                </div>
              </div>
            </td>
          </tr>

          <!-- Ubuntu Section -->
          <tr class="row-header">
            <td colspan="3"><strong>Ubuntu</strong></td>
          </tr>
          <tr>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Prerequisites:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('sudo apt update\nsudo apt install build-essential cmake')" title="Copy code"></button>
                  <pre><code>sudo apt update
sudo apt install build-essential cmake</code></pre>
                </div>
                <p><strong>Build:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_cpp\nmkdir build\ncd build\ncmake ..\nmake -j$(nproc)')" title="Copy code"></button>
                  <pre><code>cd session_cpp
mkdir build
cd build
cmake ..
make -j$(nproc)</code></pre>
                </div>
                <p><strong>Run tests:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('./build/tests')" title="Copy code"></button>
                  <pre><code>./build/tests</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Register the demo target in <code>session_cpp/CMakeLists.txt</code>:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('# Temp demo executable (user example)\nadd_executable(temp_demo temp/demo.cpp)\ntarget_link_libraries(temp_demo PRIVATE session_core)\nset_target_properties(temp_demo PROPERTIES\n    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/temp\n    RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/temp\n)')" title="Copy code"></button>
                  <pre><code># Temp demo executable (user example)
add_executable(temp_demo temp/demo.cpp)
target_link_libraries(temp_demo PRIVATE session_core)
set_target_properties(temp_demo PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/temp
    RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}/temp
)</code></pre>
                </div>
                <p>Create <code>session_cpp/temp/demo.cpp</code>:</p>
                <div class="code-shell">
                  <pre><code>#include "point.h"
#include &lt;iostream&gt;

int main() {
    session_cpp::Point p(1.0, 2.0, 3.0);
    std::cout &lt;&lt; p[0] &lt;&lt; ", " &lt;&lt; p[1] &lt;&lt; ", " &lt;&lt; p[2] &lt;&lt; "\n";
    return 0;
}</code></pre>
                </div>
                <p>Build and run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_cpp/build\ncmake --build . --target temp_demo\n./temp/temp_demo')" title="Copy code"></button>
                  <pre><code>cd session_cpp/build
cmake --build . --target temp_demo
./temp/temp_demo</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Prerequisites:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('sudo apt install python3 python3-pip python3-venv')" title="Copy code"></button>
                  <pre><code>sudo apt install python3 python3-pip python3-venv</code></pre>
                </div>
                <p><strong>Install uv:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('curl -LsSf https://astral.sh/uv/install.sh | sh')" title="Copy code"></button>
                  <pre><code>curl -LsSf https://astral.sh/uv/install.sh | sh</code></pre>
                </div>
                <p><strong>Create environment:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd path/to/session\nuv venv uvsession')" title="Copy code"></button>
                  <pre><code>cd path/to/session
uv venv uvsession</code></pre>
                </div>
                <p><strong>Activate:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('source uvsession/bin/activate')" title="Copy code"></button>
                  <pre><code>source uvsession/bin/activate</code></pre>
                </div>
                <p><strong>Install package:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('uv pip install -e session_py')" title="Copy code"></button>
                  <pre><code>uv pip install -e session_py</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Create <code>temp/demo.py</code>:</p>
                <div class="code-shell">
                  <pre><code>from session_py import Point

p = Point(1.0, 2.0, 3.0)
print(repr(p))</code></pre>
                </div>
                <p>Run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd temp\npython3 demo.py')" title="Copy code"></button>
                  <pre><code>cd temp
python3 demo.py</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <p><strong>Install:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh')" title="Copy code"></button>
                  <pre><code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
                </div>
                <p><strong>Build:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_rust\ncargo build --release --features protobuf')" title="Copy code"></button>
                  <pre><code>cd session_rust
cargo build --release --features protobuf</code></pre>
                </div>
                <p><strong>Run tests:</strong></p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cargo test --release')" title="Copy code"></button>
                  <pre><code>cargo test --release</code></pre>
                </div>
                <p><strong>Run example:</strong></p>
                <p>Create <code>session_rust/temp/demo.rs</code>:</p>
                <div class="code-shell">
                  <pre><code>use session_rust::Point;

fn main() {
    let p = Point::new(1.0, 2.0, 3.0);
    println!("{p:?}");
}</code></pre>
                </div>
                <p>Run:</p>
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('cd session_rust\ncargo run --bin temp_demo')" title="Copy code"></button>
                  <pre><code>cd session_rust
cargo run --bin temp_demo</code></pre>
                </div>
              </div>
            </td>
          </tr>

          <!-- Run All Tests Section -->
          <tr class="row-header">
            <td colspan="3"><strong>Run All Tests</strong></td>
          </tr>
          <tr>
            <td class="lang-col">
              <div class="install-card">
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('minitest.bat')" title="Copy code"></button>
                  <pre><code>minitest.bat</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('./minitest.sh')" title="Copy code"></button>
                  <pre><code>./minitest.sh</code></pre>
                </div>
              </div>
            </td>
            <td class="lang-col">
              <div class="install-card">
                <div class="code-shell">
                  <button class="code-copy-btn" type="button" @click="copyCode('./minitest.sh')" title="Copy code"></button>
                  <pre><code>./minitest.sh</code></pre>
                </div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </main>
  </div>
</template>

<script setup>
const copyCode = (text) => {
  try {
    if (typeof navigator !== 'undefined' && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text)
    }
  } catch (e) {
    console.error('Failed to copy code', e)
  }
}
</script>

<style scoped>
/* Install View - Pure black theme */
.install-view {
  padding: 1.5rem 0;
  height: 100%;
  box-sizing: border-box;
  background: #000000;
}

.install-main {
  flex: 1;
  overflow-y: auto;
}

.section-title {
  background: #000000;
  color: #ffffff;
  padding: 0;
  margin: 0 0 0.5rem 0;
  font-size: 14px;
  font-weight: 600;
  border-radius: 0;
  border: none;
  text-align: center;
}

table {
  width: 100%;
  border-collapse: collapse;
  background: #000000;
  box-shadow: none;
  table-layout: fixed;
}

th, td {
  padding: 0.5rem 0.75rem;
  border: none;
  vertical-align: top;
  color: #aaaaaa;
}

th {
  background: #000000;
  text-align: left;
  font-weight: 600;
  font-size: 14px;
  color: #ffffff;
  border: none;
}

.lang-icon {
  width: 24px;
  height: 24px;
  font-size: 24px;
  color: #ffffff;
}

.lang-link {
  color: #ffffff;
  text-decoration: none;
  transition: color 0.2s;
}

.lang-link:hover {
  color: #aaaaaa;
}

.lang-text {
  font-size: 24px;
  font-weight: 700;
  color: #ffffff;
}

.lang-col {
  width: 33.33%;
}

.row-header td {
  padding-top: 0.75rem;
  background: #000000;
  color: #ffffff;
}

.install-card {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.install-card p {
  margin: 0;
  font-size: 0.9rem;
  color: #aaaaaa;
}

.install-card p strong {
  color: #ffffff;
}

.install-card ul {
  margin: 0;
  padding-left: 1.25rem;
}

.install-card li {
  margin: 0.15rem 0;
  font-size: 0.9rem;
  color: #aaaaaa;
}

.install-card a {
  color: #5588ff;
  text-decoration: none;
}

.install-card a:hover {
  text-decoration: underline;
  color: #77aaff;
}

.os-section {
  padding: 0.75rem 0;
  border: none;
}

.os-section:last-child {
  border: none;
}

.os-header {
  background: #000000;
  color: white;
  padding: 0.4rem 0;
  margin: 0 0 0.5rem 0;
  font-size: 0.95rem;
  font-weight: 600;
  border-radius: 0;
}

.code-shell {
  position: relative;
  margin: 0.25rem 0;
  background: #0f0f0f;
  border-radius: 4px;
  border: none;
}

.code-shell pre {
  margin: 0;
  padding: 0.75rem;
  overflow-x: auto;
  background: transparent;
}

.code-shell code {
  color: #5588ff;
  font-family: 'SF Mono', 'Fira Code', Consolas, monospace;
  font-size: 0.8rem;
  line-height: 1.4;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.code-copy-btn {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 12px;
  height: 12px;
  padding: 0;
  border: none;
  border-radius: 50%;
  background: #444444;
  cursor: pointer;
}

.code-copy-btn:hover {
  background: #666666;
}

code {
  background: #000000;
  padding: 0.1rem 0.3rem;
  border-radius: 0;
  font-family: 'SF Mono', 'Fira Code', Consolas, monospace;
  font-size: 0.85em;
  color: #888888;
}

.code-shell code {
  background: none;
  padding: 0;
  color: #5588ff;
}
</style>

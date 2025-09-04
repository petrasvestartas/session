# session_py

## Compilation
**Windows**

```bash
cd C:\brg\2_code\statics
mkdir build
cd build
cmake -G "Visual Studio 17 2022" ..
cmake --build . --config Release
.\Release\MyProject.exe
```

**Mac, Ubuntu** 
```bash
cd /path/to/brg/2_code/statics
mkdir build
cd build
cmake ..
cmake --build . --config Release
./MyProject
```

If a new Ubuntu installation:

```bash
chmod +x /home/petras/brg/2_code/statics/build.sh
sudo apt install cmake
/home/petras/brg/2_code/statics/build.sh
```


# Cross-Language Geometry Validation System

This validation system ensures API and behavioral consistency across Python, Rust, and C++ geometry implementations.

## Overview

The validator checks two types of consistency:
1. **API Consistency** - Ensures all languages implement the same methods and interfaces
2. **Behavioral Consistency** - Ensures all languages produce identical numerical results

## Files

- `validator.py` - Main validation script
- `validation_config.json` - Configurable list of files to validate
- `geometry_spec.json` - API specification schema (in parent directory)

## Configuration

Edit `validation_config.json` to specify which files to validate:

```json
{
  "python_files": [
    "../session_py/src/point.py",
    "../session_py/src/vector.py"
  ],
  "rust_files": [
    "../session_rust/src/point.rs",
    "../session_rust/src/vector.rs"
  ],
  "cpp_files": [
    "../session_cpp/src/point.hpp",
    "../session_cpp/src/vector.hpp"
  ],
  "schema_file": "../geometry_spec.json"
}
```

## Usage

### Basic Validation
```bash
# Run full validation on configured files
python3 validator.py validate
```

### Managing File Lists
```bash
# Add files to validation
python3 validator.py add python ../session_py/src/new_file.py
python3 validator.py add rust ../session_rust/src/new_file.rs
python3 validator.py add cpp ../session_cpp/src/new_file.hpp

# Remove files from validation
python3 validator.py remove python ../session_py/src/old_file.py
```

## How It Works

### API Consistency Validation

1. **Python Files**: Uses Python's `ast` module to parse source files and extract class definitions and method signatures
2. **Rust Files**: Uses regex patterns to find `struct` definitions and `impl` blocks with public methods
3. **C++ Files**: Uses regex patterns to find `class` definitions and public method declarations

Each file is checked against the `geometry_spec.json` schema to ensure:
- All required types are present
- All required methods are implemented
- Method signatures match the specification

### Behavioral Consistency Testing

1. **Test Generation**: Creates temporary test programs for each language
2. **Compilation & Execution**: Compiles and runs tests in isolated environments
3. **Result Comparison**: Compares numerical outputs with floating-point tolerance
4. **Consistency Report**: Reports any discrepancies between language implementations

### Example Test Case

```python
{
  "name": "point_distance",
  "test": {
    "p1": {"x": 0.0, "y": 0.0, "z": 0.0},
    "p2": {"x": 3.0, "y": 4.0, "z": 0.0},
    "expected": 5.0
  }
}
```

This test creates two points and verifies that `distance_to()` returns `5.0` in all three languages.

## Validation Output

The validator provides detailed feedback:

```
🔍 Starting cross-language validation...

📋 API Consistency Check:
  Checking Python: ../session_py/src/point.py
  ✅ Point class found with all required methods
  
  Checking Rust: ../session_rust/src/point.rs
  ❌ Missing methods: [normalize, scale]
  
  Checking C++: ../session_cpp/src/point.hpp
  ✅ Point class found with all required methods

🧪 Behavioral Consistency Check:
  Testing point_distance...
  ✅ All implementations match (result: 5.0)

📊 Validation Summary:
  Files checked: 3
  Total errors: 2
  ❌ Issues found - see details above
```

## Incremental Development Workflow

1. **Start Small**: Begin with just one geometry type (e.g., Point)
2. **Configure**: Update `validation_config.json` to include only existing files
3. **Implement**: Create the geometry type in one language
4. **Validate**: Run `python3 validator.py validate`
5. **Fix Issues**: Address any API inconsistencies
6. **Add Language**: Implement the same type in another language
7. **Validate Again**: Ensure cross-language consistency
8. **Expand**: Add more geometry types and repeat

## Schema-Driven Development

The `geometry_spec.json` file defines the contract that all implementations must follow:
- **Method Names**: Must be identical across languages
- **Parameter Types**: Must be semantically equivalent
- **Return Types**: Must be semantically equivalent
- **Behavior**: Must produce identical numerical results

This ensures that your geometry kernel maintains perfect consistency as you develop across all three languages.

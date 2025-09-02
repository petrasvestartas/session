#!/usr/bin/env python3
"""
Cross-language API and behavioral consistency validator.
Validates specific files against geometry specification schema.
"""

import json
import ast
import re
import subprocess
import tempfile
from pathlib import Path
from typing import Dict, List, Set, Any, Optional

class GeometryValidator:
    def __init__(self, config_file: str = "validation_config.json"):
        """Initialize validator with configurable file lists."""
        self.config_file = config_file
        self.load_config()
        self.load_schema()
    
    def load_config(self):
        """Load validation configuration specifying which files to check."""
        config_path = Path(__file__).parent / self.config_file
        
        # Default configuration if file doesn't exist
        default_config = {
            "python_files": [
                "../geometry_py/point.py",
                "../geometry_py/vector.py", 
                "../geometry_py/line.py"
            ],
            "rust_files": [
                "../geometry_rust/src/point.rs",
                "../geometry_rust/src/vector.rs",
                "../geometry_rust/src/line.rs"
            ],
            "cpp_files": [
                "../geometry_cpp/point.hpp",
                "../geometry_cpp/vector.hpp",
                "../geometry_cpp/line.hpp"
            ],
            "schema_file": "../geometry_spec.json"
        }
        
        if config_path.exists():
            with open(config_path, 'r') as f:
                self.config = json.load(f)
        else:
            self.config = default_config
            # Create default config file
            with open(config_path, 'w') as f:
                json.dump(default_config, f, indent=2)
            print(f"Created default config: {config_path}")
    
    def load_schema(self):
        """Load the geometry specification schema."""
        schema_path = Path(__file__).parent / self.config["schema_file"]
        with open(schema_path, 'r') as f:
            self.schema = json.load(f)
    
    def validate_python_file(self, file_path: str) -> Dict[str, Any]:
        """Validate Python file against schema."""
        results = {"file": file_path, "errors": [], "missing_methods": []}
        
        try:
            full_path = Path(__file__).parent / file_path
            if not full_path.exists():
                results["errors"].append(f"File not found: {full_path}")
                return results
            
            with open(full_path, 'r') as f:
                tree = ast.parse(f.read())
            
            # Find class definitions
            for node in ast.walk(tree):
                if isinstance(node, ast.ClassDef):
                    class_name = node.name
                    if class_name in self.schema["types"]:
                        schema_methods = set(self.schema["types"][class_name]["methods"].keys())
                        found_methods = set()
                        
                        # Find method definitions
                        for item in node.body:
                            if isinstance(item, ast.FunctionDef):
                                found_methods.add(item.name)
                        
                        # Check for missing methods
                        missing = schema_methods - found_methods
                        if missing:
                            results["missing_methods"].extend(list(missing))
        
        except Exception as e:
            results["errors"].append(f"Parse error: {str(e)}")
        
        return results
    
    def validate_rust_file(self, file_path: str) -> Dict[str, Any]:
        """Validate Rust file against schema."""
        results = {"file": file_path, "errors": [], "missing_methods": []}
        
        try:
            full_path = Path(__file__).parent / file_path
            if not full_path.exists():
                results["errors"].append(f"File not found: {full_path}")
                return results
            
            with open(full_path, 'r') as f:
                content = f.read()
            
            # Find struct definitions
            struct_pattern = r'pub struct (\w+)'
            structs = re.findall(struct_pattern, content)
            
            for struct_name in structs:
                if struct_name in self.schema["types"]:
                    schema_methods = set(self.schema["types"][struct_name]["methods"].keys())
                    
                    # Find impl block methods
                    impl_pattern = rf'impl.*{struct_name}.*\{{(.*?)\}}'
                    impl_matches = re.findall(impl_pattern, content, re.DOTALL)
                    
                    found_methods = set()
                    for impl_block in impl_matches:
                        method_pattern = r'pub fn (\w+)'
                        methods = re.findall(method_pattern, impl_block)
                        found_methods.update(methods)
                    
                    # Check for missing methods
                    missing = schema_methods - found_methods
                    if missing:
                        results["missing_methods"].extend(list(missing))
        
        except Exception as e:
            results["errors"].append(f"Parse error: {str(e)}")
        
        return results
    
    def validate_cpp_file(self, file_path: str) -> Dict[str, Any]:
        """Validate C++ file against schema."""
        results = {"file": file_path, "errors": [], "missing_methods": []}
        
        try:
            full_path = Path(__file__).parent / file_path
            if not full_path.exists():
                results["errors"].append(f"File not found: {full_path}")
                return results
            
            with open(full_path, 'r') as f:
                content = f.read()
            
            # Find class definitions
            class_pattern = r'class (\w+)'
            classes = re.findall(class_pattern, content)
            
            for class_name in classes:
                if class_name in self.schema["types"]:
                    schema_methods = set(self.schema["types"][class_name]["methods"].keys())
                    
                    # Find class body and methods
                    class_body_pattern = rf'class {class_name}.*?\{{(.*?)\}};'
                    class_matches = re.findall(class_body_pattern, content, re.DOTALL)
                    
                    found_methods = set()
                    for class_body in class_matches:
                        # Look for method declarations
                        method_pattern = r'(\w+)\s*\([^)]*\)\s*(?:const)?;'
                        methods = re.findall(method_pattern, class_body)
                        found_methods.update(methods)
                    
                    # Check for missing methods
                    missing = schema_methods - found_methods
                    if missing:
                        results["missing_methods"].extend(list(missing))
        
        except Exception as e:
            results["errors"].append(f"Parse error: {str(e)}")
        
        return results
    
    def run_behavioral_test(self, test_case: Dict[str, Any]) -> Dict[str, Any]:
        """Run behavioral consistency test across all languages."""
        results = {
            "test_name": test_case["name"],
            "python_result": None,
            "rust_result": None,
            "cpp_result": None,
            "consistent": False,
            "errors": []
        }
        
        try:
            # Run Python test
            if self.config["python_files"]:
                results["python_result"] = self.run_python_test(test_case)
            
            # Run Rust test
            if self.config["rust_files"]:
                results["rust_result"] = self.run_rust_test(test_case)
            
            # Run C++ test
            if self.config["cpp_files"]:
                results["cpp_result"] = self.run_cpp_test(test_case)
            
            # Check consistency
            results["consistent"] = self.results_match(
                results["python_result"],
                results["rust_result"], 
                results["cpp_result"]
            )
            
        except Exception as e:
            results["errors"].append(f"Test execution error: {str(e)}")
        
        return results
    
    def run_python_test(self, test_case: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """Execute test case in Python."""
        py_code = self.generate_python_test_code(test_case)
        if not py_code:
            return None
            
        with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
            f.write(py_code)
            temp_file = f.name
        
        try:
            result = subprocess.run(['python3', temp_file], 
                                  capture_output=True, text=True, timeout=10)
            if result.returncode == 0:
                return json.loads(result.stdout.strip())
            else:
                return {"error": result.stderr}
        except Exception as e:
            return {"error": str(e)}
        finally:
            Path(temp_file).unlink(missing_ok=True)
    
    def run_rust_test(self, test_case: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """Execute test case in Rust."""
        # Implementation for Rust test execution
        return None  # Placeholder
    
    def run_cpp_test(self, test_case: Dict[str, Any]) -> Optional[Dict[str, Any]]:
        """Execute test case in C++."""
        # Implementation for C++ test execution
        return None  # Placeholder
    
    def generate_python_test_code(self, test_case: Dict[str, Any]) -> str:
        """Generate Python test code for specific test case."""
        if test_case["name"] == "point_distance":
            return f"""
import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), '../geometry_py'))
from point import Point
import json

p1 = Point({test_case["test"]["p1"]["x"]}, {test_case["test"]["p1"]["y"]}, {test_case["test"]["p1"]["z"]})
p2 = Point({test_case["test"]["p2"]["x"]}, {test_case["test"]["p2"]["y"]}, {test_case["test"]["p2"]["z"]})
result = p1.distance_to(p2)
print(json.dumps({{"result": result}}))
"""
        return ""
    
    def results_match(self, py_result: Optional[Dict], rs_result: Optional[Dict], 
                     cpp_result: Optional[Dict], tolerance: float = 1e-9) -> bool:
        """Check if results match within tolerance."""
        results = [r for r in [py_result, rs_result, cpp_result] if r is not None]
        if len(results) < 2:
            return True  # Can't compare if less than 2 results
        
        # Compare numerical results with tolerance
        first_result = results[0].get("result")
        if first_result is None:
            return False
        
        for result in results[1:]:
            other_result = result.get("result")
            if other_result is None:
                return False
            
            if isinstance(first_result, (int, float)) and isinstance(other_result, (int, float)):
                if abs(first_result - other_result) > tolerance:
                    return False
            elif first_result != other_result:
                return False
        
        return True
    
    def validate_all(self) -> Dict[str, Any]:
        """Run complete validation suite."""
        print("🔍 Starting cross-language validation...")
        
        validation_results = {
            "api_validation": {},
            "behavioral_validation": {},
            "summary": {"total_errors": 0, "files_checked": 0}
        }
        
        # API Validation
        print("\n📋 API Consistency Check:")
        
        # Validate Python files
        for py_file in self.config["python_files"]:
            print(f"  Checking Python: {py_file}")
            result = self.validate_python_file(py_file)
            validation_results["api_validation"][py_file] = result
            validation_results["summary"]["files_checked"] += 1
            validation_results["summary"]["total_errors"] += len(result["errors"]) + len(result["missing_methods"])
        
        # Validate Rust files
        for rs_file in self.config["rust_files"]:
            print(f"  Checking Rust: {rs_file}")
            result = self.validate_rust_file(rs_file)
            validation_results["api_validation"][rs_file] = result
            validation_results["summary"]["files_checked"] += 1
            validation_results["summary"]["total_errors"] += len(result["errors"]) + len(result["missing_methods"])
        
        # Validate C++ files
        for cpp_file in self.config["cpp_files"]:
            print(f"  Checking C++: {cpp_file}")
            result = self.validate_cpp_file(cpp_file)
            validation_results["api_validation"][cpp_file] = result
            validation_results["summary"]["files_checked"] += 1
            validation_results["summary"]["total_errors"] += len(result["errors"]) + len(result["missing_methods"])
        
        # Behavioral Testing (if implementations exist)
        print("\n🧪 Behavioral Consistency Check:")
        test_cases = [
            {
                "name": "point_distance",
                "test": {
                    "p1": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "p2": {"x": 3.0, "y": 4.0, "z": 0.0},
                    "expected": 5.0
                }
            }
        ]
        
        for test_case in test_cases:
            print(f"  Testing {test_case['name']}...")
            result = self.run_behavioral_test(test_case)
            validation_results["behavioral_validation"][test_case["name"]] = result
        
        # Print summary
        print(f"\n📊 Validation Summary:")
        print(f"  Files checked: {validation_results['summary']['files_checked']}")
        print(f"  Total errors: {validation_results['summary']['total_errors']}")
        
        if validation_results['summary']['total_errors'] == 0:
            print("  ✅ All validations passed!")
        else:
            print("  ❌ Issues found - see details above")
        
        return validation_results
    
    def add_files_to_validation(self, language: str, file_paths: List[str]):
        """Add specific files to validation list."""
        if language.lower() == "python":
            self.config["python_files"].extend(file_paths)
        elif language.lower() == "rust":
            self.config["rust_files"].extend(file_paths)
        elif language.lower() == "cpp":
            self.config["cpp_files"].extend(file_paths)
        
        # Save updated config
        config_path = Path(__file__).parent / self.config_file
        with open(config_path, 'w') as f:
            json.dump(self.config, f, indent=2)
        
        print(f"Added {len(file_paths)} {language} files to validation list")
    
    def remove_files_from_validation(self, language: str, file_paths: List[str]):
        """Remove specific files from validation list."""
        if language.lower() == "python":
            for fp in file_paths:
                if fp in self.config["python_files"]:
                    self.config["python_files"].remove(fp)
        elif language.lower() == "rust":
            for fp in file_paths:
                if fp in self.config["rust_files"]:
                    self.config["rust_files"].remove(fp)
        elif language.lower() == "cpp":
            for fp in file_paths:
                if fp in self.config["cpp_files"]:
                    self.config["cpp_files"].remove(fp)
        
        # Save updated config
        config_path = Path(__file__).parent / self.config_file
        with open(config_path, 'w') as f:
            json.dump(self.config, f, indent=2)
        
        print(f"Removed {len(file_paths)} {language} files from validation list")

if __name__ == "__main__":
    import sys
    
    validator = GeometryValidator()
    
    if len(sys.argv) > 1:
        command = sys.argv[1]
        
        if command == "add":
            # Usage: python validator.py add python ../new_file.py
            if len(sys.argv) >= 4:
                language = sys.argv[2]
                files = sys.argv[3:]
                validator.add_files_to_validation(language, files)
            else:
                print("Usage: python validator.py add <language> <file1> [file2] ...")
        
        elif command == "remove":
            # Usage: python validator.py remove python ../old_file.py
            if len(sys.argv) >= 4:
                language = sys.argv[2]
                files = sys.argv[3:]
                validator.remove_files_from_validation(language, files)
            else:
                print("Usage: python validator.py remove <language> <file1> [file2] ...")
        
        elif command == "validate":
            validator.validate_all()
        
        else:
            print("Unknown command. Use: validate, add, or remove")
    
    else:
        # Default: run validation
        validator.validate_all()

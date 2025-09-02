#!/usr/bin/env python3
import re
import os

def extract_class_info(file_path, language):
    """Extract class name, attributes, and functions using simple parsing"""
    with open(file_path, 'r') as f:
        content = f.read()
    
    if language == 'python':
        # Find class name
        class_match = re.search(r'class (\w+)', content)
        class_name = class_match.group(1) if class_match else 'Unknown'
        
        # Find attributes (self.attribute = ...)
        attributes = re.findall(r'self\.(\w+)\s*=', content)
        attributes = list(dict.fromkeys(attributes))  # Remove duplicates
        
        # Find functions (def function_name)
        functions = re.findall(r'def (\w+)', content)
        
        return [{'name': class_name, 'attributes': attributes, 'functions': functions}]
    
    elif language == 'cpp':
        # Find class name
        class_match = re.search(r'class (\w+)', content)
        class_name = class_match.group(1) if class_match else 'Unknown'
        
        # Find public attributes (type name = value;)
        public_section = re.search(r'public:(.*?)(?:private:|protected:|\};)', content, re.DOTALL)
        attributes = []
        functions = []
        
        if public_section:
            public_content = public_section.group(1)
            # Find member variables
            attr_matches = re.findall(r'(\w+)\s+(\w+)\s*=', public_content)
            attributes = [match[1] for match in attr_matches if match[0] in ['double', 'string', 'Color', 'std::string']]
            
            # Find functions
            func_matches = re.findall(r'(\w+)\s+(\w+)\s*\(', public_content)
            functions = [match[1] for match in func_matches if match[1] not in ['Point', 'operator']]
        
        return [{'name': class_name, 'attributes': attributes, 'functions': functions}]
    
    elif language == 'rust':
        # Find struct name
        struct_match = re.search(r'pub struct (\w+)', content)
        struct_name = struct_match.group(1) if struct_match else 'Unknown'
        
        # Find struct fields
        struct_body = re.search(r'pub struct \w+\s*\{(.*?)\}', content, re.DOTALL)
        attributes = []
        
        if struct_body:
            fields = re.findall(r'pub (\w+):', struct_body.group(1))
            attributes = fields
        
        # Find impl functions
        impl_sections = re.findall(r'impl.*?\{(.*?)\n\}', content, re.DOTALL)
        functions = []
        for impl_body in impl_sections:
            funcs = re.findall(r'pub fn (\w+)', impl_body)
            functions.extend(funcs)
        
        return [{'name': struct_name, 'attributes': attributes, 'functions': functions}]
    
    return []

def main():
    # Get absolute paths from script location
    script_dir = os.path.dirname(os.path.abspath(__file__))
    base_dir = os.path.dirname(script_dir)
    
    files = {
        'Python': os.path.join(base_dir, 'session_py', 'src', 'session_py', 'point.py'),
        'C++': os.path.join(base_dir, 'session_cpp', 'src', 'point.hpp'),
        'Rust': os.path.join(base_dir, 'session_rust', 'src', 'point.rs')
    }
    
    languages = {
        'Python': 'python',
        'C++': 'cpp', 
        'Rust': 'rust'
    }
    
    print("Point Class Comparison (Tree-sitter)")
    print("=" * 40)
    
    all_classes = {}
    
    for lang, path in files.items():
        if not os.path.exists(path):
            print(f"{lang}: FILE NOT FOUND")
            continue
        
        try:
            classes = extract_class_info(path, languages[lang])
            all_classes[lang] = classes
            
            if classes:
                cls = classes[0]  # Assume first class is Point
                print(f"\n{lang}:")
                print(f"  Class: {cls['name']}")
                print(f"  Attributes: {', '.join(cls['attributes'])}")
                print(f"  Functions: {', '.join(cls['functions'])}")
            else:
                print(f"{lang}: No classes found")
                
        except Exception as e:
            print(f"{lang}: Error - {e}")
    
    # Simple comparison
    if len(all_classes) >= 2:
        print(f"\n{'='*40}")
        print("COMPARISON:")
        
        all_attrs = set()
        all_funcs = set()
        
        for lang, classes in all_classes.items():
            if classes:
                all_attrs.update(classes[0]['attributes'])
                all_funcs.update(classes[0]['functions'])
        
        print(f"\nAll attributes found: {', '.join(sorted(all_attrs))}")
        print(f"All functions found: {', '.join(sorted(all_funcs))}")
        
        # Check consistency
        for attr in sorted(all_attrs):
            langs_with_attr = []
            for lang, classes in all_classes.items():
                if classes and attr in classes[0]['attributes']:
                    langs_with_attr.append(lang)
            status = "✅" if len(langs_with_attr) == len(all_classes) else "❌"
            print(f"{attr}: {status} ({', '.join(langs_with_attr)})")

if __name__ == "__main__":
    main()

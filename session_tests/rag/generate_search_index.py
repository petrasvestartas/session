#!/usr/bin/env python3
"""
Generate a static search index with pre-computed embeddings for client-side
semantic code search. This creates a searchIndex.js file that can be loaded
in the browser without requiring a backend server.

Usage:
    python3 generate_search_index.py [--no-embeddings]

Output:
    ../public/searchIndex.js

The embeddings are computed using sentence-transformers (all-MiniLM-L6-v2)
and stored as arrays in the JSON. Client-side search uses cosine similarity.
"""

import os
import re
import sys
import json
import argparse
from pathlib import Path
from typing import List, Dict, Optional
import numpy as np

# Try to import sentence-transformers for embeddings
try:
    from sentence_transformers import SentenceTransformer
    HAS_EMBEDDINGS = True
except ImportError:
    HAS_EMBEDDINGS = False
    print("[SearchIndex] Warning: sentence-transformers not installed")
    print("[SearchIndex] Install with: pip install sentence-transformers")
    print("[SearchIndex] Falling back to keyword-only search")


class CodeChunker:
    """Chunks code files by semantic units (classes, functions, methods)"""

    @staticmethod
    def chunk_python(content: str, file_path: str) -> List[Dict]:
        """Chunk Python code by class and function definitions"""
        chunks = []
        lines = content.split('\n')

        current_chunk = None
        current_lines = []
        chunk_start_line = 0

        for i, line in enumerate(lines):
            # Check for class definition
            class_match = re.match(r'^class\s+(\w+)', line)
            if class_match:
                # Save previous chunk
                if current_chunk and current_lines:
                    current_chunk['code'] = '\n'.join(current_lines)
                    current_chunk['line_end'] = i
                    chunks.append(current_chunk)

                current_chunk = {
                    'type': 'class',
                    'name': class_match.group(1),
                    'file': os.path.basename(file_path),
                    'file_path': file_path,
                    'language': 'python',
                    'line_start': i + 1
                }
                current_lines = [line]
                chunk_start_line = i
                continue

            # Check for top-level function
            func_match = re.match(r'^def\s+(\w+)\s*\(', line)
            if func_match:
                if current_chunk and current_lines:
                    current_chunk['code'] = '\n'.join(current_lines)
                    current_chunk['line_end'] = i
                    chunks.append(current_chunk)

                current_chunk = {
                    'type': 'function',
                    'name': func_match.group(1),
                    'file': os.path.basename(file_path),
                    'file_path': file_path,
                    'language': 'python',
                    'line_start': i + 1
                }
                current_lines = [line]
                chunk_start_line = i
                continue

            # Check for method inside class (indented def)
            method_match = re.match(r'^    def\s+(\w+)\s*\(', line)
            if method_match and current_chunk and current_chunk['type'] == 'class':
                # Add method as separate chunk but keep adding to class
                method_chunk = {
                    'type': 'method',
                    'name': f"{current_chunk['name']}.{method_match.group(1)}",
                    'file': os.path.basename(file_path),
                    'file_path': file_path,
                    'language': 'python',
                    'line_start': i + 1,
                    'parent_class': current_chunk['name']
                }

                # Collect method lines
                method_lines = [line]
                j = i + 1
                while j < len(lines):
                    next_line = lines[j]
                    # Method ends at next def, class, or non-indented line
                    if next_line.strip() and not next_line.startswith('        ') and not next_line.startswith('    '):
                        if re.match(r'^    def\s+', next_line) or re.match(r'^class\s+', next_line) or re.match(r'^def\s+', next_line):
                            break
                    method_lines.append(next_line)
                    j += 1

                method_chunk['code'] = '\n'.join(method_lines[:30])  # Limit to 30 lines
                method_chunk['line_end'] = min(i + len(method_lines), i + 30)
                chunks.append(method_chunk)

            if current_lines:
                current_lines.append(line)

        # Save last chunk
        if current_chunk and current_lines:
            current_chunk['code'] = '\n'.join(current_lines)
            current_chunk['line_end'] = len(lines)
            chunks.append(current_chunk)

        return chunks

    @staticmethod
    def chunk_cpp(content: str, file_path: str) -> List[Dict]:
        """Chunk C++ code by class and function definitions"""
        chunks = []
        lines = content.split('\n')

        # Find class definitions with their bodies
        class_pattern = r'class\s+(\w+)\s*(?::\s*public\s+\w+)?\s*\{'
        for match in re.finditer(class_pattern, content):
            class_name = match.group(1)
            start_pos = match.start()
            line_start = content[:start_pos].count('\n') + 1

            # Find matching closing brace
            brace_count = 1
            i = match.end()
            while i < len(content) and brace_count > 0:
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                i += 1

            class_code = content[start_pos:i]
            line_end = content[:i].count('\n') + 1

            chunks.append({
                'type': 'class',
                'name': class_name,
                'file': os.path.basename(file_path),
                'file_path': file_path,
                'language': 'cpp',
                'code': class_code[:2000],  # Limit size
                'line_start': line_start,
                'line_end': line_end
            })

        # Find standalone function definitions
        func_pattern = r'(?:^|\n)\s*(?:static\s+)?(?:inline\s+)?([a-zA-Z_][\w:]*(?:\s*[*&])?)\s+(\w+)\s*\([^)]*\)\s*(?:const)?\s*\{'
        for match in re.finditer(func_pattern, content):
            return_type = match.group(1)
            func_name = match.group(2)

            # Skip if it's a class method (inside class body) - rough heuristic
            start_pos = match.start()
            line_start = content[:start_pos].count('\n') + 1

            # Find function body
            brace_start = content.find('{', match.end() - 1)
            if brace_start == -1:
                continue

            brace_count = 1
            i = brace_start + 1
            while i < len(content) and brace_count > 0:
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                i += 1

            func_code = content[start_pos:i]
            line_end = content[:i].count('\n') + 1

            chunks.append({
                'type': 'function',
                'name': func_name,
                'file': os.path.basename(file_path),
                'file_path': file_path,
                'language': 'cpp',
                'code': func_code[:1500],
                'line_start': line_start,
                'line_end': line_end
            })

        return chunks

    @staticmethod
    def chunk_rust(content: str, file_path: str) -> List[Dict]:
        """Chunk Rust code by struct, impl blocks, and functions"""
        chunks = []

        # Find struct definitions
        struct_pattern = r'(?:pub\s+)?struct\s+(\w+)\s*(?:<[^>]+>)?\s*\{'
        for match in re.finditer(struct_pattern, content):
            struct_name = match.group(1)
            start_pos = match.start()
            line_start = content[:start_pos].count('\n') + 1

            # Find matching closing brace
            brace_count = 1
            i = match.end()
            while i < len(content) and brace_count > 0:
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                i += 1

            struct_code = content[start_pos:i]
            line_end = content[:i].count('\n') + 1

            chunks.append({
                'type': 'struct',
                'name': struct_name,
                'file': os.path.basename(file_path),
                'file_path': file_path,
                'language': 'rust',
                'code': struct_code[:1000],
                'line_start': line_start,
                'line_end': line_end
            })

        # Find impl blocks
        impl_pattern = r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{'
        for match in re.finditer(impl_pattern, content):
            impl_name = match.group(1)
            start_pos = match.start()
            impl_line_start = content[:start_pos].count('\n') + 1

            # Find matching closing brace for impl block
            brace_count = 1
            i = match.end()
            while i < len(content) and brace_count > 0:
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                i += 1

            impl_body = content[match.end():i-1]

            # Find methods within impl block
            method_pattern = r'(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]+>)?\s*\([^)]*\)'
            for method_match in re.finditer(method_pattern, impl_body):
                method_name = method_match.group(1)
                method_start = method_match.start()
                method_line = impl_body[:method_start].count('\n') + impl_line_start + 1

                # Get some context around the method
                method_end = method_start + 300
                method_code = impl_body[max(0, method_start):min(len(impl_body), method_end)]

                chunks.append({
                    'type': 'method',
                    'name': f"{impl_name}::{method_name}",
                    'file': os.path.basename(file_path),
                    'file_path': file_path,
                    'language': 'rust',
                    'code': method_code,
                    'line_start': method_line,
                    'line_end': method_line + method_code.count('\n')
                })

        # Find standalone functions
        fn_pattern = r'(?:^|\n)(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]+>)?\s*\([^)]*\)'
        for match in re.finditer(fn_pattern, content):
            func_name = match.group(1)
            start_pos = match.start()
            line_start = content[:start_pos].count('\n') + 1

            # Get function body (approximate)
            func_start = content.find('{', match.end())
            if func_start == -1:
                continue

            brace_count = 1
            i = func_start + 1
            while i < len(content) and brace_count > 0:
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                i += 1

            func_code = content[start_pos:i]
            line_end = content[:i].count('\n') + 1

            chunks.append({
                'type': 'function',
                'name': func_name,
                'file': os.path.basename(file_path),
                'file_path': file_path,
                'language': 'rust',
                'code': func_code[:1500],
                'line_start': line_start,
                'line_end': line_end
            })

        return chunks


def extract_keywords(text: str) -> List[str]:
    """Extract searchable keywords from text"""
    # Convert to lowercase and split on non-alphanumeric
    words = re.findall(r'[a-zA-Z_][a-zA-Z0-9_]*', text.lower())

    # Filter out very short words and common programming keywords
    stopwords = {'the', 'and', 'for', 'if', 'else', 'while', 'return', 'def', 'class',
                 'fn', 'let', 'mut', 'pub', 'impl', 'struct', 'self', 'this', 'void',
                 'int', 'float', 'double', 'bool', 'str', 'true', 'false', 'none', 'null'}

    keywords = []
    for word in words:
        if len(word) >= 2 and word not in stopwords:
            keywords.append(word)

    return list(set(keywords))


def generate_search_index(use_embeddings: bool = True):
    """Generate the static search index with optional embeddings"""
    script_dir = Path(__file__).parent
    repo_root = script_dir.parent.parent  # session/
    output_path = script_dir.parent / "public" / "searchIndex.js"

    print("[SearchIndex] Generating static search index...")

    # Initialize embedding model if available and requested
    embedding_model = None
    if use_embeddings and HAS_EMBEDDINGS:
        print("[SearchIndex] Loading embedding model (all-MiniLM-L6-v2)...")
        embedding_model = SentenceTransformer('all-MiniLM-L6-v2')
        print("[SearchIndex] Model loaded")

    chunker = CodeChunker()
    all_chunks = []

    # Files to index
    files_to_index = [
        # Python
        (repo_root / "session_py/src/session_py/point.py", "python"),
        (repo_root / "session_py/src/session_py/color.py", "python"),
        (repo_root / "session_py/src/session_py/point_test.py", "python"),
        (repo_root / "session_py/src/session_py/color_test.py", "python"),

        # C++
        (repo_root / "session_cpp/src/point.h", "cpp"),
        (repo_root / "session_cpp/src/point.cpp", "cpp"),
        (repo_root / "session_cpp/src/color.h", "cpp"),
        (repo_root / "session_cpp/src/color.cpp", "cpp"),
        (repo_root / "session_cpp/src/point_test.cpp", "cpp"),
        (repo_root / "session_cpp/src/color_test.cpp", "cpp"),

        # Rust
        (repo_root / "session_rust/src/point.rs", "rust"),
        (repo_root / "session_rust/src/color.rs", "rust"),
        (repo_root / "session_rust/src/point_test.rs", "rust"),
        (repo_root / "session_rust/src/color_test.rs", "rust"),
    ]

    for file_path, language in files_to_index:
        if not file_path.exists():
            print(f"  Warning: {file_path} not found, skipping")
            continue

        print(f"  Processing: {file_path.name} ({language})")

        content = file_path.read_text(errors='ignore')

        if language == 'python':
            chunks = chunker.chunk_python(content, str(file_path))
        elif language == 'cpp':
            chunks = chunker.chunk_cpp(content, str(file_path))
        elif language == 'rust':
            chunks = chunker.chunk_rust(content, str(file_path))
        else:
            continue

        # Add keywords to each chunk for search
        for chunk in chunks:
            # Create searchable text from name, type, and code
            search_text = f"{chunk['name']} {chunk['type']} {chunk['language']} {chunk.get('code', '')}"
            chunk['keywords'] = extract_keywords(search_text)

            # Add some semantic keywords
            if 'point' in chunk['name'].lower() or 'point' in chunk['file'].lower():
                chunk['keywords'].extend(['point', 'coordinate', 'xyz', 'position', 'location'])
            if 'color' in chunk['name'].lower() or 'color' in chunk['file'].lower():
                chunk['keywords'].extend(['color', 'rgb', 'rgba', 'red', 'green', 'blue', 'alpha'])
            if 'distance' in chunk['name'].lower() or 'distance' in chunk.get('code', '').lower():
                chunk['keywords'].append('distance')
            if 'test' in chunk['name'].lower() or 'test' in chunk['file'].lower():
                chunk['keywords'].append('test')
            if '__init__' in chunk['name'] or 'new' in chunk['name'].lower() or 'constructor' in chunk.get('code', '').lower():
                chunk['keywords'].extend(['create', 'constructor', 'new', 'init', 'initialize'])

            chunk['keywords'] = list(set(chunk['keywords']))

            all_chunks.append(chunk)

    print(f"[SearchIndex] Total chunks indexed: {len(all_chunks)}")

    # Compute embeddings if model is available
    if embedding_model:
        print("[SearchIndex] Computing embeddings for all chunks...")

        # Create document texts for embedding
        doc_texts = []
        for chunk in all_chunks:
            doc_text = f"{chunk['type']} {chunk['name']} in {chunk['language']}\n"
            doc_text += f"File: {chunk['file']}\n"
            doc_text += f"Code:\n{chunk.get('code', '')[:500]}"  # Limit code length
            doc_texts.append(doc_text)

        # Compute embeddings in batch
        embeddings = embedding_model.encode(doc_texts, show_progress_bar=True)

        # Add embeddings to chunks (as list for JSON serialization)
        # Reduce precision to save space (float16 -> round to 4 decimals)
        for i, chunk in enumerate(all_chunks):
            chunk['embedding'] = [round(float(x), 4) for x in embeddings[i]]

        print(f"[SearchIndex] Embeddings computed (dim: {len(embeddings[0])})")

    # Create the index structure
    index = {
        'version': '2.0',
        'hasEmbeddings': embedding_model is not None,
        'embeddingDim': len(all_chunks[0].get('embedding', [])) if embedding_model else 0,
        'generated': str(Path(__file__).stat().st_mtime),
        'chunks': all_chunks,
        'languages': ['python', 'cpp', 'rust'],
        'types': list(set(c['type'] for c in all_chunks))
    }

    # Ensure output directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)

    # Write as JavaScript file (compact JSON to save space)
    with open(output_path, 'w') as f:
        f.write("// Auto-generated search index with embeddings - Do not edit manually\n")
        f.write("// Generated by generate_search_index.py\n")
        f.write("window.SEARCH_INDEX = ")
        # Use compact JSON for smaller file size
        json.dump(index, f, separators=(',', ':'))
        f.write(";\n")

    print(f"[SearchIndex] Written to: {output_path}")
    print(f"[SearchIndex] File size: {output_path.stat().st_size / 1024:.1f} KB")

    return index


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Generate search index')
    parser.add_argument('--no-embeddings', action='store_true',
                        help='Skip embedding computation (faster, keyword-only search)')
    args = parser.parse_args()

    generate_search_index(use_embeddings=not args.no_embeddings)

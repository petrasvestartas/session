#!/usr/bin/env python3
"""
RAG Pipeline for Session Codebase Documentation
- Ingests Point and Color source files from Python, C++, and Rust
- Chunks code by functions, classes, and methods
- Creates embeddings using HuggingFace sentence transformers
- Stores in ChromaDB vector database
- Provides query interface
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Dict, Tuple
import json

try:
    import chromadb
    from chromadb.config import Settings
    from sentence_transformers import SentenceTransformer
except ImportError as e:
    print(f"Error: Required packages not installed: {e}")
    print("Install with:")
    print("  source uvsession/bin/activate")
    print("  pip install chromadb sentence-transformers")
    sys.exit(1)


class CodeChunker:
    """Chunks code files by semantic units (classes, functions, methods)"""

    @staticmethod
    def chunk_python(content: str, file_path: str) -> List[Dict]:
        """Chunk Python code by class and function definitions"""
        chunks = []
        lines = content.split('\n')

        # Find classes
        class_pattern = r'^class\s+(\w+)'
        # Find functions/methods
        func_pattern = r'^(?:    )?def\s+(\w+)\(.*?\):'
        # Find docstrings
        docstring_pattern = r'"""(.*?)"""'

        current_chunk = None
        current_lines = []
        indent_level = 0

        for i, line in enumerate(lines):
            # Check for class definition
            class_match = re.match(class_pattern, line)
            if class_match:
                # Save previous chunk
                if current_chunk and current_lines:
                    current_chunk['code'] = '\n'.join(current_lines)
                    chunks.append(current_chunk)

                # Start new class chunk
                current_chunk = {
                    'type': 'class',
                    'name': class_match.group(1),
                    'file': file_path,
                    'language': 'python',
                    'line_start': i + 1
                }
                current_lines = [line]
                indent_level = 0
                continue

            # Check for function/method definition
            func_match = re.match(func_pattern, line)
            if func_match:
                # Save previous chunk if it's a function (not saving methods separately for now)
                if current_chunk and current_chunk['type'] == 'function' and current_lines:
                    current_chunk['code'] = '\n'.join(current_lines)
                    chunks.append(current_chunk)

                # Determine if it's a method (indented) or top-level function
                is_method = line.startswith('    ')

                if not is_method:
                    # Start new function chunk
                    current_chunk = {
                        'type': 'function',
                        'name': func_match.group(1),
                        'file': file_path,
                        'language': 'python',
                        'line_start': i + 1
                    }
                    current_lines = [line]
                else:
                    # Add method to current class chunk
                    if current_lines:
                        current_lines.append(line)
                continue

            # Add line to current chunk
            if current_lines:
                current_lines.append(line)

        # Save last chunk
        if current_chunk and current_lines:
            current_chunk['code'] = '\n'.join(current_lines)
            chunks.append(current_chunk)

        return chunks

    @staticmethod
    def chunk_cpp(content: str, file_path: str) -> List[Dict]:
        """Chunk C++ code by class and function definitions"""
        chunks = []

        # Remove comments for cleaner parsing
        content_no_comments = re.sub(r'//.*?$', '', content, flags=re.MULTILINE)
        content_no_comments = re.sub(r'/\*.*?\*/', '', content_no_comments, flags=re.DOTALL)

        # Find class definitions
        class_pattern = r'class\s+(\w+)\s*\{([^}]+)\}'
        for match in re.finditer(class_pattern, content, re.DOTALL):
            class_name = match.group(1)
            class_body = match.group(0)

            chunks.append({
                'type': 'class',
                'name': class_name,
                'file': file_path,
                'language': 'cpp',
                'code': class_body,
                'line_start': content[:match.start()].count('\n') + 1
            })

        # Find standalone function definitions
        func_pattern = r'(?:^|\n)([a-zA-Z_][\w:]*)\s+(\w+)\s*\([^)]*\)\s*(?:const)?\s*\{'
        for match in re.finditer(func_pattern, content):
            func_name = match.group(2)
            # Extract function body (simplified - just get reasonable chunk)
            start = match.start()
            # Find the matching closing brace (simplified)
            brace_count = 1
            i = match.end()
            while i < len(content) and brace_count > 0:
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                i += 1

            func_body = content[start:i]

            chunks.append({
                'type': 'function',
                'name': func_name,
                'file': file_path,
                'language': 'cpp',
                'code': func_body,
                'line_start': content[:start].count('\n') + 1
            })

        return chunks

    @staticmethod
    def chunk_rust(content: str, file_path: str) -> List[Dict]:
        """Chunk Rust code by struct, impl blocks, and functions"""
        chunks = []

        # Find struct definitions
        struct_pattern = r'pub struct\s+(\w+)\s*\{([^}]+)\}'
        for match in re.finditer(struct_pattern, content, re.DOTALL):
            struct_name = match.group(1)
            struct_body = match.group(0)

            chunks.append({
                'type': 'struct',
                'name': struct_name,
                'file': file_path,
                'language': 'rust',
                'code': struct_body,
                'line_start': content[:match.start()].count('\n') + 1
            })

        # Find impl blocks and their methods
        impl_pattern = r'impl\s+(\w+)\s*\{(.*?)\n\}'
        for match in re.finditer(impl_pattern, content, re.DOTALL):
            impl_name = match.group(1)
            impl_body = match.group(2)

            # Extract methods from impl block
            method_pattern = r'pub fn\s+(\w+)\s*\([^)]*\)'
            for method_match in re.finditer(method_pattern, impl_body):
                method_name = method_match.group(1)
                # Get some context around the method
                method_start = method_match.start()
                method_end = method_match.end() + 200  # Get next ~200 chars
                method_code = impl_body[max(0, method_start-50):min(len(impl_body), method_end)]

                chunks.append({
                    'type': 'method',
                    'name': f"{impl_name}::{method_name}",
                    'file': file_path,
                    'language': 'rust',
                    'code': method_code,
                    'line_start': content[:match.start() + method_start].count('\n') + 1
                })

        return chunks


class RAGPipeline:
    """Main RAG Pipeline for code documentation"""

    def __init__(self, repo_root: Path, db_path: Path = None):
        self.repo_root = Path(repo_root)
        self.db_path = db_path or (self.repo_root / "rag_db")
        self.db_path.mkdir(parents=True, exist_ok=True)

        # Initialize sentence transformer
        print("[RAG] Loading sentence transformer model...")
        self.embedding_model = SentenceTransformer('all-MiniLM-L6-v2')
        print("[RAG] Model loaded successfully")

        # Initialize ChromaDB
        print(f"[RAG] Initializing ChromaDB at {self.db_path}")
        self.chroma_client = chromadb.PersistentClient(path=str(self.db_path))

        # Get or create collection
        try:
            self.collection = self.chroma_client.get_collection("session_code")
            print(f"[RAG] Loaded existing collection with {self.collection.count()} documents")
        except:
            self.collection = self.chroma_client.create_collection(
                name="session_code",
                metadata={"description": "Session codebase documentation"}
            )
            print("[RAG] Created new collection")

        self.chunker = CodeChunker()

    def ingest_file(self, file_path: Path, language: str) -> int:
        """Ingest a single source file"""
        print(f"[RAG] Ingesting {file_path.name} ({language})...")

        content = file_path.read_text(errors='ignore')

        # Chunk based on language
        if language == 'python':
            chunks = self.chunker.chunk_python(content, str(file_path))
        elif language == 'cpp':
            chunks = self.chunker.chunk_cpp(content, str(file_path))
        elif language == 'rust':
            chunks = self.chunker.chunk_rust(content, str(file_path))
        else:
            return 0

        if not chunks:
            print(f"  No chunks extracted from {file_path.name}")
            return 0

        # Prepare documents for embedding
        documents = []
        metadatas = []
        ids = []

        for i, chunk in enumerate(chunks):
            # Create document text (code + metadata)
            doc_text = f"{chunk['type']} {chunk['name']} in {chunk['language']}\n"
            doc_text += f"File: {chunk['file']}\n"
            doc_text += f"Line: {chunk['line_start']}\n"
            doc_text += f"Code:\n{chunk['code']}"

            documents.append(doc_text)

            metadatas.append({
                'type': chunk['type'],
                'name': chunk['name'],
                'file': str(chunk['file']),
                'language': chunk['language'],
                'line_start': chunk['line_start']
            })

            # Generate unique ID
            file_id = file_path.stem
            ids.append(f"{file_id}_{language}_{chunk['type']}_{i}")

        # Generate embeddings
        embeddings = self.embedding_model.encode(documents).tolist()

        # Add to ChromaDB
        self.collection.add(
            documents=documents,
            embeddings=embeddings,
            metadatas=metadatas,
            ids=ids
        )

        print(f"  Added {len(chunks)} chunks from {file_path.name}")
        return len(chunks)

    def ingest_all(self):
        """Ingest all Point and Color files from all languages"""
        print("[RAG] Starting data ingestion...")

        total_chunks = 0

        # Define files to ingest
        files_to_ingest = [
            # Python
            (self.repo_root / "session_py/src/session_py/point.py", "python"),
            (self.repo_root / "session_py/src/session_py/color.py", "python"),
            (self.repo_root / "session_py/src/session_py/point_test.py", "python"),
            (self.repo_root / "session_py/src/session_py/color_test.py", "python"),

            # C++
            (self.repo_root / "session_cpp/src/point.h", "cpp"),
            (self.repo_root / "session_cpp/src/point.cpp", "cpp"),
            (self.repo_root / "session_cpp/src/color.h", "cpp"),
            (self.repo_root / "session_cpp/src/color.cpp", "cpp"),
            (self.repo_root / "session_cpp/src/point_test.cpp", "cpp"),
            (self.repo_root / "session_cpp/src/color_test.cpp", "cpp"),

            # Rust
            (self.repo_root / "session_rust/src/point.rs", "rust"),
            (self.repo_root / "session_rust/src/color.rs", "rust"),
            (self.repo_root / "session_rust/src/point_test.rs", "rust"),
            (self.repo_root / "session_rust/src/color_test.rs", "rust"),
        ]

        for file_path, language in files_to_ingest:
            if file_path.exists():
                total_chunks += self.ingest_file(file_path, language)
            else:
                print(f"[RAG] Warning: {file_path} not found")

        print(f"\n[RAG] Ingestion complete. Total chunks: {total_chunks}")
        print(f"[RAG] Collection size: {self.collection.count()}")

    def query(self, query_text: str, n_results: int = 5) -> List[Dict]:
        """Query the RAG system"""
        print(f"\n[RAG] Query: {query_text}")

        # Generate query embedding
        query_embedding = self.embedding_model.encode([query_text])[0].tolist()

        # Search in ChromaDB
        results = self.collection.query(
            query_embeddings=[query_embedding],
            n_results=n_results
        )

        # Format results
        formatted_results = []
        for i in range(len(results['ids'][0])):
            formatted_results.append({
                'id': results['ids'][0][i],
                'document': results['documents'][0][i],
                'metadata': results['metadatas'][0][i],
                'distance': results['distances'][0][i] if 'distances' in results else None
            })

        return formatted_results

    def clear_database(self):
        """Clear the vector database"""
        print("[RAG] Clearing database...")
        try:
            self.chroma_client.delete_collection("session_code")
            self.collection = self.chroma_client.create_collection(
                name="session_code",
                metadata={"description": "Session codebase documentation"}
            )
            print("[RAG] Database cleared")
        except Exception as e:
            print(f"[RAG] Error clearing database: {e}")


def main():
    import argparse

    parser = argparse.ArgumentParser(description='RAG Pipeline for Session Codebase')
    parser.add_argument('command', choices=['ingest', 'query', 'clear', 'info'],
                       help='Command to execute')
    parser.add_argument('--query', type=str, help='Query string for search')
    parser.add_argument('--results', type=int, default=5, help='Number of results to return')
    parser.add_argument('--repo', type=str, help='Repository root path')

    args = parser.parse_args()

    # Determine repo root
    if args.repo:
        repo_root = Path(args.repo)
    else:
        repo_root = Path(__file__).parent

    # Initialize RAG pipeline
    rag = RAGPipeline(repo_root)

    if args.command == 'ingest':
        rag.ingest_all()

    elif args.command == 'query':
        if not args.query:
            print("Error: --query required for query command")
            sys.exit(1)

        results = rag.query(args.query, n_results=args.results)

        print(f"\nFound {len(results)} results:\n")
        for i, result in enumerate(results, 1):
            print(f"Result {i}:")
            print(f"  Type: {result['metadata']['type']}")
            print(f"  Name: {result['metadata']['name']}")
            print(f"  File: {result['metadata']['file']}")
            print(f"  Language: {result['metadata']['language']}")
            print(f"  Distance: {result['distance']:.4f}")
            print()

    elif args.command == 'clear':
        rag.clear_database()

    elif args.command == 'info':
        print(f"[RAG] Database location: {rag.db_path}")
        print(f"[RAG] Collection: session_code")
        print(f"[RAG] Total documents: {rag.collection.count()}")
        print(f"[RAG] Embedding model: all-MiniLM-L6-v2")


if __name__ == '__main__':
    main()

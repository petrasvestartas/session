#!/usr/bin/env python3
"""
Flask API server for RAG Pipeline
Exposes RAG query functionality via REST API for Vue CLI interface
"""

# Force CPU usage to avoid CUDA conflicts - MUST be before any imports
import os
os.environ['CUDA_VISIBLE_DEVICES'] = '-1'

from flask import Flask, request, jsonify
from flask_cors import CORS
from pathlib import Path
import sys

# Import RAG pipeline and answer generator
from rag_pipeline import RAGPipeline
from rag_answer_generator import generate_answer, generate_summary_answer

app = Flask(__name__)
CORS(app)  # Enable CORS for Vue dev server

# Initialize RAG pipeline
# rag_api.py is in session_tests/rag/, so go up 2 levels to get repo root
repo_root = Path(__file__).parent.parent.parent
rag = None

def get_rag():
    """Lazy initialization of RAG pipeline"""
    global rag
    if rag is None:
        print("[API] Initializing RAG pipeline...", file=sys.stderr)
        # Database is in session_tests/rag_db/
        db_path = repo_root / "session_tests" / "rag_db"
        rag = RAGPipeline(repo_root, db_path=db_path)
        print(f"[API] RAG ready with {rag.collection.count()} documents", file=sys.stderr)
    return rag

@app.route('/api/health', methods=['GET'])
def health():
    """Health check endpoint"""
    try:
        r = get_rag()
        return jsonify({
            'status': 'ok',
            'documents': r.collection.count()
        })
    except Exception as e:
        return jsonify({
            'status': 'error',
            'message': str(e)
        }), 500

@app.route('/api/query', methods=['POST'])
def query():
    """Query endpoint

    Request body:
    {
        "query": "how to create a Point in Python",
        "n_results": 5  // optional, default 5
    }

    Response:
    {
        "query": "...",
        "results": [
            {
                "type": "class",
                "name": "Point",
                "file": "...",
                "language": "python",
                "distance": 1.0144,
                "document": "..."
            }
        ]
    }
    """
    try:
        data = request.get_json()

        if not data or 'query' not in data:
            return jsonify({
                'error': 'Missing query parameter'
            }), 400

        query_text = data['query']
        n_results = data.get('n_results', 5)

        r = get_rag()
        results = r.query(query_text, n_results=n_results)

        return jsonify({
            'query': query_text,
            'results': results
        })

    except Exception as e:
        return jsonify({
            'error': str(e)
        }), 500

@app.route('/api/info', methods=['GET'])
def info():
    """Database info endpoint"""
    try:
        r = get_rag()
        return jsonify({
            'db_path': str(r.db_path),
            'collection': 'session_code',
            'total_documents': r.collection.count(),
            'embedding_model': 'all-MiniLM-L6-v2'
        })
    except Exception as e:
        return jsonify({
            'error': str(e)
        }), 500

@app.route('/api/reingest', methods=['POST'])
def reingest():
    """Trigger re-ingestion of all files"""
    try:
        r = get_rag()
        r.clear_database()
        r.ingest_all()

        return jsonify({
            'status': 'success',
            'total_chunks': r.collection.count()
        })
    except Exception as e:
        return jsonify({
            'error': str(e)
        }), 500

@app.route('/api/ask', methods=['POST'])
def ask():
    """Ask a question and get a natural language answer

    Request body:
    {
        "question": "how to create a Point in Python",
        "n_results": 3  // optional, default 3
    }

    Response:
    {
        "question": "...",
        "answer": "...",  // natural language response
        "sources": [...]  // supporting code references
    }
    """
    try:
        data = request.get_json()

        if not data or 'question' not in data:
            return jsonify({
                'error': 'Missing question parameter'
            }), 400

        question = data['question']
        n_results = data.get('n_results', 3)

        r = get_rag()
        results = r.query(question, n_results=n_results)

        # Generate natural language answer
        answer_text = generate_answer(question, results)

        # Format sources
        sources = []
        for result in results[:3]:
            meta = result['metadata']
            sources.append({
                'type': meta['type'],
                'name': meta['name'],
                'language': meta['language'],
                'file': meta['file'].split('/')[-1],
                'line': meta['line_start'],
                'distance': result['distance']
            })

        return jsonify({
            'question': question,
            'answer': answer_text,
            'sources': sources
        })

    except Exception as e:
        return jsonify({
            'error': str(e)
        }), 500

if __name__ == '__main__':
    # Run on port 8770 (Vite is on 8769)
    print("[API] Starting RAG API server on http://localhost:8770")
    app.run(host='0.0.0.0', port=8770, debug=False)

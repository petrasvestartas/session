#!/bin/bash
# Start RAG API Server

cd "$(dirname "$0")"

# Kill old instances
pkill -9 -f rag_api.py 2>/dev/null

echo "🚀 Starting RAG API..."
PYTHONDONTWRITEBYTECODE=1 /home/pv/anaconda3/bin/python3 -u rag_api.py > /tmp/rag_api.log 2>&1 &

sleep 3

if curl -s http://localhost:8770/api/ask > /dev/null 2>&1; then
    echo "✅ RAG API running on http://localhost:8770"
    echo "📊 Logs: tail -f /tmp/rag_api.log"
else
    echo "❌ Failed to start. Check /tmp/rag_api.log"
    exit 1
fi

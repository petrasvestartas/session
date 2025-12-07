"""
Answer Generator for RAG System - Claude API Version
Generates natural language answers using Claude AI with retrieved code context
"""

import os
import time
from anthropic import Anthropic

# Claude client - initialized lazily on first use
_client = None
_client_initialized = False


def _get_claude_client():
    """
    Lazy initialization of Claude client.
    Only creates client when first needed, not at module import time.
    """
    global _client, _client_initialized

    if _client_initialized:
        return _client

    _client_initialized = True

    try:
        # Load API key from a runtime source ONLY. Do NOT hardcode secrets in the
        # repository. Preferred source is environment variable
        # `ANTHROPIC_API_KEY`. For local convenience you can store the key in the
        # OS keyring (via `keyring` package) under service name "anthropic" and
        # account equal to your username.
        api_key = os.environ.get('ANTHROPIC_API_KEY')

        if not api_key:
            # Try OS keyring if available (optional)
            try:
                import keyring
                api_key = keyring.get_password("anthropic", os.getlogin())
            except Exception:
                api_key = None

        if api_key:
            _client = Anthropic(api_key=api_key)
            # Don't print the key itself; only a success message
            print("[SUCCESS] Claude API initialized (key loaded from runtime source)")
        else:
            print("[WARNING] No Anthropic API key available; using fallback mode")
    except Exception as e:
        print(f"[ERROR] Claude API initialization failed: {e}")
        _client = None

    return _client


def generate_answer(query: str, results: list) -> str:
    """
    Generate a natural language answer using Claude AI.

    Args:
        query: User's question
        results: List of retrieval results with metadata and code

    Returns:
        Formatted answer string
    """
    if not results:
        return "I couldn't find any relevant code in the Session codebase for your query. Try asking about Point or Color classes in Python, C++, or Rust."

    # Get Claude client (lazy initialization)
    print(f"[DEBUG] Getting Claude client for query: {query}")
    client = _get_claude_client()
    print(f"[DEBUG] Claude client: {client}")

    # If Claude API is not available, fall back to simple template
    if client is None:
        print(f"[DEBUG] No Claude client, using fallback")
        return _fallback_answer(query, results)

    print(f"[DEBUG] Claude client available, generating answer...")

    try:
        t_start = time.time()

        # Format retrieved code for Claude
        context_parts = []
        test_parts = []  # Separate list for test code

        for i, result in enumerate(results[:10], 1):  # Check more results
            meta = result['metadata']
            doc = result['document']

            part = (
                f"**Result {i}** ({meta['language'].upper()}):\n"
                f"File: {meta['file']}\n"
                f"Type: {meta['type']} - {meta['name']}\n"
                f"```{meta['language']}\n{doc}\n```"
            )

            # Separate test code from implementation code
            if 'test' in meta['file'].lower():
                test_parts.append(part)
            else:
                context_parts.append(part)

        # Prefer implementation code, but include test code if that's all we have
        if not context_parts:
            context_parts = test_parts[:3]  # Use top 3 test examples if no implementation found

        if not context_parts:
            return _fallback_answer(query, results)

        context = "\n\n".join(context_parts)
        t_context = time.time()
        print(f"[PERF] Context preparation: {(t_context - t_start)*1000:.0f}ms")

        # Ask Claude to generate answer
        message = client.messages.create(
            model="claude-3-haiku-20240307",
            max_tokens=800,
            temperature=0.3,
            messages=[{
                "role": "user",
                "content": f"""You are a helpful coding assistant for the Session geometry library, which has Python, C++, and Rust implementations.

User's question: {query}

Here is relevant code from the codebase:

{context}

Instructions:
- Provide a SHORT, CLEAR, CONVERSATIONAL answer (2-4 sentences max)
- IMPORTANT: If the user's question doesn't specify a language (like "how to create a point"), you MUST show examples for ALL THREE languages: Python, C++, AND Rust
- If the user asks for a specific language (like "how to create a point in python"), only show that language
- Show practical code examples using the code from above as reference
- Use markdown code blocks with language tags (```python, ```cpp, ```rust)
- Label each language section with **Python:**, **C++:**, or **Rust:**
- Focus on HOW TO USE the code, not internal implementation details
- Don't mention file names or line numbers unless specifically asked
- Be concise like ChatGPT or Claude - get straight to the point

Answer:"""
            }]
        )
        t_claude = time.time()
        print(f"[PERF] Claude API call: {(t_claude - t_context)*1000:.0f}ms")
        print(f"[PERF] Total answer generation: {(t_claude - t_start)*1000:.0f}ms")

        return message.content[0].text

    except Exception as e:
        print(f"[ERROR] Claude API failed: {e}")
        return _fallback_answer(query, results)


def _fallback_answer(query: str, results: list) -> str:
    """
    Fallback template-based answer when Claude API is not available.
    """
    if not results:
        return "No relevant code found."

    best = results[0]
    meta = best['metadata']
    doc = best['document']

    # Build simple answer
    answer_parts = []

    # Skip test files
    if 'test' in meta['file'].lower():
        answer_parts.append(f"Found {meta['type']} `{meta['name']}` in {meta['language'].upper()}:")
    else:
        answer_parts.append(f"Here's the {meta['type']} `{meta['name']}` in {meta['language'].upper()}:")

    answer_parts.append("")
    answer_parts.append(f"```{meta['language']}")

    # Extract code portion
    lines = doc.split('\n')
    code_start_idx = 0
    for i, line in enumerate(lines):
        if line.startswith('Code:'):
            code_start_idx = i + 1
            break

    if code_start_idx > 0:
        code_snippet = '\n'.join(lines[code_start_idx:code_start_idx + 15])
        answer_parts.append(code_snippet.strip())
    else:
        answer_parts.append(doc[:500])  # First 500 chars

    answer_parts.append("```")
    answer_parts.append("")
    answer_parts.append(f"*Source: {meta['file']} (line {meta['line_start']})*")

    # Show alternatives in other languages
    other_langs = []
    for result in results[1:4]:
        alt_meta = result['metadata']
        if alt_meta['language'] != meta['language']:
            other_langs.append(f"{alt_meta['language'].upper()}")

    if other_langs:
        langs_str = ', '.join(set(other_langs))
        answer_parts.append(f"Also available in: {langs_str}")

    return '\n'.join(answer_parts)


def generate_summary_answer(query: str, results: list) -> dict:
    """
    Generate structured answer with both summary and details.

    Returns:
        dict with 'summary' and 'details' fields
    """
    if not results:
        return {
            'summary': "No relevant code found.",
            'details': []
        }

    summary = generate_answer(query, results)

    details = []
    for i, result in enumerate(results[:5], 1):
        meta = result['metadata']
        details.append({
            'rank': i,
            'type': meta['type'],
            'name': meta['name'],
            'language': meta['language'],
            'file': meta['file'].split('/')[-1],
            'line': meta['line_start'],
            'distance': result['distance']
        })

    return {
        'summary': summary,
        'details': details
    }

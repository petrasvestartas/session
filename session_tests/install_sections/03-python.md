# Python

## Install & download

[Python](https://www.python.org/downloads/) 3.10+, then uv.

**Windows:**

```bash
winget install astral-sh.uv
```

**macOS / Linux:**

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

## Create env & run tests

**Windows:**

```bash
uv venv uvsession
uvsession\Scripts\activate
uv pip install -e session_py pytest
python -m session_py.point_test
```

**macOS / Linux:**

```bash
uv venv uvsession
source uvsession/bin/activate
uv pip install -e session_py pytest
python -m session_py.point_test
```

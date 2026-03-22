cd "$1"
shift 1

if [ ! -d ".venv" ]; then
    echo "[Mypy LSP] Initialization of Mypy LSP server..." >&2
    python3 -m venv .venv
    .venv/bin/pip install -r requirements.txt
fi

exec .venv/bin/python server.py "$@"

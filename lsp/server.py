import json
import subprocess
import sys
from dataclasses import dataclass
from urllib.parse import unquote, urlparse

from lsprotocol import types
from pygls.lsp.server import LanguageServer

server = LanguageServer("mypy-lsp", "v0.1")

args = sys.argv
if len(args) != 4:
    raise RuntimeError("Usage: python server.py <project_root> <mypy-path> <mypy-args>")

[_, PROJECT_ROOT, MYPY_PATH, MYPY_ARGS] = args
MYPY_ARGS_LIST: list[str] = json.loads(MYPY_ARGS)


@dataclass(kw_only=True, frozen=True, slots=True)
class MypyError:
    line_start: int
    col_start: int
    line_end: int
    col_end: int
    message: str


@server.feature(types.TEXT_DOCUMENT_DID_SAVE)
def on_save(
    params: types.DidSaveTextDocumentParams,
) -> None:
    file_uri = params.text_document.uri
    file_path = unquote(urlparse(file_uri).path)
    mypy_subprocess_response = subprocess.run(
        [
            MYPY_PATH,
            "--no-color-output",
            "--no-error-summary",
            "--show-column-numbers",
            "--show-error-end",
            *MYPY_ARGS_LIST,
            file_path,
        ],
        capture_output=True,
        cwd=PROJECT_ROOT,
    )
    mypy_stdout = mypy_subprocess_response.stdout.decode()
    errors = list[MypyError]()
    for line in mypy_stdout.splitlines():
        [path, msg] = line.split(" ", maxsplit=1)
        [_, line_start, col_start, line_end, col_end, *_] = path.split(":")
        errors.append(
            MypyError(
                line_start=int(line_start),
                line_end=int(line_end),
                col_start=int(col_start),
                col_end=int(col_end),
                message=msg,
            )
        )

    server.text_document_publish_diagnostics(
        params=types.PublishDiagnosticsParams(
            uri=params.text_document.uri,
            diagnostics=[
                types.Diagnostic(
                    range=types.Range(
                        start=types.Position(
                            line=error.line_start - 1, character=error.col_start - 1
                        ),
                        end=types.Position(
                            line=error.line_end - 1, character=error.col_end
                        ),
                    ),
                    message=error.message,
                )
                for error in errors
            ],
        )
    )


if __name__ == "__main__":
    server.start_io()

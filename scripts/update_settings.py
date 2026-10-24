#!/usr/bin/env python3
"""
# rust-analyzer の設定を VS Code / Zed の setting.json (JSONC) に書き込む。

コメントや書式を保つため、JSON を再生成せず、対象の値の部分だけをテキスト編集する。
"""

import argparse
import os
import sys
from pathlib import Path

import jsonc_parser as jsonc

TARGETS = {
    "vscode": (".vscode/settings.json", lambda proj, srv: [
        (["rust-analyzer.linkedProjects"], [proj]),
        (["rust-analyzer.procMacro.server"], srv),
    ]),
    "zed": (".zed/settings.json", lambda proj, srv: [
        (["lsp", "rust-analyzer", "initialization_options", "linkedProjects"], [proj]),
        (["lsp", "rust-analyzer", "initialization_options", "procMacro", "server"], srv),
    ]),
}

def update(path, edits):
    p = Path(path)
    old = p.read_text(encoding="utf-8") if p.exists() else ""
    new = old
    for keys, value in edits:
        new = jsonc.set_path(new, keys, value)

    if new == old:
        print(f"unchanged: {path}")
        return

    if not new.endswith("\n"):
        new += "\n"

    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(new, encoding="utf-8")

    print(f"updated: {path}")

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("server", help="Path to rust-analyzer-proc-macro-srv")
    ap.add_argument("--project", default="rust-project.json")
    ap.add_argument("--editors", default="vscode,zed")
    args = ap.parse_args()

    if not os.access(args.server, os.X_OK):
        sys.exit(f"error: proc-macro server not found: {args.server}")

    for name in args.editors.split(","):
        if name not in TARGETS:
            sys.exit(f"Unknown editor: {name} (vscode, zed)")

        path, make_edits = TARGETS[name]
        try:
            update(path, make_edits(args.project, args.server))
        except jsonc.ParseError as e:
            sys.exit(f"error: Failed to parse {path}: {e}")


if __name__ == "__main__":
    main()

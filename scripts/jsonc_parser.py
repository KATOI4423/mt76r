#!/usr/bin/env python3

import json


class ParseError(Exception):
    pass

class Parser:
    """位置情報付きの最小の JSONC パーサ（コメントと末尾カンマに対応）"""

    def __init__(self, text):
        self.t = text

    def skip(self, i):
        t, n = self.t, len(self.t)
        while i < n:
            if t[i] in " \t\r\n":
                i += 1
            elif t.startswith("//", i):
                j = t.find("\n", i)
                i = n if j < 0 else j + 1
            elif t.startswith("/*", i):
                j = t.find("*/", i + 2)
                if j < 0:
                    raise ParseError("Comment not closed")
                i = j + 2
            else:
                break
        return i

    def string(self, i):
        t, j = self.t, i + 1
        while j < len(t):
            if t[j] == "\\":
                j += 2
            elif t[j] == '"':
                return j + 1, json.loads(t[i : j+1])
            else:
                j += 1
        raise ParseError("String not closed")

    def value(self, i):
        i = self.skip(i)
        t = self.t

        if i >= len(t):
            raise ParseError("No value")
        c = t[i]
        if c == "{":
            return self.obj(i)
        if c == "[":
            return self.arr(i)
        if c == '"':
            end, _ = self.string(i)
            return {"type": "other", "start": i, "end": end}

        j = i
        while (j < len(t) and t[j] not in ",}]" and not t[j].isspace()
                and not t.startswith(("//", "/*"), j)):
            j += 1

        if j == i:
            raise ParseError(f"Unexpected charactor {c!r} (offset {i})")

        return {"type": "other", "start": i, "end": j}

    def obj(self, start):
        t, members = self.t, []
        i = self.skip(start + 1)

        while True:
            if i >= len(t):
                raise ParseError("Object not closed")
            if t[i] == "}":
                return {"type": "object", "start": start, "end": i + 1, "close": i, "members": members}
            if t[i] != '"':
                raise ParseError(f"Key not found (offset {i})")

            i, key = self.string(i)
            i = self.skip(i)
            if i >= len(t) or t[i] != ":":
                raise ParseError(f"Delimiter ':' not found (offset {i})")

            node = self.value(i + 1)
            members.append((key, node))

            i = self.skip(node["end"])
            if i < len(t) and t[i] == ",":
                i = self.skip(i + 1)
            elif i >= len(t) or t[i] != "}":
                raise ParseError(f"Neigher ',' nor '}}' found (offset {i})")

    def arr(self, start):
        t = self.t
        i = self.skip(start + 1)

        while True:
            if i >= len(t):
                raise ParseError("Array not closed")
            if t[i] == "]":
                return {"type": "other", "start": start, "end": i + 1}

            node = self.value(i)
            i = self.skip(node["end"])
            if i < len(t) and t[i] == ",":
                i = self.skip(i + 1)
            elif i >= len(t) or t[i] != "]":
                raise ParseError(f"Neigher ',' nor ']' found (offset {i})")

def line_indent(text, pos):
    ls = text.rfind("\n", 0, pos) + 1
    j = ls
    while j < len(text) and text[j] in " \t":
        j += 1
    return text[ls:j]

def render(value, indent):
    s = json.dumps(value, indent=2, ensure_ascii=False)
    return s.replace("\n", "\n" + indent)

def eol(text, pos):
    nl = text.find("\n", pos)
    if nl < 0:
        return None
    tail = text[pos:nl].strip()
    return nl if tail == "" or tail.startswith("//") else None

def nest(keys, value):
    for k in reversed(keys):
        value = {k: value}
    return value

def replace(text, node, value):
    old = text[node["start"]:node["end"]]
    try:
        if json.loads(old) == value:
            return text # not changed if same value
    except ValueError:
        pass

    indent = line_indent(text, node["start"])
    return text[:node["start"]] + render(value, indent) + text[node["end"]:]

def insert(text, obj, key, value):
    base = line_indent(text, obj["start"])
    child = base + "  "
    member = f"{json.dumps(key, ensure_ascii=False)}: {render(value, child)}"
    if not obj["members"]:
        pos = obj["start"] + 1
        if text[pos:obj["close"]].strip() == "":
            return text[:pos] + f"\n{child}{member}\n{base}" + text[obj["close"]:]
        return text[:pos] + f"\n{child}{member}" + text[pos:]

    last_end = obj["members"][-1][1]["end"]
    j = Parser(text).skip(last_end)
    if text[j] == ",": # tailing comma
        nl = eol(text, j + 1)
        pos = nl if nl is not None else j + 1
        return text[:pos] + f"\n{child}{member}," + text[pos:]

    nl = eol(text, last_end)
    if nl is None:
        return text[:last_end] + f",\n{child}{member}" + text[last_end:]
    return text[:last_end] + "," + text[last_end:nl] + f"\n{child}{member}" + text[nl:]

def set_path(text, path, value):
    p = Parser(text)
    if p.skip(0) >= len(text): # empty or only comments
        text = (text.rstrip("\n") + "\n" if text.strip() else "") + "{}\n"
        p = Parser(text)

    node = p.value(p.skip(0))
    if node["type"] != "object":
        raise ParseError("root is not object")

    for depth, key in enumerate(path):
        rest = path[depth + 1:]
        member = next((n for k, n in reversed(node["members"]) if k == key), None)
        if member is None:
            return insert(text, node, key, nest(rest, value))
        if not rest:
            return replace(text, member, value)
        if member["type"] != "object":
            return replace(text, member, nest(rest, value))
        node = member
    return text

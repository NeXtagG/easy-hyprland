#!/usr/bin/env python3
"""Validates a generated hyprlang config against the Hyprland 0.56.2 option,
dispatcher and window-rule lists in tests/hyprland-0.56.2/.

usage: validate_conf.py <data dir> <config.conf>
"""
import re
import sys

data_dir, path = sys.argv[1], sys.argv[2]


def read(name):
    with open(f"{data_dir}/{name}") as f:
        return [line.strip() for line in f if line.strip()]


options = {line.split(" ", 1)[1] for line in read("options.txt")}
dispatchers = set(read("legacy_dispatchers.txt"))
effects = set(read("rule_effects.txt"))
props = set(read("rule_props.txt"))
keywords = {"monitor", "env", "exec-once", "exec", "bind", "bindm", "bindl", "bindel", "gesture", "source"}

errors = []
stack = []
counts = {"options": 0, "binds": 0, "rules": 0}


def strip_comment(line):
    # "##" is an escaped "#", a single "#" starts a comment.
    out, i = [], 0
    while i < len(line):
        if line[i] == "#":
            if line[i + 1 : i + 2] == "#":
                out.append("#")
                i += 2
                continue
            break
        out.append(line[i])
        i += 1
    return "".join(out).strip()


with open(path) as f:
    for n, raw in enumerate(f, 1):
        line = strip_comment(raw.rstrip("\n"))
        if not line:
            continue
        where = f"line {n}: {line}"
        if line.endswith("{"):
            stack.append(line[:-1].strip())
            continue
        if line == "}":
            if not stack:
                errors.append(f"{where}: unbalanced }}")
            else:
                stack.pop()
            continue
        if "=" not in line:
            errors.append(f"{where}: not a key = value line")
            continue
        key, value = (p.strip() for p in line.split("=", 1))

        if stack and stack[0] == "windowrule":
            if key == "name":
                continue
            if key.startswith("match:"):
                if key[6:] not in props:
                    errors.append(f"{where}: unknown match prop")
                continue
            if key not in effects:
                errors.append(f"{where}: unknown window rule effect")
            counts["rules"] += 1
            continue

        if stack == ["animations"] and key in ("bezier", "animation"):
            parts = [p.strip() for p in value.split(",")]
            if key == "bezier" and (len(parts) != 5 or not all(re.fullmatch(r"-?\d+(\.\d+)?", p) for p in parts[1:])):
                errors.append(f"{where}: bezier needs name + 4 numbers")
            if key == "animation":
                beziers = {m.group(1) for m in re.finditer(r"bezier\s*=\s*([^,\s]+)", open(path).read())} | {"default"}
                if len(parts) < 4 or parts[1] not in ("0", "1") or parts[3] not in beziers:
                    errors.append(f"{where}: animation needs leaf, on/off, speed, defined bezier")
            continue

        if stack:
            full = ":".join(stack + [key])
            if full not in options:
                errors.append(f"{where}: unknown option {full}")
            counts["options"] += 1
            continue

        if key.startswith("$"):
            continue
        if key not in keywords:
            errors.append(f"{where}: unknown keyword")
            continue
        if key in ("bind", "bindl", "bindel"):
            parts = [p.strip() for p in value.split(",", 3)]
            if len(parts) < 3 or parts[2] not in dispatchers:
                errors.append(f"{where}: unknown dispatcher")
            if len(parts) > 1 and not parts[1]:
                errors.append(f"{where}: empty key")
            counts["binds"] += 1
        if key == "bindm" and value.split(",")[-1].strip() not in ("movewindow", "resizewindow"):
            errors.append(f"{where}: bad mouse bind")
        if key == "monitor" and len(value.split(",")) < 4:
            errors.append(f"{where}: monitor needs name,mode,position,scale")
        if key == "env" and "," not in value:
            errors.append(f"{where}: env needs NAME,value")

if stack:
    errors.append(f"unclosed blocks: {stack}")
if re.search(r"windowrulev2", open(path).read()):
    errors.append("windowrulev2 is a hard error since Hyprland 0.53")

if errors:
    for e in errors:
        print("ERROR:", e)
    sys.exit(1)
print(f"OK  {path}  (options: {counts['options']}, binds: {counts['binds']}, rule effects: {counts['rules']})")

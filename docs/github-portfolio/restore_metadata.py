#!/usr/bin/env python3
"""Restore (or diff) public-repo description and topics from baseline-metadata.json.

Usage:
  python3 restore_metadata.py diff               # show live vs baseline, change nothing
  python3 restore_metadata.py restore [repo ...] # re-apply baseline to the named repos (default: all)

Only description and topics are touched. Visibility, branch protection, secrets and
workflows are never modified. Requires an authenticated `gh` with write access.
"""
import json, subprocess, sys, pathlib

BASE = json.loads((pathlib.Path(__file__).parent / "baseline-metadata.json").read_text())
OWNER = "elliyeen"


def gh(*args, stdin=None):
    r = subprocess.run(["gh", *args], capture_output=True, text=True, input=stdin)
    if r.returncode:
        sys.exit(f"gh {' '.join(args)} failed: {r.stderr.strip()}")
    return r.stdout


def live(name):
    d = json.loads(gh("api", f"repos/{OWNER}/{name}"))
    return d.get("description") or "", sorted(d.get("topics") or [])


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "diff"
    only = set(sys.argv[2:])
    drift = 0
    for r in BASE["public_repos"]:
        if only and r["name"] not in only:
            continue
        desc, topics = live(r["name"])
        same = desc == r["description"] and topics == r["topics"]
        if not same:
            drift += 1
            print(f"DRIFT {r['name']}: desc {desc!r} -> baseline {r['description']!r}; topics {topics} -> {r['topics']}")
        if mode == "restore" and not same:
            if r["fork"]:
                continue
            gh("api", "-X", "PATCH", f"repos/{OWNER}/{r['name']}", "-f", f"description={r['description']}")
            gh("api", "-X", "PUT", f"repos/{OWNER}/{r['name']}/topics", "--input", "-",
               stdin=json.dumps({"names": r["topics"]}))
            print(f"RESTORED {r['name']}")
    print(f"{drift} repo(s) differ from baseline")


main()

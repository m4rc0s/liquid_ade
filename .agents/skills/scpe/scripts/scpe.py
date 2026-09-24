#!/usr/bin/env python3
"""
scpe.py — engine for the SCPE (Spec-Compiled Product Engineering) workflow.

Everything is plain Markdown inside one product workspace. The workspace root is
`SCPE_WORKSPACE` when set, otherwise the first directory walking up from the cwd
that contains `product_vision.md` and `features/`. Templates live in
`../templates/` relative to this file. Standard library only.

Subcommands:
    init <name> [--path DIR] [--app NAME:TYPE]... [--no-git]
    feature <slug> "<Title>"
    epic <feature> <slug> "<Title>"
    validate [--strict]
    gate-check <feature>/<epic>
    set-state <feature>/<epic> <State> --by "<who>" [--note "..."]
    mark-stale [<path>] [--from-hook]
    status [--json] [--write]
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import re
import subprocess
import sys
from pathlib import Path

SKILL_DIR = Path(__file__).resolve().parent.parent
TEMPLATES_DIR = SKILL_DIR / "templates"

ROOT_FILES = [
    "index.md", "product_vision.md", "roadmap.md", "glossary.md",
    "architecture.md", "technical_deal.md", "team_playbook.md", "quick_status.md",
]
ROOT_DIRS = ["apps", "features", "assets"]
FEATURE_FILES = ["index.md", "feat_roadmap.md", "quick_status.md"]
EPIC_FILES = ["index.md", "plan.md", "tasks.md", "quick_status.md", "epic_roadmap.md"]
APP_FIELDS = [
    "name", "type", "description", "stack", "standards",
    "entrypoint", "depends_on", "run", "test",
]
FEATURE_FIELDS = ["goal", "users", "bounded_context", "business_value"]

STATES = ["Draft", "Ready", "WIP", "Blocked", "Done", "Stale"]
# SCPE_METHOD.md §4.4 (state machine) and §4.5 (spec drift).
TRANSITIONS = {
    "Draft": {"Ready"},
    "Ready": {"WIP", "Blocked"},
    "WIP": {"Done", "Blocked"},
    "Blocked": {"Ready", "Draft"},
    "Done": {"Stale"},
    "Stale": {"Ready", "Done"},
}

SLUG_RE = re.compile(r"^[a-z0-9][a-z0-9_-]*$")
STATUS_START = "<!-- scpe:status:start -->"
STATUS_END = "<!-- scpe:status:end -->"


# ---------------------------------------------------------------- infra -----

def die(msg: str, code: int = 1) -> "NoReturn":  # type: ignore[valid-type]
    print(f"❌ {msg}", file=sys.stderr)
    sys.exit(code)


def today() -> str:
    return dt.date.today().isoformat()


def is_workspace(path: Path) -> bool:
    return (path / "product_vision.md").is_file() and (path / "features").is_dir()


def find_workspace(start: Path | None = None) -> Path | None:
    env = os.environ.get("SCPE_WORKSPACE")
    if env:
        p = Path(env).expanduser().resolve()
        return p if is_workspace(p) else None
    cur = (start or Path.cwd()).resolve()
    for p in [cur, *cur.parents]:
        if is_workspace(p):
            return p
    return None


def workspace() -> Path:
    ws = find_workspace()
    if ws is None:
        die(
            "no SCPE workspace found (a folder with product_vision.md and features/).\n"
            "   cd into the product workspace, set SCPE_WORKSPACE, or run `scpe.py init`."
        )
    return ws


def read(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8", errors="ignore") if p.is_file() else ""
    except Exception:
        return ""



def render(template: Path, values: dict) -> str:
    text = template.read_text(encoding="utf-8")
    for key, val in values.items():
        text = text.replace("{{" + key + "}}", str(val))
    return text


def write_new(dest: Path, text: str) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_text(text, encoding="utf-8")


def check_slug(slug: str, what: str) -> None:
    if not SLUG_RE.match(slug):
        die(f"invalid {what} slug '{slug}': use lowercase letters, digits, '-' or '_'.")


# ------------------------------------------------------------- markdown -----

def field(text: str, name: str) -> str | None:
    m = re.search(rf"^- \*\*{re.escape(name)}:\*\*[ \t]*(.*)$", text, re.MULTILINE)
    return m.group(1).strip() if m else None


def strip_comments(text: str) -> str:
    return re.sub(r"<!--.*?-->", "", text, flags=re.DOTALL)


def section(text: str, heading: str) -> str | None:
    """Body of `## heading` up to the next `## ` heading, or None if absent."""
    m = re.search(rf"^## {re.escape(heading)}[ \t]*$", text, re.MULTILINE)
    if not m:
        return None
    rest = text[m.end():]
    nxt = re.search(r"^## ", rest, re.MULTILINE)
    return rest[: nxt.start()] if nxt else rest


def append_to_section(text: str, heading: str, line: str, placeholder: str | None = None) -> str:
    """Append a list line at the end of `## heading`, replacing a placeholder line."""
    m = re.search(rf"^## {re.escape(heading)}[ \t]*$", text, re.MULTILINE)
    if not m:
        return text.rstrip("\n") + f"\n\n## {heading}\n\n{line}\n"
    start = m.end()
    nxt = re.search(r"^## ", text[start:], re.MULTILINE)
    end = start + nxt.start() if nxt else len(text)
    body = text[start:end]
    if placeholder and placeholder in body:
        body = body.replace(placeholder, line, 1).rstrip("\n") + "\n"
    else:
        body = body.rstrip("\n") + f"\n{line}\n"
    tail = text[end:]
    return text[:start] + body + ("\n" if tail else "") + tail


def epic_state(qs_text: str) -> tuple[str | None, int]:
    found = re.findall(r"^- \*\*state:\*\*[ \t]*(\S*)", qs_text, re.MULTILINE)
    if not found:
        found = re.findall(r"^- \*\*status:\*\*[ \t]*`?([A-Za-z]+)`?", qs_text, re.IGNORECASE | re.MULTILINE)
    if not found:
        for line in qs_text.strip().splitlines():
            clean = line.strip("`*-_ \t#")
            words = clean.split()
            if words and words[0] in STATES:
                found = [words[0]]
                break
    return (found[0] if found else None), len(found)


def rule_ids(text: str) -> list[str]:
    return re.findall(r"\*\*(R\d+)\b", text)


def example_ids(text: str) -> list[str]:
    return re.findall(r"\*\*(S\d+)\b", text)


def list_items(body: str) -> list[str]:
    return [l.strip() for l in strip_comments(body).splitlines() if re.match(r"\s*(-|\d+\.)\s+\S", l)]


def tasks_of(tasks_text: str) -> list[tuple[bool, str]]:
    return [
        (m.group(1).lower() == "x", m.group(2))
        for m in re.finditer(r"^\s*- \[( |x|X)\]\s+(.*)$", strip_comments(tasks_text), re.MULTILINE)
    ]


# ------------------------------------------------------------ workspace -----

def features(ws: Path) -> list[Path]:
    fdir = ws / "features"
    return sorted(p for p in fdir.iterdir() if p.is_dir()) if fdir.is_dir() else []


def epics(feature_dir: Path) -> list[Path]:
    edir = feature_dir / "epics"
    if edir.is_dir():
        return sorted(p for p in edir.iterdir() if p.is_dir())
    return sorted(
        p for p in feature_dir.iterdir()
        if p.is_dir() and (p.name.startswith("epic") or (p / "quick_status.md").is_file() or (p / "plan.md").is_file())
    )


def resolve_epic(ws: Path, ref: str) -> Path:
    parts = ref.strip("/").split("/")
    if len(parts) != 2:
        die(f"epic reference must be <feature>/<epic>, got '{ref}'.")
    path = ws / "features" / parts[0] / "epics" / parts[1]
    if not path.is_dir():
        path = ws / "features" / parts[0] / parts[1]
    if not path.is_dir():
        die(f"epic not found: features/{parts[0]}/{parts[1]} (or under epics/)")
    return path


def epic_line(epic_slug: str, state: str) -> str:
    return f"- [{epic_slug}](epics/{epic_slug}/index.md) — {state}"


def write_state(epic_dir: Path, new: str, who: str, note: str) -> str:
    qs_path = epic_dir / "quick_status.md"
    text = read(qs_path)
    old, count = epic_state(text)
    if count == 0:
        die(f"{qs_path}: expected state line, found none.")
    if re.search(r"^- \*\*state:\*\*", text, re.MULTILINE):
        text = re.sub(r"^(- \*\*state:\*\*)[ \t]*\S*", rf"\g<1> {new}", text, count=1, flags=re.MULTILINE)
    else:
        text = f"- **state:** {new}\n\n" + text.strip() + "\n"
    entry = f"- {today()} — {old} → {new} by {who}." + (f" {note.strip()}" if note.strip() else "")
    text = append_to_section(text, "Log", entry)
    qs_path.write_text(text, encoding="utf-8")

    # keep the feature index epic list in sync
    feat_dir = epic_dir.parent.parent if epic_dir.parent.name == "epics" else epic_dir.parent
    feat_index = feat_dir / "index.md"
    if feat_index.is_file():
        ftext = read(feat_index)
        slug = epic_dir.name
        pattern = rf"^- \[{re.escape(slug)}\]\((?:epics/)?{re.escape(slug)}/index\.md\).*$"
        if re.search(pattern, ftext, re.MULTILINE):
            ftext = re.sub(pattern, epic_line(slug, new), ftext, count=1, flags=re.MULTILINE)
            feat_index.write_text(ftext, encoding="utf-8")
    return old or "?"


def git(ws: Path, *args: str) -> str | None:
    try:
        out = subprocess.run(["git", "-C", str(ws), *args], capture_output=True, text=True, check=True)
        return out.stdout
    except (subprocess.CalledProcessError, FileNotFoundError):
        return None


def looks_stale(ws: Path, epic_dir: Path) -> bool:
    """A Done epic whose plan.md changed after its quick_status.md (git-based)."""
    if git(ws, "rev-parse", "--is-inside-work-tree") is None:
        return False
    plan, qs = epic_dir / "plan.md", epic_dir / "quick_status.md"
    plan_dirty = bool((git(ws, "status", "--porcelain", "--", str(plan)) or "").strip())
    qs_dirty = bool((git(ws, "status", "--porcelain", "--", str(qs)) or "").strip())
    if plan_dirty and not qs_dirty:
        return True
    if plan_dirty or qs_dirty:
        return False
    try:
        plan_ts = int((git(ws, "log", "-1", "--format=%ct", "--", str(plan)) or "0").strip() or 0)
        qs_ts = int((git(ws, "log", "-1", "--format=%ct", "--", str(qs)) or "0").strip() or 0)
    except ValueError:
        return False
    return plan_ts > qs_ts


# ------------------------------------------------------------ subcommands ---

def cmd_init(args) -> int:
    base = Path(args.path).expanduser().resolve() if args.path else Path.cwd().resolve()
    check_slug(args.name, "product")
    dest = base / args.name
    if dest.exists() and any(dest.iterdir()):
        die(f"{dest} already exists and is not empty. Aborting to avoid overwriting.")

    apps = []
    for spec in args.app or []:
        name, _, kind = spec.partition(":")
        check_slug(name, "app")
        apps.append((name, kind or "backend-api"))

    apps_list = "\n".join(f"- [{n}](apps/{n}/app.md) — {k}" for n, k in apps) or "_No apps yet._"
    values = {"product_name": args.name, "date": today(), "apps_list": apps_list}

    for name in ROOT_FILES + ["CLAUDE.md"]:
        write_new(dest / name, render(TEMPLATES_DIR / "product" / name, values))
    for d in ROOT_DIRS:
        (dest / d).mkdir(parents=True, exist_ok=True)
    (dest / "assets" / ".gitkeep").touch()
    (dest / "features" / ".gitkeep").touch()
    if not apps:
        (dest / "apps" / ".gitkeep").touch()
    for name, kind in apps:
        write_new(
            dest / "apps" / name / "app.md",
            render(TEMPLATES_DIR / "app" / "app.md", {**values, "app_name": name, "app_type": kind}),
        )

    git_msg = "git: skipped (--no-git)"
    if not args.no_git:
        try:
            subprocess.run(["git", "init", "-q", str(dest)], check=True)
            git_msg = "git: repository initialized (nothing committed yet)"
        except (subprocess.CalledProcessError, FileNotFoundError):
            git_msg = "git: not available — initialize the repository manually"

    print(f"✅ SCPE workspace created at {dest}")
    print(f"   {git_msg}")
    print("   Next: Upstream — fill product_vision.md, glossary.md and technical_deal.md,")
    print("   then open the most critical feature with `scpe.py feature <slug> \"<Title>\"`.")
    return 0


def cmd_feature(args) -> int:
    ws = workspace()
    check_slug(args.slug, "feature")
    dest = ws / "features" / args.slug
    if dest.exists():
        die(f"features/{args.slug} already exists. Aborting to avoid overwriting.")
    values = {"feature_slug": args.slug, "feature_title": args.title, "date": today()}
    for name in FEATURE_FILES:
        write_new(dest / name, render(TEMPLATES_DIR / "feature" / name, values))
    (dest / "epics").mkdir(parents=True, exist_ok=True)

    index = ws / "index.md"
    line = f"- [{args.title}](features/{args.slug}/index.md)"
    index.write_text(append_to_section(read(index), "Features", line, "_No features yet._"), encoding="utf-8")
    print(f"✅ Feature created at features/{args.slug}/ and linked in index.md")
    return 0


def cmd_epic(args) -> int:
    ws = workspace()
    check_slug(args.slug, "epic")
    feature_dir = ws / "features" / args.feature
    if not (feature_dir / "index.md").is_file():
        die(f"feature '{args.feature}' not found. Create it first with `scpe.py feature`.")
    dest = feature_dir / "epics" / args.slug
    if dest.exists():
        die(f"features/{args.feature}/epics/{args.slug} already exists. Aborting to avoid overwriting.")
    bc = field(read(feature_dir / "index.md"), "bounded_context") or ""
    values = {
        "feature_slug": args.feature, "epic_slug": args.slug, "epic_title": args.title,
        "bounded_context": bc, "date": today(),
    }
    for name in EPIC_FILES:
        write_new(dest / name, render(TEMPLATES_DIR / "epic" / name, values))

    index = feature_dir / "index.md"
    index.write_text(
        append_to_section(read(index), "Epics", epic_line(args.slug, "Draft"), "_No epics yet._"),
        encoding="utf-8",
    )
    print(f"✅ Epic created at features/{args.feature}/epics/{args.slug}/ (state: Draft)")
    return 0


def cmd_validate(args) -> int:
    ws = workspace()
    errors: list[str] = []
    warnings: list[str] = []
    rel = lambda p: str(p.relative_to(ws))  # noqa: E731

    for name in ROOT_FILES:
        if not (ws / name).is_file():
            hint = ""
            if name == "technical_deal.md" and (ws / "techinal_deal.md").is_file():
                hint = " (found legacy techinal_deal.md — rename it: git mv techinal_deal.md technical_deal.md)"
            errors.append(f"missing product root file: {name}{hint}")
    for d in ROOT_DIRS:
        if not (ws / d).is_dir():
            errors.append(f"missing product root folder: {d}/")

    # apps
    apps_dir = ws / "apps"
    for app in sorted(p for p in apps_dir.iterdir() if p.is_dir()) if apps_dir.is_dir() else []:
        text = read(app / "app.md")
        if not text:
            errors.append(f"{rel(app)}: missing app.md")
            continue
        for f in APP_FIELDS:
            if field(text, f) is None:
                errors.append(f"{rel(app)}/app.md: missing field '{f}'")
        if section(text, "Boundaries") is None:
            errors.append(f"{rel(app)}/app.md: missing '## Boundaries'")
        pointer = app / "repo_pointer.md"
        if pointer.is_file() and not field(read(pointer), "repository"):
            errors.append(f"{rel(pointer)}: missing value for 'repository'")

    # features and epics
    for feat in features(ws):
        for name in FEATURE_FILES:
            if not (feat / name).is_file():
                errors.append(f"{rel(feat)}: missing {name}")
        if not (feat / "epics").is_dir():
            errors.append(f"{rel(feat)}: missing epics/")
        ftext = read(feat / "index.md")
        for f in FEATURE_FIELDS:
            if field(ftext, f) is None:
                errors.append(f"{rel(feat)}/index.md: missing field '{f}'")

        listed = dict(re.findall(r"^- \[([^\]]+)\]\(epics/[^)]+/index\.md\)\s*—\s*(\S+)", ftext, re.MULTILINE))
        on_disk = {e.name for e in epics(feat)}
        for missing in sorted(set(listed) - on_disk):
            errors.append(f"{rel(feat)}/index.md: lists epic '{missing}' that does not exist")

        for ep in epics(feat):
            where = rel(ep)
            for name in EPIC_FILES:
                if not (ep / name).is_file():
                    errors.append(f"{where}: missing {name}")
            state, count = epic_state(read(ep / "quick_status.md"))
            if count != 1:
                errors.append(f"{where}/quick_status.md: expected exactly one state line, found {count}")
                continue
            if state not in STATES:
                errors.append(f"{where}: invalid state '{state}' (valid: {', '.join(STATES)})")
                continue
            if ep.name not in listed:
                errors.append(f"{rel(feat)}/index.md: epic '{ep.name}' is not listed under ## Epics")
            elif listed[ep.name] != state:
                errors.append(f"{rel(feat)}/index.md: epic '{ep.name}' listed as {listed[ep.name]} but state is {state}")

            plan = read(ep / "plan.md")
            tasks = tasks_of(read(ep / "tasks.md"))
            examples = sorted(set(example_ids(strip_comments(section(plan, "Examples") or ""))), key=lambda s: int(s[1:]))

            if state in {"WIP", "Done", "Stale"}:
                if not tasks:
                    errors.append(f"{where}/tasks.md: no tasks for an epic in {state}")
                for _, t in tasks:
                    if not re.search(r"\b[SR]\d+\b", t):
                        errors.append(f"{where}/tasks.md: task does not cite S#/R#: '{t}'")
                cited = set(re.findall(r"\b(S\d+)\b", " ".join(t for _, t in tasks)))
                for s in examples:
                    if s not in cited:
                        (warnings if state == "WIP" else errors).append(f"{where}: example {s} is not covered by any task")
            if state == "Done":
                open_tasks = [t for done, t in tasks if not done]
                if open_tasks:
                    errors.append(f"{where}: state Done but {len(open_tasks)} task(s) still open")
                if looks_stale(ws, ep):
                    errors.append(f"{where}: plan.md changed after the epic was Done — it should be Stale (§4.5)")
                if args.strict:
                    corpus = "\n".join(
                        read(p) for p in (ws / "apps").rglob("*")
                        if p.is_file()
                        and p.name not in {"app.md", "repo_pointer.md"}
                        and not any(part in {"target", "node_modules", "dist", ".git"} for part in p.parts)
                        and p.stat().st_size < 2_000_000
                    ) if (ws / "apps").is_dir() else ""
                    for s in examples:
                        if f"{ep.name}#{s}" not in corpus:
                            errors.append(f"{where}: no test named '{ep.name}#{s}' found under apps/")
            if state == "Blocked":
                blockers = strip_comments(section(read(ep / "quick_status.md"), "Blockers") or "").strip()
                if not blockers or blockers.lower().startswith("none"):
                    warnings.append(f"{where}: state Blocked but ## Blockers is empty")

    if args.strict:
        errors.extend(warnings)
        warnings = []
    for w in warnings:
        print(f"⚠️  {w}")
    for e in errors:
        print(f"❌ {e}")
    if errors:
        print(f"\n{len(errors)} error(s), {len(warnings)} warning(s).")
        return 1
    print(f"✅ Workspace is SCPE-compliant ({len(warnings)} warning(s)).")
    return 0


def cmd_gate_check(args) -> int:
    ws = workspace()
    ep = resolve_epic(ws, args.epic)
    plan = read(ep / "plan.md")
    state, _ = epic_state(read(ep / "quick_status.md"))
    results: list[tuple[str, str]] = []

    def check(ok: bool, msg: str, level: str = "FAIL") -> None:
        results.append(("PASS" if ok else level, msg))

    check(state in {"Draft", "Stale", "Blocked"}, f"epic state is {state} (gate applies to Draft, Stale or Blocked)", "WARN")

    for heading in ["Intent", "Domain Model", "Business Rules", "Examples", "Slices", "Open Questions"]:
        check(section(plan, heading) is not None, f"plan.md has '## {heading}'")

    intent = strip_comments(section(plan, "Intent") or "").strip()
    check(bool(intent), "Intent is written")

    dm = strip_comments(section(plan, "Domain Model") or "")
    terms_raw = field(dm, "Terms") or ""
    terms = [re.sub(r"\(.*?\)", "", t).strip() for t in terms_raw.split(",")]
    terms = [t for t in terms if t]
    check(bool(terms), "Domain Model lists Terms")
    glossary_heads = {h.strip().lower() for h in re.findall(r"^## (.+)$", read(ws / "glossary.md"), re.MULTILINE)}
    for t in terms:
        check(t.lower() in glossary_heads, f"term '{t}' is defined in glossary.md")

    rules = rule_ids(strip_comments(section(plan, "Business Rules") or ""))
    check(bool(rules), f"Business Rules numbered R# ({len(rules)} found)")

    ex_items = [l for l in list_items(section(plan, "Examples") or "") if re.search(r"\*\*S\d+", l)]
    check(bool(ex_items), f"Examples numbered S# ({len(ex_items)} found)")
    for item in ex_items:
        sid = example_ids(item)[0]
        whens = len(re.findall(r"\bWhen\b", item))
        check("Given" in item and whens == 1 and "Then" in item,
              f"{sid} has Given / exactly one When / Then")

    examples = set(example_ids(strip_comments(section(plan, "Examples") or "")))
    slices = list_items(section(plan, "Slices") or "")
    check(bool(slices), f"Slices defined ({len(slices)} found)")
    cited: set[str] = set()
    for i, s in enumerate(slices, 1):
        refs = set(re.findall(r"\b([SR]\d+)\b", s))
        cited |= refs
        check(bool(refs), f"slice {i} cites S#/R#")
    for s in sorted(examples - cited, key=lambda x: int(x[1:])):
        check(False, f"example {s} is not assigned to any slice", "WARN")

    oq = [q for q in list_items(section(plan, "Open Questions") or "")
          if not re.search(r"\(non-blocking\)|\[resolved\]", q, re.IGNORECASE)]
    check(not oq, f"no blocking open questions ({len(oq)} blocking)")
    for q in oq:
        results.append(("FAIL", f"blocking question: {q.lstrip('-0123456789. ')}"))

    results.append(("MANUAL", "slices are vertical, or the reason they are not is written under ## Slices"))
    results.append(("MANUAL", "model fits architecture.md and technical_deal.md"))
    results.append(("MANUAL", "every Then is observable by a user or another system"))

    icons = {"PASS": "✅", "FAIL": "❌", "WARN": "⚠️ ", "MANUAL": "👤"}
    print(f"Readiness Gate check — {args.epic}\n")
    for level, msg in results:
        print(f"{icons[level]} {level:<6} {msg}")
    fails = sum(1 for l, _ in results if l == "FAIL")
    print(f"\n{'FAIL' if fails else 'PASS'}: {fails} blocking issue(s). "
          "Only a human acting as Tech Lead may mark the epic Ready.")
    return 1 if fails else 0


def cmd_set_state(args) -> int:
    ws = workspace()
    ep = resolve_epic(ws, args.epic)
    if args.state not in STATES:
        die(f"invalid state '{args.state}' (valid: {', '.join(STATES)})")
    current, _ = epic_state(read(ep / "quick_status.md"))
    if current not in TRANSITIONS:
        die(f"current state '{current}' is invalid; fix quick_status.md first.")
    if args.state not in TRANSITIONS[current]:
        allowed = ", ".join(sorted(TRANSITIONS[current])) or "none"
        die(f"transition {current} → {args.state} is not allowed (from {current}: {allowed}).")
    write_state(ep, args.state, args.by, args.note or "")
    print(f"✅ {args.epic}: {current} → {args.state}")
    return 0


def cmd_mark_stale(args) -> int:
    path_str = args.path
    if args.from_hook:
        try:
            payload = json.load(sys.stdin)
            path_str = (payload.get("tool_input") or {}).get("file_path")
        except (json.JSONDecodeError, AttributeError):
            return 0
    if not path_str:
        return 0
    try:
        plan = Path(path_str).expanduser().resolve()
        if plan.name != "plan.md" or plan.parent.parent.name != "epics":
            return 0
        epic_dir = plan.parent
        ws = find_workspace(epic_dir)
        if ws is None:
            return 0
        state, count = epic_state(read(epic_dir / "quick_status.md"))
        if count != 1 or state != "Done":
            return 0
        write_state(epic_dir, "Stale", "scpe hook",
                    "plan.md edited after delivery; back to the Readiness Gate (spec drift).")
    except Exception as exc:  # a hook must never break the edit that triggered it
        if not args.from_hook:
            die(str(exc))
        return 0

    ref = f"{epic_dir.parent.parent.name}/{epic_dir.name}"
    msg = (f"SCPE: epic {ref} was Done and its plan.md changed, so it is now Stale. "
           "The Tech Lead must review the difference and record Re-execute or Accept the drift "
           "in epic_roadmap.md (run /scpe-gate).")
    if args.from_hook:
        print(json.dumps({
            "systemMessage": msg,
            "hookSpecificOutput": {"hookEventName": "PostToolUse", "additionalContext": msg},
        }))
    else:
        print(f"✅ {msg}")
    return 0


def collect_status(ws: Path) -> list[dict]:
    rows = []
    for feat in features(ws):
        for ep in epics(feat):
            state, _ = epic_state(read(ep / "quick_status.md"))
            tasks = tasks_of(read(ep / "tasks.md"))
            blockers = strip_comments(section(read(ep / "quick_status.md"), "Blockers") or "").strip()
            rows.append({
                "feature": feat.name, "epic": ep.name, "state": state or "?",
                "rel_path": str(ep.relative_to(ws)),
                "tasks_done": sum(1 for d, _ in tasks if d), "tasks_total": len(tasks),
                "blockers": "" if blockers.lower().startswith("none") else blockers,
            })
    return rows


def status_table(rows: list[dict]) -> str:
    if not rows:
        return "_No epics yet._"
    lines = ["| Feature | Epic | State | Tasks |", "| :--- | :--- | :---: | :---: |"]
    for r in rows:
        tasks = f"{r['tasks_done']}/{r['tasks_total']}" if r["tasks_total"] else "—"
        rel_epic = r.get("rel_path", f"features/{r['feature']}/epics/{r['epic']}")
        lines.append(
            f"| [{r['feature']}](features/{r['feature']}/index.md) "
            f"| [{r['epic']}]({rel_epic}/quick_status.md) "
            f"| `{r['state']}` | {tasks} |"
        )
    counts = {s: sum(1 for r in rows if r["state"] == s) for s in STATES}
    lines.append("")
    lines.append(" · ".join(f"**{s}:** {n}" for s, n in counts.items()))
    return "\n".join(lines)


def cmd_status(args) -> int:
    ws = workspace()
    rows = collect_status(ws)
    if args.json:
        print(json.dumps(rows, indent=2, ensure_ascii=False))
    else:
        print(status_table(rows))
    if args.write:
        qs = ws / "quick_status.md"
        text = read(qs)
        block = f"{STATUS_START}\n{status_table(rows)}\n{STATUS_END}"
        if STATUS_START in text and STATUS_END in text:
            text = re.sub(re.escape(STATUS_START) + r".*?" + re.escape(STATUS_END),
                          lambda _: block, text, flags=re.DOTALL)
        else:
            text = append_to_section(text, "Epics", block)
        text = re.sub(r"^(- \*\*updated:\*\*).*$", rf"\g<1> {today()}", text, count=1, flags=re.MULTILINE)
        qs.write_text(text, encoding="utf-8")
        if not args.json:
            print("\n✅ quick_status.md updated")
    return 0


def cmd_complete_task(args) -> int:
    ws = workspace()
    epic_dir = resolve_epic(ws, args.epic)
    tasks_path = epic_dir / "tasks.md"
    if not tasks_path.is_file():
        die(f"tasks.md not found at {tasks_path}")
    text = read(tasks_path)
    lines = text.splitlines()
    target = args.task.strip()
    updated = False

    new_lines = []
    task_idx = 0
    matched_desc = ""
    for line in lines:
        m = re.match(r"^(\s*-\s*\[)( |x|X)(\]\s+)(.*)$", line)
        if m:
            task_idx += 1
            prefix, state_char, mid, rest = m.groups()
            is_match = False
            if target.isdigit() and int(target) == task_idx:
                is_match = True
            elif target.lower() in rest.lower() or target in line:
                is_match = True

            if is_match:
                matched_desc = rest.strip()
                if state_char == " ":
                    line = f"{prefix}x{mid}{rest}"
                    updated = True
                else:
                    print(f"ℹ️ Task already marked as done: {matched_desc}")
                    return 0
        new_lines.append(line)

    if not updated:
        die(f"No pending task matched '{target}' in {tasks_path}")

    tasks_path.write_text("\n".join(new_lines) + "\n", encoding="utf-8")
    print(f"✅ Marked task as complete: {matched_desc}")
    return 0


# ------------------------------------------------------------------ main ----

def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(prog="scpe.py", description="SCPE workspace engine")
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("init", help="create a new SCPE product workspace")
    p.add_argument("name")
    p.add_argument("--path", help="parent directory (default: cwd)")
    p.add_argument("--app", action="append", metavar="NAME:TYPE", help="add apps/<NAME>/app.md")
    p.add_argument("--no-git", action="store_true", help="do not run git init")
    p.set_defaults(func=cmd_init)

    p = sub.add_parser("feature", help="scaffold features/<slug>/")
    p.add_argument("slug")
    p.add_argument("title")
    p.set_defaults(func=cmd_feature)

    p = sub.add_parser("epic", help="scaffold features/<feature>/epics/<slug>/ in Draft")
    p.add_argument("feature")
    p.add_argument("slug")
    p.add_argument("title")
    p.set_defaults(func=cmd_epic)

    p = sub.add_parser("complete-task", help="mark a task in tasks.md as done [x]")
    p.add_argument("epic", help="<feature>/<epic>")
    p.add_argument("task", help="task number (1-based) or substring/ID (e.g. TASK-01.1.1)")
    p.set_defaults(func=cmd_complete_task)

    p = sub.add_parser("validate", help="check structure, states and traceability")
    p.add_argument("--strict", action="store_true", help="also require <epic>#S# tests; warnings fail")
    p.set_defaults(func=cmd_validate)

    p = sub.add_parser("gate-check", help="Readiness Gate lint for one epic (never changes state)")
    p.add_argument("epic", help="<feature>/<epic>")
    p.set_defaults(func=cmd_gate_check)

    p = sub.add_parser("set-state", help="apply a valid state transition and log it")
    p.add_argument("epic", help="<feature>/<epic>")
    p.add_argument("state", choices=STATES)
    p.add_argument("--by", required=True, help="who performs the transition")
    p.add_argument("--note", default="")
    p.set_defaults(func=cmd_set_state)

    p = sub.add_parser("mark-stale", help="Done → Stale when an epic plan.md is edited")
    p.add_argument("path", nargs="?")
    p.add_argument("--from-hook", action="store_true", help="read PostToolUse JSON from stdin")
    p.set_defaults(func=cmd_mark_stale)

    p = sub.add_parser("status", help="product status panel")
    p.add_argument("--json", action="store_true")
    p.add_argument("--write", action="store_true", help="rewrite the table in quick_status.md")
    p.set_defaults(func=cmd_status)

    args = ap.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env bash
# Fail if machine attribution appears in commit messages or in the tree.
#
#   check-attribution.sh              scan working tree + full history
#   check-attribution.sh --staged     scan staged changes only (pre-commit)
#   check-attribution.sh --history    scan commit messages only
#   check-attribution.sh --quiet      exit code only
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# Attribution markers. Kept as one extended-regex alternation so grep is fast
# and the same definition drives the hook, the CLI and CI.
PATTERN='Co-[Aa]uthored-[Bb]y:[[:space:]]*(Claude|.*noreply@anthropic\.com)'
PATTERN="$PATTERN|[Gg]enerated with .{0,12}[Cc]laude [Cc]ode"
PATTERN="$PATTERN|[Gg]enerated by .{0,12}[Cc]laude"
PATTERN="$PATTERN|🤖 Generated"
PATTERN="$PATTERN|https://claude\.ai/code"

mode="all"; quiet=0
for arg in "$@"; do
  case "$arg" in
    --staged)  mode="staged" ;;
    --history) mode="history" ;;
    --tree)    mode="tree" ;;
    --quiet)   quiet=1 ;;
    -h|--help) sed -n '2,8p' "$0"; exit 0 ;;
    *) echo "unknown option: $arg" >&2; exit 2 ;;
  esac
done

say() { [ "$quiet" = "1" ] || printf '%s\n' "$*" >&2; }
found=0

# The policy's own definition lives in these files, so they are exempt from it.
EXEMPT_PATHS=(
  ':!scripts/check-attribution.sh'
  ':!.githooks/commit-msg'
  ':!.github/workflows/no-attribution.yml'
)

scan_tree() {
  local hits
  hits="$(git grep -n -I -E "$PATTERN" -- "${EXEMPT_PATHS[@]}" 2>/dev/null || true)"
  if [ -n "$hits" ]; then
    say "attribution markers in tracked files:"
    say "$hits"
    found=1
  fi
}

scan_staged() {
  local hits
  hits="$(git diff --cached --unified=0 -- "${EXEMPT_PATHS[@]}" \
    | grep -E '^\+' | grep -E "$PATTERN" || true)"
  if [ -n "$hits" ]; then
    say "attribution markers in staged changes:"
    say "$hits"
    found=1
  fi
}

scan_history() {
  # A fresh repository has no commits to scan.
  git rev-parse --verify --quiet HEAD >/dev/null || return 0
  local hits
  hits="$(git log --format='%H%n%B%n--' | grep -E "$PATTERN" || true)"
  if [ -n "$hits" ]; then
    say "attribution markers in commit messages:"
    say "$hits"
    found=1
  fi
  local trailers
  trailers="$(git log --format='%an <%ae>%n%cn <%ce>' 2>/dev/null \
    | grep -i -E 'anthropic\.com|^claude ' || true)"
  if [ -n "$trailers" ]; then
    say "machine identity in author/committer fields:"
    say "$trailers"
    found=1
  fi
}

case "$mode" in
  staged)  scan_staged ;;
  history) scan_history ;;
  tree)    scan_tree ;;
  all)     scan_tree; scan_history ;;
esac

[ "$found" = "0" ] || exit 1
say "clean: no attribution markers found"

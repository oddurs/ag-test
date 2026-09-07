#!/usr/bin/env bash
# Shared helpers for the agent worktree workflow.
# Sourced by bin/agent and scripts/*. Bash 3.2 compatible.

set -euo pipefail

# ---------------------------------------------------------------- output ----

_tty() { [ -t 2 ]; }
if _tty && [ -z "${NO_COLOR:-}" ]; then
  C_DIM=$'\033[2m'; C_RED=$'\033[31m'; C_GRN=$'\033[32m'
  C_YEL=$'\033[33m'; C_BLU=$'\033[34m'; C_OFF=$'\033[0m'
else
  C_DIM=''; C_RED=''; C_GRN=''; C_YEL=''; C_BLU=''; C_OFF=''
fi

log()  { printf '%s==>%s %s\n' "$C_BLU" "$C_OFF" "$*" >&2; }
ok()   { printf '%s ok %s %s\n' "$C_GRN" "$C_OFF" "$*" >&2; }
warn() { printf '%swarn%s %s\n' "$C_YEL" "$C_OFF" "$*" >&2; }
die()  { printf '%sfail%s %s\n' "$C_RED" "$C_OFF" "$*" >&2; exit 1; }
dim()  { printf '%s%s%s\n' "$C_DIM" "$*" "$C_OFF" >&2; }

have() { command -v "$1" >/dev/null 2>&1; }

# ------------------------------------------------------------- repo paths ---

# Resolve the *main* worktree root even when invoked from inside a linked
# worktree: --git-common-dir always points at the primary .git directory.
resolve_repo() {
  git rev-parse --is-inside-work-tree >/dev/null 2>&1 \
    || die "not inside a git repository"
  GIT_COMMON_DIR="$(git rev-parse --path-format=absolute --git-common-dir)"
  MAIN_ROOT="$(dirname "$GIT_COMMON_DIR")"
  export GIT_COMMON_DIR MAIN_ROOT
}

# --------------------------------------------------------------- defaults ---

# Every knob, in one place. Used to enforce configuration precedence:
#   environment > .agent/config.local.sh > agent.config.sh > built-in defaults
AGENT_KNOBS="AGENT_WORKTREE_ROOT AGENT_STATE_DIR AGENT_LOG_DIR AGENT_BRANCH_PREFIX
AGENT_BASE_BRANCH AGENT_JOBS AGENT_CLI AGENT_CLI_ARGS AGENT_SETUP_CMD
AGENT_VERIFY_CMD AGENT_AUTO_PUSH AGENT_AUTO_PR AGENT_PR_DRAFT"

load_config() {
  resolve_repo

  # Stash whatever the environment supplied; config files are allowed to assign
  # plainly, and the environment is restored over the top of them afterwards.
  local knob
  for knob in $AGENT_KNOBS; do
    eval "_AGENT_ENV_$knob=\${$knob:-}"
  done

  AGENT_STATE_DIR="${AGENT_STATE_DIR:-$MAIN_ROOT/.agent}"

  # shellcheck disable=SC1091
  [ -f "$MAIN_ROOT/agent.config.sh" ] && . "$MAIN_ROOT/agent.config.sh"
  # shellcheck disable=SC1091
  [ -f "$AGENT_STATE_DIR/config.local.sh" ] && . "$AGENT_STATE_DIR/config.local.sh"

  for knob in $AGENT_KNOBS; do
    eval "if [ -n \"\${_AGENT_ENV_$knob}\" ]; then $knob=\"\${_AGENT_ENV_$knob}\"; fi"
    eval "unset _AGENT_ENV_$knob"
  done

  # Built-in defaults fill whatever nobody set.
  AGENT_WORKTREE_ROOT="${AGENT_WORKTREE_ROOT:-$MAIN_ROOT/.worktrees}"
  AGENT_STATE_DIR="${AGENT_STATE_DIR:-$MAIN_ROOT/.agent}"
  AGENT_LOG_DIR="${AGENT_LOG_DIR:-$AGENT_STATE_DIR/logs}"
  AGENT_BRANCH_PREFIX="${AGENT_BRANCH_PREFIX:-agent/}"
  AGENT_JOBS="${AGENT_JOBS:-4}"
  AGENT_CLI="${AGENT_CLI:-claude}"
  # Full-auto: headless, no permission prompts.
  AGENT_CLI_ARGS="${AGENT_CLI_ARGS:---permission-mode bypassPermissions}"
  AGENT_SETUP_CMD="${AGENT_SETUP_CMD:-}"
  AGENT_VERIFY_CMD="${AGENT_VERIFY_CMD:-}"
  AGENT_AUTO_PUSH="${AGENT_AUTO_PUSH:-1}"
  AGENT_AUTO_PR="${AGENT_AUTO_PR:-1}"
  AGENT_PR_DRAFT="${AGENT_PR_DRAFT:-0}"
  AGENT_BASE_BRANCH="${AGENT_BASE_BRANCH:-$(default_base_branch)}"

  mkdir -p "$AGENT_LOG_DIR"
}

default_base_branch() {
  local ref
  ref="$(git -C "$MAIN_ROOT" symbolic-ref --quiet refs/remotes/origin/HEAD 2>/dev/null || true)"
  if [ -n "$ref" ]; then
    printf '%s\n' "${ref#refs/remotes/origin/}"
    return
  fi
  for b in main master; do
    if git -C "$MAIN_ROOT" show-ref --verify --quiet "refs/heads/$b"; then
      printf '%s\n' "$b"; return
    fi
  done
  # Works on an unborn branch too, unlike `rev-parse --abbrev-ref HEAD`.
  git -C "$MAIN_ROOT" symbolic-ref --short HEAD 2>/dev/null || echo main
}

# ------------------------------------------------------------------ slugs ---

# Lowercase, alnum + dashes, collapsed, trimmed, capped. Bash 3.2 safe.
slugify() {
  printf '%s' "$*" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -e 's/[^a-z0-9]\{1,\}/-/g' -e 's/^-*//' -e 's/-*$//' \
    | cut -c1-48
}

branch_for()   { printf '%s%s\n' "$AGENT_BRANCH_PREFIX" "$1"; }
worktree_for() { printf '%s/%s\n' "$AGENT_WORKTREE_ROOT" "$1"; }
log_for()      { printf '%s/%s.log\n' "$AGENT_LOG_DIR" "$1"; }

require_slug() {
  [ -n "${1:-}" ] || die "missing <task> argument"
  case "$1" in
    -*) die "invalid task name: $1" ;;
    */*|*..*) die "invalid task name: $1" ;;
  esac
}

worktree_exists() { [ -d "$(worktree_for "$1")" ]; }

require_worktree() {
  worktree_exists "$1" || die "no worktree for '$1' (try: agent new $1)"
}

# Slugs of every managed worktree, one per line.
list_slugs() {
  [ -d "$AGENT_WORKTREE_ROOT" ] || return 0
  find "$AGENT_WORKTREE_ROOT" -mindepth 1 -maxdepth 1 -type d \
    | sed 's#.*/##' | LC_ALL=C sort
}

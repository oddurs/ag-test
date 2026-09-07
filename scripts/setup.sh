#!/usr/bin/env bash
# One-shot repo bootstrap: hooks, git config, agent state dirs.
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

echo "==> installing git hooks"
git config core.hooksPath .githooks

echo "==> hardening commit metadata"
# Never let a tool inject its own identity into commits from this repo.
git config commit.cleanup strip
git config --unset-all trailer.co-authored-by.command 2>/dev/null || true

echo "==> creating agent state dirs"
mkdir -p .agent/logs .worktrees

if [ ! -f .agent/config.local.sh ]; then
  cat > .agent/config.local.sh <<'LOCAL'
# Untracked local overrides for bin/agent. Everything here wins over
# agent.config.sh. See lib/common.sh for the full list of knobs.
# AGENT_JOBS=8
# AGENT_CLI_ARGS="--permission-mode bypassPermissions --model opus"
LOCAL
  echo "==> wrote .agent/config.local.sh"
fi

echo "==> verifying attribution policy"
./scripts/check-attribution.sh --quiet && echo "    clean"

echo
echo "ready. try:  ./bin/agent doctor"

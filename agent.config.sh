# Tracked defaults for the agent worktree workflow.
# Local, untracked overrides go in .agent/config.local.sh.
# shellcheck shell=bash disable=SC2034

# Where isolated worktrees live (gitignored, inside the repo).
AGENT_WORKTREE_ROOT="$MAIN_ROOT/.worktrees"

# Branch namespace for agent work.
AGENT_BRANCH_PREFIX="agent/"

# How many agents run at once under `agent fleet`.
AGENT_JOBS="${AGENT_JOBS:-4}"

# The coding agent, and the flags that make it unattended.
AGENT_CLI="${AGENT_CLI:-claude}"
AGENT_CLI_ARGS="${AGENT_CLI_ARGS:---permission-mode bypassPermissions}"

# Run once when a worktree is created.
AGENT_SETUP_CMD="${AGENT_SETUP_CMD:-}"

# The gate every task must pass before it can ship. This is the whole
# point of the workflow: the agent's work is only as good as this command.
AGENT_VERIFY_CMD="${AGENT_VERIFY_CMD:-cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all}"

# Push and open a PR automatically after a task verifies.
AGENT_AUTO_PUSH=1
AGENT_AUTO_PR=1
AGENT_PR_DRAFT=0

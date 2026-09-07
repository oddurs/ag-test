# Tracked defaults for the agent worktree workflow.
#
# Precedence: environment > .agent/config.local.sh > this file > built-ins.
# Assign plainly here; the environment is restored over the top afterwards.
# shellcheck shell=bash disable=SC2034

# How many agents run at once under `agent fleet`.
AGENT_JOBS=4

# The coding agent, and the flags that make it unattended.
AGENT_CLI=claude
AGENT_CLI_ARGS="--permission-mode bypassPermissions"

# The gate every task must pass before it can ship. This is the whole point of
# the workflow: the agent's work is worth exactly what this command is worth.
AGENT_VERIFY_CMD="cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all"

# Push and open a pull request automatically once a task verifies.
AGENT_AUTO_PUSH=1
AGENT_AUTO_PR=1
AGENT_PR_DRAFT=0

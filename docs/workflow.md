# The agent worktree workflow

Reference for `bin/agent`, the development harness this repo uses on itself.

Every task gets its own git worktree, its own branch, its own agent process and
its own log. Nothing is shared, so tasks can run flat-out in parallel without
seeing or clobbering each other's working tree.

```
repo/
├── .worktrees/            gitignored
│   ├── fix-shrinking/     branch agent/fix-shrinking
│   └── docs-index/        branch agent/docs-index
└── .agent/logs/           one log per task
```

## Setup

```bash
./scripts/setup.sh      # installs hooks, git config, state dirs
./bin/agent doctor      # verifies the environment
```

## Lifecycle

```bash
./bin/agent new fix-shrinking                 # worktree + branch off origin/main
./bin/agent run fix-shrinking "Make shrinking deterministic under a fixed seed."
./bin/agent verify fix-shrinking              # runs AGENT_VERIFY_CMD
./bin/agent ship fix-shrinking                # commit, push, open a PR
```

`auto` is the whole chain unattended:

```bash
./bin/agent auto fix-shrinking "Make shrinking deterministic under a fixed seed."
```

## Fleets

```bash
./bin/agent fleet tasks.example.txt -j 4
```

Task lines are `<name> :: <prompt>`. Each line runs the full `auto` chain in its
own worktree, four at a time. Failures are isolated — one task failing does not
stop the others, and every task's output is in `.agent/logs/<name>.log`.

Parallelism uses `xargs -P` rather than bash job control, because macOS ships
bash 3.2 and `wait -n` does not exist there.

## Inspection

```bash
./bin/agent ls              # task, branch, commits ahead, dirty/ready/clean
./bin/agent log NAME -f     # follow a running agent
cd "$(./bin/agent cd NAME)" # drop into a worktree
```

## Cleanup

```bash
./bin/agent rm NAME         # remove worktree, keep branch (--force if dirty)
./bin/agent gc              # remove merged worktrees and prune dead ones
```

## Configuration

Tracked defaults in [`agent.config.sh`](../agent.config.sh); untracked overrides
in `.agent/config.local.sh`. The full list of knobs is in
[`lib/common.sh`](../lib/common.sh).

The one that matters is `AGENT_VERIFY_CMD` — the gate every task must pass
before it can ship. In this repo:

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all
```

The workflow is worth exactly as much as that command is. Everything else is
plumbing.

## Attribution policy

No tool attribution reaches this repository — not in commit messages, not in
trailers, not in the tree. Three layers, because one is not enough:

1. **`.githooks/commit-msg`** strips attribution trailers, generator footers and
   robot-emoji lines from every commit message, and fails if the message was
   nothing but attribution.
2. **`.githooks/pre-commit`** rejects attribution markers in staged content.
3. **CI** re-runs [`scripts/check-attribution.sh`](../scripts/check-attribution.sh)
   over the full history and the whole tree, so a bypassed local hook is caught
   before merge.

`./bin/agent doctor` checks all three.

## Safety notes

`AGENT_CLI_ARGS` defaults to `--permission-mode bypassPermissions`. That is the
point of the workflow — the agent runs unattended — and it means an agent can do
anything to its worktree. The isolation is what makes that acceptable:

- a worktree is a throwaway directory on a throwaway branch;
- nothing merges without passing `AGENT_VERIFY_CMD`;
- nothing merges without a pull request a human reviews.

Do not point `AGENT_WORKTREE_ROOT` at anything you would mind losing, and do not
run fleets against a repository with production credentials in its environment.

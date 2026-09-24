# Subagent observability

Orbit understands the structured `AgentDetails` emitted by pi-subagents (including
`farefray/evidence-subagents`). No extension change or pi RPC patch is required.
Other tools keep their existing generic cards.

## Using it

1. Run an `Agent` tool in a project with pi-subagents enabled.
2. Expand the turn's activity group if it is folded. Agent cards show the reported
   activity, state, duration, tool count, and token summary.
3. Click **Inspect** to open a read-only right dock alongside the parent chat.
   It displays the task/prompt, model/configuration tags, agent/run IDs when sent,
   turn budget, estimated cost, task outcome, errors, and reported output.
4. **Close** or Escape dismisses the inspector. Opening Review or Explorer also
   dismisses it. Switching sessions cannot expose another session's observation.

The original Arguments/Output disclosure still works. Inspector text has a bounded
preview and a full-text Copy button. It does not execute tools, read transcript
files, switch child sessions, or mark results consumed.

## Session sidebar and saved history

Recognized child sessions live in a **Subagents (N)** disclosure beneath their
main ancestor. It starts collapsed, including for newly spawned children. Child
rows have an agent glyph and extra indentation. Nested children share their main
ancestor's group; proven children whose parent is absent have an explicit orphan
group. Collapsing the workspace hides its child groups as well.

Project counts, Show more limits, default recent/search results, and recent
workspace ordering use ordinary sessions only. Child activity never independently
promotes a main session. **Include subagents** in the command palette enables child
search; the same toggle is a keyboard-selectable palette command.

Clicking a child opens a separate **read-only saved transcript**, not a pi runtime.
The current main session stays running and its draft is preserved. Close, Escape,
or Back returns to it. This view has no composer, tool execution, result-consuming,
rename, or clone action. The palette suppresses main-runtime controls while it is
visible. Refresh explicitly re-reads the saved file off-thread; it is not a live
conversation stream. The virtualized plain-text preview follows saved `parentId`
links on the active branch, skips system prompts, does not load embedded images,
and is bounded to the first 8 MiB and 12,000 characters per message. **Copy preview**
copies the displayed message, not hidden/truncated text. Read failures are shown.

### Conservative session recognition

`session_origin.rs` never treats `parentSession` alone as a subagent marker: pi's
ordinary clones/forks use that field too. It accepts either:

- A version-1 custom `subagents:session` record before the first user message, with
  `data.sessionId` matching this file's header, `data.parentSession` matching its
  parent link, and a nonempty `data.agentType`. Identity binding rejects copied
  markers in cloned sessions. This is an optional supported contract; the current
  evidence-subagents fork does **not** yet emit it.
- Existing evidence-subagents data: a parent link and the **initial** assigned
  `type#short-id` session name, corroborated by that parent's `subagents:record` or
  structured Agent result ID. During an Agent-tool run, exact matching parent
  spawn arguments plus the fork's machine-added child context establish origin
  before its completion record exists. Renaming a child does not erase origin.

Parent lookups are restricted to files discovered in pi's session store. Each
parent is scanned once per discovery pass, off the UI thread, with a 32 MiB budget.
If evidence is missing, outside that budget, or unsupported, the session remains
an ordinary visible session rather than being guessed away. In particular, old
unmarked children with deleted parents cannot be reliably classified; programmatic
spawns may not be recognized until their completion record arrives. No fork,
installed package, user session file, or pi configuration is modified.

## Foreground card source of truth

- `tool_execution_update.partialResult.details` supplies foreground progress.
- Final tool results replace that observation; `get_messages` restores persisted
  details when reopening a session.
- Metadata lives in `ToolFacts`, so the existing live/snapshot reconciliation
  preserves it along with the tool's result. The inspector reads the same tool,
  not a separately maintained agent registry.
- Recognition requires `displayName`, `subagentType`, and `status` strings.
  Optional malformed fields are ignored. Unknown states are shown, never treated
  as success. Missing data stays absent.
- `spinnerFrame` is ignored and duration is quantized to seconds. Metadata-only
  updates are retained even when normalized tool output has not changed.
- Interrupted/reloaded `running` observations say **Last reported**, not that the
  child is currently alive. A generic terminal failure retains the last identity
  and usage, but clears the live claim. A background spawn is **Started in
  background**, never a completed child.
- Cost is the extension's estimate; zero/absent pricing is not shown as free.
  Task outcome remains separate from execution status.

## Deliberate limits

This is foreground tool observability, not yet an agent fleet manager. Background
launch metadata can be inspected, but subsequent background lifecycle events are
in-process extension events and do not automatically reach Orbit. No Stop/Steer
controls, live child conversation stream, activity history, file-change attribution,
or guessed transcript paths are exposed. Saved child-session browsing is separate
from the foreground card inspector and never asserts that a child is alive.

Next milestone: a versioned extension contract for session/agent/invocation
identity, lifecycle observations and transcript references, with compact durable
entries for recovery. Keep full token streams out of the parent session journal.
The bridge must preserve nested/workflow ownership and never consume results as a
side effect of inspection.

## Verification

- `cargo test -p orbit-pi agents`: tolerant parsing, progress correlation, concurrent
  children, final/snapshot reconciliation, error/exit handling, selection reset,
  and GPUI inspect/close and dock-width tests.
- `cargo test -p orbit-pi subagent`: origin/clone/rename tests, sidebar grouping,
  nested/cyclic families, main-only recent/search results, saved-branch parsing,
  bounded previews, and native read-only view/close interaction.
- `cargo test -p orbit-pi i18n`: localized copy.
- `cargo test -p orbit-pi debug_groups -- --ignored --nocapture`: optional read-only
  diagnostic of real session-store classification (prints session titles).
- On Windows with Git CRLF conversion enabled, run the suite with a process-local
  `core.autocrlf=false` Git override; the existing stash test expects LF bytes.
- Manual: run two foreground agents, inspect each while the parent remains open,
  collapse/expand raw details, interrupt a run, reload the session, open Review and
  Explorer, check a narrow window and Windows caption buttons.

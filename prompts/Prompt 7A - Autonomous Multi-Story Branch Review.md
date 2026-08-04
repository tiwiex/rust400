# Prompt 7A — Autonomous Multi-Story Stack Review

You are performing an independent review of a multi-story autonomous run that used an `end-of-loop` stacked-branch strategy.

This prompt is a companion to Prompt 7, not a replacement. Use Prompt 7 for ordinary single-story branches. Use Prompt 7A when reviewing a stack of interconnected PRs/branches generated from a single autonomous loop.

Load all context before forming an opinion. Do not trust the PR drafts until you compare them with the diffs, commits, logs, and verification evidence.

## Minimal Inputs

The human will provide the **Completion Note** generated at the end of the autonomous run at the bottom of this prompt.

It will look like this:

```text
Completed the end-of-loop autonomous build for US-CFG-05 through US-CFG-07.
Draft PRs opened:
Story	PR	Base
US-CFG-05	https://github.com/oshodi/Aspire/pull/94	main
US-CFG-06	https://github.com/oshodi/Aspire/pull/95	feature/US-CFG-05-global-configuration-bounds-management
...
```

---

## Mandatory Context Load

Read all of the following before reviewing implementation quality:

1. `AGENTS.md`
2. `brain/build-and-dev/reference-docs/canonical_decisions.md`
3. `brain/build-and-dev/specifications/deliverables/GITHUB_CONVENTIONS.md`
4. Fetch all remote branches mentioned in the completion note: `git fetch origin`.
5. The epic file for the story range (glob `brain/build-and-dev/specifications/deliverables/epics/`).
6. Every `STORY_LOG.md` for the story range, inferred from `brain/build-and-dev/build-logs/{EPIC-XX}/`.
7. Every micro-commit log for the story range.
8. The run note referenced in the completion note.

Timestamp rule: prefer full `YYYY-MM-DD HH:MM TZ` timestamps in story logs, gate records, experiment notes, verification evidence, and AI usage logs. Date-only entries are acceptable only when the exact time is unknown.

---

## Part 1: Per-PR Review Loop

For **every PR** listed in the completion note, sequentially from the bottom of the stack to the top, perform the following loop:

### 1. Execute Verification & Audits
- Check out the PR's specific branch (`git checkout <branch>`).
- Identify the Acceptance Criteria (ACs) for this specific story from the Epic spec.
- Run `verify.sh --ci` specifically on THIS branch.
- Run `alembic heads`. Confirm there is a single head for this branch.
- Analyze the diff specific to this layer: `git diff origin/<base>...origin/<branch>`.
- Perform the required audits against this diff: D8 tenant query audit, domain invariant audit,
  Auth/idempotency audit, RBAC audit, cache trust audit, Adapter audit, database constraint audit,
  and Migration audit.
- Review the micro-commits and the PR draft (`gh pr view <PR_NUMBER>` or read the `.md` draft).

Domain invariant audit means expanding key story words such as "validated", "current", "source of
truth", "admin", and "operator" into concrete invariants from the epic prose, ACs, and technical
notes. Verify every relevant invariant, not only the easiest visible one.

### 2. Output & Commit the Per-PR Report
For each PR, generate the review using the exact template below. 
**Crucially:** After generating this template, you must append it to the bottom of that story's `STORY_LOG.md` file under an `## Independent Review` heading, commit the change to the branch (`docs(review): append independent review findings`), and push it. This provides a durable trace of the review before the implementer makes repairs.

**Per-PR Output Template:**
```markdown
## Independent Review

### PR #<NUMBER> / Story <ID> Review

**1. Context & AC Verification**
- [ ] AC 1: `<PASS/FAIL/NOT VERIFIED>` - Evidence: `<...>`
- [ ] AC 2: `<PASS/FAIL/NOT VERIFIED>` - Evidence: `<...>`

**2. Independent Verification**
- `verify.sh --ci` result: `<Passed X, Failed Y>`
- Alembic single head confirmed: `<Yes/No>`

**3. Required Audits**
- [ ] D8 Tenant Query Audit: `<Clean / Finding: ...>`
- [ ] Domain Invariant Audit: `<Clean / Finding: ...>`
- [ ] Auth/RBAC Audit: `<Clean / Finding: ...>`
- [ ] Cache Trust Audit: `<Clean / Finding: ...>`
- [ ] Idempotency/Replay Audit: `<Clean / Finding: ...>`
- [ ] Adapter Audit: `<Clean / Finding: ...>`
- [ ] Database Constraint Audit: `<Clean / Finding: ...>`
- [ ] Migration Audit: `<Clean / Finding: ...>`

**4. PR Draft & Commit Log**
- PR message accuracy: `<Accurate / Inaccurate: ...>`
- Commit logs match diff: `<Yes / No: ...>`

**5. PR Verdict**
- `<APPROVE / REQUEST CHANGES / BLOCK MERGE>` - `<One sentence reason>`

**6. Repair Guidance** (one block per finding, grouped by PR/commit)
- PR/commit `<identifier>`:
  - Problem/omission: `<specific issue>`
  - Suggested solution: `<specific fix>`
  - Must be applied to lower branches and merged upward through the stack: `<Yes / No>`
  - Code sketch:
    ```text
    <small illustrative snippet, or "N/A" for docs/process-only fixes>
    ```
```

*(Repeat Part 1 for the next PR in the stack until you reach the top-of-stack branch)*

---

## Part 2: Final Stack-Wide Checks

Once the loop is complete for all PRs and the individual reports have been committed to their respective branches, perform a final sweep across the whole range.

Output the results of this final sweep using the exact template below to the human (you do not need to commit this stack-wide report, just output it to the user):

**Final Stack Report Template:**
```markdown
### Final Stack-Wide Checks

**1. Stack Topology & Consistency**
- Stack Topology: `<Correct / Broken>`
- Evidence Consistency: `<Clean / Stale placeholders found: ...>`

**2. Process Artifacts**
| Artifact | Present and complete? |
|---|---|
| Story logs exist for each story in range | Y / N |
| Stopped-story log explains stop reason and owner decision needed | Y / N / N/A |
| Micro-commit logs present for every story commit | Y / N |
| Verification script exists for each implemented story | Y / N |
| Verification scripts are self-contained and lock-aware | Y / N |
| Experiment run note exists and is current | Y / N / N/A |
| A separate PR exists for each story, properly stacked | Y / N |
| PR drafts/messages exist and match their specific diffs | Y / N |
| Process timestamps include time + timezone where known | Y / N |

**3. Stack Concerns**
- **Blocking (must fix before merging stack):** `<List or "None">`
- **Non-blocking (worth noting):** `<List or "None">`
- **Spec/process follow-ups:** `<List or "None">`

**4. Overall Process Verdict**
- `<APPROVE / REQUEST CHANGES / BLOCK MERGE>` - `<Reason>`

**5. Experiment Learning**
- `<Concise observation about model performance, guardrails, and the end-of-loop cadence>`
```

---

## Completion Note

*(Paste the completion note from the autonomous run below)*

You are a Senior Technical Product Manager working on ASPIRE.

Your task is to produce:
  brain/specifications/FEATURE_BACKLOG.md
  brain/specifications/STORY_BACKLOG.md
  brain/specifications/GITHUB_CONVENTIONS.md

---

STEP 1: READ THESE FILES

  brain/specifications/PRD.md
  brain/specifications/USE_CASES.md
  brain/specifications/epics/  (all derived epic files from Prompt 4 — the exact count is determined by the owner-approved epic list, not a fixed number)
  brain/build-and-dev/canonical_decisions.md

---

STEP 2: FEATURE_BACKLOG.md

A complete catalogue of every feature across all epics.

Header section: total feature count, epic count, date.

For each epic, one section:
  ## EPIC-XX: [Name]
  | # | Feature | Actor | FR | UC |
  |---|---------|-------|----|----|
  
  ### Explicitly Out of Scope for This Epic
  [Things that might be assumed but are not included]

Footer: cross-cutting features (localisation, offline mode, 
fraud detection, notifications) and which epics they touch.

---

STEP 3: STORY_BACKLOG.md

All user stories across all epics in prioritised order.

Priority is determined by dependency, not business value alone.
Nothing in a later group can be built before the earlier group.

Structure:

Use the owner-approved epic list from Prompt 4 to determine which
epics belong in each group. The group structure below reflects
the logical dependency order — replace EPIC-XX placeholders with
the actual epic IDs from the approved list.

## Priority Group 1: Platform Must Exist
  Stories from Group 0 epics (Foundation: infrastructure, auth, 
  geographic data layer)
  [These are not optional — nothing else runs without them]

## Priority Group 2: A Member Can Join
  Stories from Group 1 epics (Acquisition: registration channels,
  identity verification)
  [The front door must exist before the product exists]

## Priority Group 3: The Growth Loop Runs
  Stories from Group 2 epics (Growth: referral/progression,
  rewards engine, gamification)
  [Without this, there is no reason to join or stay]
  Note: The Rewards Engine epic is significantly complex — see
  Prompt 4 complexity note. It may span multiple sprints.

## Priority Group 4: The Network Organises
  Stories from Group 3 epics (Organisation: structure, leadership)

## Priority Group 5: The Campaign Communicates
  Stories from Group 4 epics (Communication: outbound, inbound)

## Priority Group 6: Events and Content
  Stories from Group 5 epics (Events & Content)

## Priority Group 7: The Candidate Sees and Acts
  Stories from Group 6 and 7 epics (Intelligence, Political Client)

## Priority Group 8: Platform Matures
  Stories from Group 8 epics (Platform: member portal, white label,
  admin, tenant provisioning & configuration, physical goods)

## Priority Group 9: Advanced Platform
  Blockchain audit trail, advanced AI, national scale features

For each story in each group:
  | Story ID | Epic | As a... | I want... | So that... | FRs | UCs |

At the bottom: dependency graph in Mermaid (story group → 
story group → code)

---

STEP 4: GITHUB_CONVENTIONS.md

The rules for how specifications connect to the GitHub repository.

Include:

### Milestone Naming
  One GitHub Milestone per epic.
  Format: "EPIC-XX: [Epic Name]"
  Description links to the epic file.

### Issue Template — User Story
  The exact markdown template to use when creating a story issue:
    - Story ID
    - Epic and Milestone
    - As a / I want / So that
    - Functional Requirements implemented (FR-XXX)
    - Use Cases covered (UC-XXX)
    - Acceptance Criteria (as checkboxes — each EARS criterion)
    - Definition of Done checklist

### Issue Template — Task
  For sub-tasks within a story (from tasks.md):
    - Task reference
    - Parent story (closes or part of #XX)
    - Done when: [criterion]

### Label Schema
  epic:XX (one per epic)
  req:FR-XXX (functional requirement tags)
  type:story | type:task | type:bug | type:spike
  priority:must | priority:should | priority:could
  status:draft | status:ready | status:in-progress | status:done
  domain:rewards | domain:auth | domain:comms | etc.

### Branch Naming Convention
  feature/US-[EPIC_CODE]-[NN]-short-description
  Examples:
    feature/US-REW-01-instant-airtime
    feature/US-REG-03-whatsapp-registration
    infra/EPIC-01-docker-compose-setup

### Commit Message Convention
  Conventional commits format:
  <type>(<domain>): <description> [US-XXX][FR-XXX]
  
  Types: feat, fix, test, refactor, docs, infra, chore
  Examples:
    feat(rewards): implement airtime disbursement [US-REW-01][FR-035]
    infra(auth): configure JWT with refresh tokens [US-AUTH-02]
    test(referral): add cascade attribution integration test [US-REF-04]

### Pull Request Template
  The exact markdown PR template:
    - Summary (1-2 sentences)
    - Story ID: [US-XXX-NN]
    - Implements FR-XXX, FR-XXX
    - Covers UC-XXX
    - Acceptance Criteria (checkboxes — each ticked = passing test)
    - Test files added or modified
    - Traceability matrix updated? (yes/no)
    - Any open questions resolved?

### Traceability Update Rule
  When a PR is merged, the person merging must:
  1. Mark the story's FRs as "implemented" in TRACEABILITY_MATRIX.md
  2. Update the epic file status if all stories are done
  3. Add an entry to _CHANGELOG.md

Stop when complete. No review gate needed — 
this is reference material, not requirements.

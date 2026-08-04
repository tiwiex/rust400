You are a Senior Technical Product Manager working on ASPIRE —
a Nigerian political mobilisation platform.

Your task is to:
  A. Derive the canonical epic list from the requirements 
     and use cases produced in earlier prompts
  B. Produce brain/specifications/TRACEABILITY_MATRIX.md
  C. Produce one epic file per derived epic in 
     brain/specifications/epics/

---

STEP 1: READ ALL OF THESE IN FULL

  brain/specifications/PRD.md
  brain/specifications/USE_CASES.md
  brain/specifications/DIAGRAMS.md
  brain/build-and-dev/canonical_decisions.md

---

STEP 2: DERIVE THE EPIC LIST

Do not use a pre-determined list of epics. 
Derive them from the material you have read.

Method:
  A. List every functional requirement from PRD.md (FR-XXX)
  B. List every use case domain from USE_CASES.md
  C. Group FRs and UCs into clusters where:
     — requirements in the cluster are closely related
     — they share primary actors
     — they could be built and delivered together
     — they have natural boundaries with other clusters
  D. Each cluster becomes a candidate epic

Naming each epic:
  — Name it for what it DOES, not what it IS
  — Name must be understood by a developer and a PM alike
  — Use plain English

Validate your derived list against this suggested starting 
point of 22 epics. For each suggested epic, confirm it is 
covered, or explain why you merged, split, or renamed it.
If you identify areas not covered by the suggested list,
add them and explain why.

Suggested starting epics (validate, do not adopt blindly):
  Group 0 — Foundation
    Platform Infrastructure & DevOps
    Authentication, Authorisation & Access Control
    Geographic & INEC Data Layer
  Group 1 — Acquisition
    Member Registration (WhatsApp, Web, App)
    Member Registration (USSD & SMS)
    Identity Verification
  Group 2 — Growth
    Referral System & Level Progression
    Rewards Engine & Commission Cascade
    Gamification, Points & Prize Draws
  Group 3 — Organisation
    Organisation Structure & Role Management
    Leadership Discovery & Development
  Group 4 — Communication
    Outbound Broadcast & Targeting
    Inbound Communication & Sentiment
  Group 5 — Events & Content
    Rally & Event Operations
    Content & Media Engine
  Group 6 — Intelligence
    Geographic Mapping & Density Intelligence
    AI Intelligence & Message Generation
  Group 7 — Political Client
    Candidate Dashboard & Campaign Management
    Election Day Operations
  Group 8 — Platform
    Member Portal & App Experience
    White Label & Multi-Tenancy
    Platform Administration & Operations

Known gaps to specifically check — do your analysis surface 
requirements for any of the following? If yes, ensure they 
have a home in an epic:
  — Training modules & content delivery
  — Physical prize fulfilment tracking
  — Member support & human escalation path
  — Financial reconciliation & failed payment handling
  — New political client onboarding & setup flow
  — Campaign archive & cross-cycle political memory
  — Data export & candidate portability on contract end
  — Blockchain ledger (large enough to separate from rewards?)

IMPORTANT: FR-147 to FR-173 (27 FRs across PRD §6.20 and §6.21)
must have epic homes in your derived list:
  §6.20 Platform & Tenant Configuration (FR-147–156): tenant
    onboarding, provisioning wallet load, percentage config,
    load-time pre-commitment atomic transaction, audit log.
    Closest suggested match: "Platform Administration & Operations"
    — but given the financial complexity, a dedicated
    "Tenant Provisioning & Financial Configuration" epic may be
    warranted. Assess and decide during derivation.
  §6.21 Physical Goods & Inventory (FR-164–173): Physical Goods
    Account, unit tracking, bulk procurement, fulfilment lifecycle,
    inventory carryover, prize catalogue, prize reservation.
    No direct match in suggested 22 — this is likely a new epic.

IMPORTANT: The Rewards Engine & Commission Cascade epic is
significantly more complex than a standard feature epic. It covers:
  — 4 ledger accounts with strict isolation rules
  — 3 disbursement triggers with different timing and atomicity
  — Configurable percentage cascade summing to 100%
  — Tenant General Pool fallback for missing upline levels
  — Outbox pattern for all financial events (D6.2)
  — Reconciliation sweep for expired verification timeouts
  — Raffle Pool geographic sub-accounts (Ward/LGA/National)
  — Target: Polygon EVM smart contract audit trail (FR-046)
  Mark this epic with a complexity note when you define it.
  Consider whether it should be split into sub-epics.

Also check: PRD.md §13 Known Gaps (GAP-001 to GAP-012) lists
12 open gaps. For each gap, your epic derivation should confirm:
  — Does the gap now have a FR home (e.g., FR-174/175 for GAP-004)?
  — Is the gap closed as out of scope?
  — Or is the gap still open and needs an epic placeholder?
  List unresolved gaps explicitly in your coverage gap table
  (Traceability Matrix TABLE 5).

Output of Step 2:
  A numbered epic list with:
    - Derived epic ID and name
    - One-line description
    - FR count it covers
    - UC count it covers
    - Mapping to suggested list (same / merged from X / 
      split from X / new — not in suggested list)

STOP HERE. Present the derived epic list and wait for 
owner review and approval before proceeding to Steps 3 and 4.
The epic list, once approved, becomes the authoritative 
structure for all subsequent work.

---

STEP 3: TRACEABILITY MATRIX
(only after epic list is approved)

Produce brain/specifications/TRACEABILITY_MATRIX.md

TABLE 1 — Business Goals to Functional Requirements
  G-XXX | Goal | FR-XXX list that implements it

TABLE 2 — Functional Requirements to Use Cases to Epics
  FR-XXX | Statement | Priority | UC-XXX list | Epic ID

TABLE 3 — Use Cases to Epics to Stories
  UC-XXX | Name | Primary Actor | Epic ID | Story count (TBD)

TABLE 4 — Epic Coverage Summary
  Epic ID | Name | FR count | UC count | Depends on

TABLE 5 — Coverage Gaps (do not hide these)
  Any FR with no UC. Any UC with no epic.
  Any epic with no FRs. Any epic with no UCs.
  State the gap, its likely cause, and what should resolve it.

---

STEP 4: EPIC FILES
(only after traceability matrix is reviewed)

For each approved epic, produce one file:
brain/specifications/epics/EPIC-[ID]_[name].md

Use this template for every epic:

---
id: EPIC-XX
title: [Name]
status: draft
version: 1.0
created: [today]
group: [Group name]
functional_requirements: [FR-XXX, ...]
use_cases: [UC-XXX, ...]
depends_on: [EPIC-XX, ...]
blocks: [EPIC-XX, ...]
primary_actors: [list including system actors where relevant]
---

# EPIC-XX: [Name]

## Purpose
What this epic enables. Why it exists. 
For infrastructure epics: what it makes possible for others.
2–3 sentences for a developer or PM.

## Why It Matters
Consequence of not building this.

## Primary Actors
Human and system actors. Infrastructure epics name 
DevOps Engineer, Platform Admin, System/Worker explicitly.

## Regulatory & Compliance Notes
Specific NDPR, INEC, CBN/NFIU constraints on this epic.
Reference NFR-XXX. Omit section if none apply.

## Feature List
Plain English bullets. One capability per bullet.
Start each with the actor: "A Member can...", 
"The System automatically...", "A Campaign Manager can..."
Be specific. 8–15 features.

## User Stories
4–8 stories.
US-[EPIC-CODE]-[NN]: [Title]
As a [role], I want [action], so that [outcome].
Each story annotated: implements FR-XXX, covers UC-XXX

## Acceptance Criteria
3–5 EARS criteria per story:
  WHEN [condition] THE SYSTEM SHALL [behaviour]
  IF [condition] WHEN [trigger] THE SYSTEM SHALL [behaviour]
  THE SYSTEM SHALL [always-active constraint]

## Dependencies
Which epics must be partially or fully complete first, and why.

## Key Technical Notes
Relevant canonical decisions from canonical_decisions.md.
The primary service or channel this epic involves.
Brief — technical detail goes in plan.md when building.

## Open Questions
OQ-[EPIC-CODE]-NN: question — impact — who decides

---

After all epic files:
  - Produce a Mermaid dependency graph of the derived epics
  - List any FRs still without an epic home
  - List cross-cutting open questions that need 
    canonical_decisions.md entries

Stop and wait for final review.

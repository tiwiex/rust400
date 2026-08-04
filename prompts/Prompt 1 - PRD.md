You are a Senior Technical Product Manager and Business Analyst 
working on ASPIRE — a Nigerian political mobilisation platform.

Your task in this session is to produce two documents:
  brain/specifications/START_HERE.md
  brain/specifications/PRD.md

---

STEP 1: READ THESE FILES IN FULL BEFORE WRITING ANYTHING

  brain/knowledge-base/ASPIRE_Vision_Strategic_Political_Infrastructure.md
  brain/build-and-dev/ASPIRE_Product_Specification.md
  brain/build-and-dev/canonical_decisions.md

These are context and input sources, not templates. 
Do not reformat them. Build from them.

---

STEP 2: PRODUCE START_HERE.md

A navigation document for anyone joining this project.
Include:
- What ASPIRE is (3 sentences)
- The document map: what each file in brain/specifications/ is for
- Reading order for four roles: New Developer, Product Manager, 
  Political Operator/Sponsor, Architect/Tech Lead
- Glossary of ASPIRE-specific terms: L1–L5, Mobiliser, Organiser, 
  Street Captain, Ward Coordinator, PU (Polling Unit), Commission 
  Cascade, USSD, White Label, Exclusivity Zone, Constitution
- A note on the traceability convention: FR-XXX → UC-XXX → EPIC-XX 
  → US-XXX → AC, and how these map to GitHub milestones and issues

---

STEP 3: PRODUCE PRD.md

This is a formal Product Requirements Document written from 
first principles. It is NOT a reformat of the product spec.
The product spec is a narrative explanation. This is a structured 
requirements contract.

Structure exactly as follows:

## 1. Background
  Political context. Why this product exists. What market failure 
  it addresses. 2-3 paragraphs, evidenced from the vision doc.

## 2. Problem Statement
  Specific, numbered problems this product solves. Not narrative —
  structured. Each problem should be independently stated.
  P-001: [problem title] — [description] — [consequence if unsolved]

## 3. Product Vision
  One paragraph. The future state ASPIRE creates.

## 4. Goals
  Numbered business goals. G-001, G-002...
  Each goal: statement + how it will be measured.

## 5. Stakeholders & Personas
  For each stakeholder/persona:
  - Name and role
  - Primary motivation
  - Pain point today
  - What ASPIRE gives them
  - How they interact with the system
  
  Include ALL of these:
  Member (L1), Mobiliser (L2), Organiser (L3), Street Captain / 
  Ward Coordinator (L4), Elite Operator (L5), Candidate, 
  Campaign Manager, ASPIRE Platform Admin, ASPIRE DevOps Engineer,
  ASPIRE Support Agent, Existing Political Loyalist (being integrated)

## 6. Functional Requirements
  Numbered: FR-001, FR-002...
  Each requirement:
    ID | Statement | Priority (Must/Should/Could) | Source | Epic
  
  Group by domain:
  - Registration & Onboarding
  - Identity Verification
  - Referral & Viral Growth
  - Level Progression
  - Rewards & Commission
  - Gamification & Points
  - Communication (Outbound)
  - Communication (Inbound & Sentiment)
  - Organisation Structure & Leadership
  - Rally & Event Operations
  - Content & Media
  - Geographic Intelligence
  - AI & Candidate Intelligence
  - Candidate Dashboard & Campaign Tools
  - Election Day Operations
  - White Label & Multi-Tenancy
  - Fraud Detection (cross-cutting)
  - Platform Infrastructure & Operations
  - Compliance & Data Protection

## 7. Non-Functional Requirements
  Numbered: NFR-001, NFR-002...
  Each: ID | Category | Statement | Measurement | Source
  
  Categories: Performance, Security, Reliability, Scalability,
  Compliance (NDPR, INEC, CBN/NFIU), Accessibility, 
  Localisation (Yoruba/Igbo/Hausa/Pidgin/English),
  Offline capability, Data residency

## 8. External Dependencies & Integrations
  Every external system the product depends on:
  - NIMC (NIN verification)
  - CBN partner (BVN verification)
  - Termii / Africa's Talking (SMS + USSD + airtime)
  - WhatsApp Business API provider
  - Flutterwave / Paystack (payments)
  - INEC (geographic/electoral data)
  - Polygon / EVM chain (blockchain ledger)
  - MapTiler (maps — MVP) / Protomaps (scale)
  For each: purpose, dependency level (critical/optional), 
  failure handling requirement

## 9. Success Metrics
  How we know the product is working. Reformatted from §20 of 
  the product spec into measurable, time-bounded KPIs.
  Group: Growth, Engagement, Political Delivery, Financial, 
  Community Impact.

## 10. Scope
  ### In Scope (full product — no MVP filtering)
  ### Explicitly Out of Scope
    Items the product spec or vision doc explicitly excludes,
    plus anything that might be assumed but is not included.
    Examples: live in-app group chat (deferred per D6.1), 
    iOS-first development (Android primary), 
    international deployment.

## 11. Assumptions & Constraints
  Things assumed true that if false would affect the design.
  Regulatory constraints. Technology constraints from D3/D6.

## 12. Open Questions
  Formal numbered list: OQ-001...
  Each: question, impact if unresolved, who decides.

---

QUALITY RULES FOR BOTH DOCUMENTS:
- Every functional requirement must be independently testable
- Use plain English — no jargon without a glossary entry
- Requirements state WHAT, not HOW
- Do not invent requirements not in the source documents
- Flag every ambiguity as an open question rather than deciding it
- Cross-reference between documents using the ID conventions 
  (FR-XXX, NFR-XXX, G-XXX, P-XXX)

After completing both documents, summarise:
- Total FR count, NFR count
- Top 5 open questions that block further progress
- Any significant gaps you found in the source documents
Then stop and wait for review before proceeding.

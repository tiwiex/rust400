You are a Senior Business Analyst working on ASPIRE — a Nigerian 
political mobilisation platform.

Your task is to produce brain/specifications/USE_CASES.md

---

STEP 1: READ THESE FILES IN FULL

  brain/specifications/PRD.md           (Prompt 1 output)
  brain/specifications/DIAGRAMS.md      (Prompt 2 output)
  brain/build-and-dev/canonical_decisions.md

---

STEP 2: DEFINE ALL ACTORS

Before any use cases, define every actor:
  For each: name, type (human / system / external), 
  description, goals, what system component they 
  primarily interact with.

Human actors (internal):
  Member (L1), Mobiliser (L2), Organiser (L3), 
  Street Captain / Ward Coordinator (L4), Elite Operator (L5),
  Candidate, Campaign Manager, 
  ASPIRE Platform Admin, ASPIRE Support Agent, ASPIRE DevOps Engineer,
  Existing Political Loyalist (being onboarded)

System actors (non-human):
  System/Scheduler — Celery Beat time-triggered jobs
  System/Worker — Celery async job processing
  System/Webhook — inbound callbacks from external APIs
  System/FraudDetector — continuous monitoring process
  System/Reconciliation — Celery Beat daily reconciliation sweep:
    checks acquisition attempts past verification timeout, releases
    held Stage 2 allocations to Tenant General Pool, marks attempts
    as expired in the acquisition attempt log (FR-045, FR-162)

External actors:
  NIMC (NIN verification service)
  CBN Partner (BVN verification service)
  Airtime API (Termii / Africa's Talking)
  Payment Gateway (Flutterwave / Paystack) — covers TWO distinct
    flows: (1) Candidate provisioning wallet top-up (candidate
    deposits campaign funds into their Provisioning Wallet via
    Flutterwave/Paystack — triggers load-time pre-commitments);
    (2) Member cash disbursement / bank transfer (Stage 2 cash
    credited to member wallet and available for bank withdrawal)
  WhatsApp Business API
  USSD Network (MTN/Airtel/Glo/9mobile)
  Blockchain Network (Polygon EVM)

Regulatory constraints (not actors, but define them here 
as external authorities that impose constraints on use cases):
  NDPR Authority, INEC (regulatory), CBN/NFIU

---

STEP 3: PRODUCE THE USE CASE CATALOGUE

For each use case, use this exact structure:

---
ID: UC-XXX
Name: [Short descriptive name]
Primary Actor: [who initiates]
Secondary Actors: [who else is involved, including system actors]
Related Requirements: FR-XXX, FR-XXX
Related Epic: EPIC-XX

Preconditions:
  [What must be true before this use case can begin]

Trigger:
  [What event initiates this use case]

Main Success Flow:
  1. [Actor does X]
  2. [System responds with Y]
  3. [Continue...]

Alternative Flows:
  A1. [Condition] — [different path] — [rejoins at step N]
  
Exception Flows:
  E1. [Failure condition] — [system response] — [outcome]

Postconditions:
  [What is true after successful completion]

Business Rules:
  [Any specific rules that apply — e.g., "airtime must credit 
   within 60 seconds", "duplicate phone numbers rejected"]
---

Group use cases by domain:
  - Registration & Onboarding (UC-REG-XXX)
  - Identity Verification (UC-VER-XXX)
  - Referral & Viral Growth (UC-REF-XXX)
  - Level Progression (UC-LVL-XXX)
  - Rewards & Commission (UC-REW-XXX)
  - Gamification & Prize Draws (UC-GAM-XXX)
  - Communication — Outbound (UC-COM-XXX)
  - Communication — Inbound & Sentiment (UC-INB-XXX)
  - Organisation & Leadership (UC-ORG-XXX)
  - Rally & Event Operations (UC-EVT-XXX)
  - Content & Media (UC-MED-XXX)
  - Geographic Intelligence (UC-GEO-XXX)
  - AI & Candidate Intelligence (UC-AI-XXX)
  - Candidate Dashboard (UC-DSH-XXX)
  - Election Day Operations (UC-ELC-XXX)
  - White Label & Multi-Tenancy (UC-WHL-XXX)
  - Platform Administration (UC-ADM-XXX)
  - Platform & Tenant Configuration (UC-CFG-XXX)
      covers PRD §6.20 FRs 147–156: tenant onboarding, provisioning
      wallet load, percentage configuration, platform fee config,
      load-time pre-commitment trigger, configuration audit log
  - Physical Goods & Inventory (UC-PHY-XXX)
      covers PRD §6.21 FRs 164–173: Physical Goods Account funding,
      inventory unit tracking, bulk procurement order, welcome pack
      fulfilment, prize catalogue management, prize reservation,
      winner fulfilment records, inventory carryover
  - System / Background Jobs (UC-SYS-XXX)
  - Fraud Detection (UC-FRD-XXX)

At the end, produce a summary table:
  UC ID | Name | Primary Actor | Related FRs | Related Epic

Flag any FR from the PRD that has no use case covering it.
These are gaps. List them explicitly.

Stop and wait for review before proceeding.

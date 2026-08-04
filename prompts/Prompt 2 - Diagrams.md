You are a Senior Technical Architect and Product Designer working 
on ASPIRE — a Nigerian political mobilisation platform.

Your task is to produce brain/specifications/DIAGRAMS.md

---

STEP 1: READ THESE FILES IN FULL BEFORE WRITING ANYTHING

  brain/knowledge-base/ASPIRE_Vision_Strategic_Political_Infrastructure.md
  brain/build-and-dev/ASPIRE_Product_Specification.md
  brain/build-and-dev/canonical_decisions.md
  brain/specifications/PRD.md        (produced in Prompt 1)

---

STEP 2: PRODUCE DIAGRAMS.md

One document containing all system and business diagrams.
Every diagram uses Mermaid syntax in a ```mermaid block.
Every diagram has: a numbered title, a one-paragraph plain-English 
caption explaining what it shows and why it matters.

Produce the diagrams in this order:

---

DIAGRAM 1 — C4 Level 1: System Context

Show ASPIRE as a single black box at the centre.
Surround it with:
  Human actors: Member, Mobiliser, Organiser, Candidate, 
  Campaign Manager, ASPIRE Platform Admin, ASPIRE Support Agent
  
  External systems (clearly labelled with their purpose):
  NIMC (NIN verification), CBN Partner (BVN), 
  Termii/Africa's Talking (SMS + USSD + Airtime),
  WhatsApp Business API, Flutterwave/Paystack (payments),
  INEC Data (ward/PU geographic registry),
  Polygon/EVM (blockchain ledger), MapTiler (base maps),
  MTN/Airtel/Glo/9mobile (carrier networks)
  
  Regulatory bodies shown as constraints (not connected by data 
  flows — shown with a dashed boundary):
  NDPR Authority, INEC (regulatory), CBN/NFIU

Use C4 notation. Caption explains what each external entity 
contributes or receives.

---

DIAGRAM 2 — C4 Level 2: Container Diagram

Show the internal containers of ASPIRE:
  FastAPI Backend (Python — core API, business logic)
  React + Vite Frontend (candidate dashboard + member web portal)
  WhatsApp Bot Service (registration, conversational flow)
  USSD Gateway Handler (USSD session management)
  Celery Workers (background jobs — rewards, broadcasts, draws)
  Celery Beat Scheduler (time-triggered jobs)
  PostgreSQL + PostGIS (primary database + geographic data)
  Redis (cache, leaderboards, rate limiting, pub/sub, Celery backend)
  RabbitMQ (financial message queue — durable)
  Mobile App — Android (member-facing)
  Blockchain Service (EVM L2 ledger interface — Polygon)
  
Show communication paths between containers with labels 
(REST, async queue, pub/sub, TCP, etc.)
Show which external systems each container talks to.

Caption explains the responsibilities of each container and 
why the separation exists.

---

DIAGRAM 3 — 14-Layer Product Architecture

Show the 14 canonical ASPIRE layers as a visual stack or map.
The 14 layers (from canonical_decisions.md D5) are:
  Verification, Recruitment, Rewards, Gamification, 
  Communication, Intelligence, AI, Media, Organization, 
  Rally Operations, Election Operations, Density Mapping, 
  Political Memory, Leadership Development

For each layer show: layer name + one-line description of 
what it does for the campaign.
Group into logical clusters if it aids readability 
(e.g., member-facing, candidate-facing, cross-cutting).

Caption explains how the layers relate to each other and 
that this is the product concept view (not the build order).

---

DIAGRAM 4 — Stakeholder & Actor Map

Show every human actor in the system and their relationships:
  - The member progression chain: Member → Mobiliser → Organiser → 
    Street Captain → Ward Coordinator → Elite Operator
  - Candidate and Campaign Manager as a separate cluster
  - ASPIRE internal roles: Platform Admin, Support Agent, DevOps
  - How actors relate to each other (recruits, reports to, 
    manages, communicates with, pays, monitors)

Caption explains the actor hierarchy and how ASPIRE sits 
between all of them.

---

DIAGRAM 5 — Member Journey: End to End

A flowchart from Discovery to Movement Leader.
Show: Discovery → Registration → Verification → 
Welcome Package → First 72 Hours Experience → 
First Recruitment → L2 Achievement → Network Growth → 
Event Participation → Content Creation → Leadership → 
L3/L4/L5 Progression

At key stages show: the emotional state (from product spec §4.6),
the trigger that moves them forward, and the reward received.

This must be readable by a non-technical political operator.

---

DIAGRAM 6 — Level Progression System

A structured diagram showing L1 through L5:
For each level: internal name, narrative name(s) from crosswalk 
(canonical_decisions.md D2), network requirement, key reward, 
new access unlocked, verification required.

Show the gates between levels — what a member must do/prove 
to progress. Mark numeric thresholds as PROVISIONAL.

---

DIAGRAM 7 — Financial Architecture: Three Triggers, Four Accounts

IMPORTANT: Do NOT use fixed naira amounts in this diagram. All
allocations are percentages of the per-member provisioning cost,
configurable by the tenant. Members are never charged to register.
All funds originate from the Tenant Provisioning Wallet loaded by
the candidate before any acquisition begins.

Show the complete financial flow across three disbursement triggers:

TRIGGER 0 — Wallet Load Time (atomic DB transaction):
  Candidate loads Provisioning Wallet
  → Immediately routes Physical Goods % to Physical Goods Account
    (pre-funds batch procurement before any member registers)
  → Immediately routes Platform Fee % to ASPIRE Operations Account
    (ASPIRE platform fee, set per-tenant by ASPIRE)
  → Remainder is the Provisioning Pool available for member events

TRIGGER 1 — Stage 1: OTP Confirmed (within 60 seconds):
  Member confirms phone via OTP
  → Airtime % sent to new member immediately via carrier API
  → Airtime cost is irrecoverable if Stage 2 never completes
  → Fulfilment order raised against Physical Goods Account
    for welcome pack items (T-shirt, wristband, referral card)

TRIGGER 2 — Stage 2: NIN/BVN Verified (on verification event):
  Member completes identity verification
  → Cash % credited to Member Wallet (available for bank withdrawal)
  → Commission Cascade %: distributed across upline levels
    (L1 upline %, L2 upline %, L3 upline %, L4 upline %)
    IF an upline level does not exist for this member,
    THAT level's % routes to the Tenant General Pool (not dropped)
  → Raffle Pool % distributed across geographic sub-pools:
    Ward Raffle Pool % + LGA Raffle Pool % + National Pool %
  All percentages across all six cascade destinations must 
  sum to 100% of the per-member acquisition cost.
  
  Note: Timing is "on NIN/BVN verification" — there is no
  fixed 24-hour window. Cash fires when identity is confirmed,
  which could be minutes, hours, or days after Stage 1 (up to
  the configured verification timeout, default 14 days).

Show the four ledger accounts:
  1. Physical Goods Account (funded at load; tracks ₦ + units)
  2. ASPIRE Operations Account (funded at load; ASPIRE's ledger)
  3. Provisioning Pool (funded at load minus pre-commitments)
  4. Member Wallet (funded at Stage 2; member can withdraw)
  Plus: Tenant General Pool (cascade fallback + pool)
  Plus: Raffle Sub-Pools (Ward, LGA, National)

Show the execution mechanism:
  MVP: PostgreSQL ledger + Celery outbox pattern 
       (event written atomically, published reliably)
  Target: Polygon EVM smart contract audit trail (FR-046)
  Mark blockchain as target state, Postgres as MVP.

Mark all percentage values as PROVISIONAL (configurable by tenant,
must sum to 100%, within ASPIRE-set bounds per FR-148/FR-149).

---

DIAGRAM 8 — Organisation Hierarchy

Show the geographic and organisational structure:
  Individual Member
    → Cell (10–50 members, led by Mobiliser L2)
      → Ward Group (all cells in a ward, led by Organiser L3)
        → LGA Group (all wards in an LGA, led by Executive L4)
          → State Group (all LGAs, led by Elite L5)
            → National Network

Show alongside the political geography:
  Street / Polling Unit → Ward → LGA → State → National

Caption explains how ASPIRE's org structure mirrors Nigeria's 
electoral geography by design.

---

DIAGRAM 9 — Multi-Channel Access Model

A table or diagram showing:
  Channels: USSD | SMS | WhatsApp | Android App | Web
  For each channel:
    - Who uses it (device/connectivity level)
    - What they can do (registration, balance check, referral, 
      event check-in, messaging, full dashboard)
    - Fallback behaviour if channel is unavailable

Caption emphasises that USSD is the inclusion channel — 
the system must work without smartphones or data.

---

DIAGRAM 10 — High-Level Data Flow

Show how data moves through the system for the core 
member lifecycle (not a database schema):
  Member action → Channel (WhatsApp/USSD/App) → 
  API → Business logic → Database → 
  Event queue → Celery worker → 
  External APIs (airtime/payment/blockchain) → 
  Notification back to member

Show the financial path specifically (no fixed amounts — 
all are configurable percentages per Diagram 7):
  Candidate wallet load → load-time pre-commitments (Physical 
  Goods Account, ASPIRE Operations Account) → Provisioning Pool →
  Stage 1 trigger (airtime to member + fulfilment order) →
  Stage 2 trigger (cash to member + cascade to uplines via 
  Tenant General Pool fallback + raffle pool sub-accounts) →
  bank transfer / ATM disbursement → blockchain audit record.

---

DIAGRAM 11 — Election Day Operations Flow

Show the flow for election day visibility:
  Polling unit observer check-in → 
  Incident report submission → 
  Escalation to coordinator → 
  Dashboard visibility for campaign manager → 
  Result sheet awareness → 
  Social media exposure

Show the actors involved at each step and the channels used.

---

---

DIAGRAM 12 — Provisioning Wallet Lifecycle

Show the complete lifecycle of a Tenant Provisioning Wallet
from load to depletion and reconciliation:

Stage A — Load:
  Candidate initiates wallet top-up (via payment gateway)
  → Payment confirmed → Wallet balance increases
  → Atomic DB transaction fires immediately:
      Physical Goods % → Physical Goods Account
      Platform Fee % → ASPIRE Operations Account
      Remainder → Provisioning Pool
  → Candidate sees: Total loaded / Pre-committed / Pool available

Stage B — Per-Member Disbursements (Provisioning Pool depletes):
  For each member registration:
    Stage 1: Airtime % drawn from Provisioning Pool
    Stage 2: Cash % + Cascade % + Raffle % drawn from Pool
    Acquisition attempt expires (timeout): Stage 2 allocation
      returns to Tenant General Pool (not back to Pool)

Stage C — Physical Goods (Physical Goods Account):
  Campaign manager issues bulk purchase order against balance
  → Inventory units created (e.g., 100 T-shirts at ₦X each)
  → Per Stage 1: one unit reserved for new member
  → Fulfilment team marks delivery → unit consumed
  → Unused units and balance carry forward to next campaign

Stage D — Reconciliation (Celery Beat / System):
  Daily sweep: check all members with Stage 1 complete 
  and Stage 2 pending past timeout threshold
  → Expired: release held Stage 2 allocation → Tenant General Pool
  → Flagged: acquisition attempt recorded as incomplete
  Wallet top-ups and disbursements reconciled against ledger

Stage E — End of Campaign:
  Remaining Provisioning Pool balance: candidate decision 
  (see OQ-016 — options pending owner/legal confirmation)
  Physical Goods Account balance and units: carry forward
  Tenant General Pool: candidate may redeploy for bonus events

Show all four accounts plus Tenant General Pool and Raffle Pool
sub-accounts on a single view. Mark OQ-016 on the end-of-campaign
balance as an open question. Mark all percentages as PROVISIONAL.

---

AFTER ALL DIAGRAMS:

List any aspects of the system that you were unable to diagram 
clearly due to ambiguity in the source documents. 
Flag these as open questions in the format OQ-DIA-XX.

Account reference note: Diagrams 7, 10, and 12 all reference the
same four ledger accounts (Physical Goods Account, ASPIRE Operations
Account, Provisioning Pool, Member Wallet) plus Tenant General Pool
and Raffle sub-pools. Ensure consistent naming across all captions.

Summarise: number of diagrams produced, any source conflicts 
encountered, any provisional data used (mark PROVISIONAL).
Note that all naira amounts shown in any diagram must be labelled
PROVISIONAL or shown as percentage variables, not fixed values.
Then stop and wait for review.

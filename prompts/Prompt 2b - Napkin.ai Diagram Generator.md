You are an Information Architect and Diagram Designer.

Your task is to generate a Napkin AI visual-generation brief for ONE diagram from the ASPIRE project.

Read:

* DIAGRAMS.md
* PRD.md
* canonical_decisions.md
* ASPIRE Product Specification
* ASPIRE Vision documents

The goal is NOT to explain ASPIRE.

The goal is to produce the minimum information Napkin needs to generate an accurate diagram.

Output only the following sections:

# Diagram Type

Choose exactly one:

* Flowchart
* Hierarchy
* Lifecycle
* Architecture
* Network Graph
* Layered Stack
* Swimlane
* Value Chain

# Purpose

2-4 sentences explaining what the diagram represents.

# Required Entities

Bullet list.

Only include entities that must appear.

# Required Relationships

Bullet list.

Explicitly state how entities connect.

# Visual Structure

Describe the preferred layout and grouping.

# Terminology That Must Be Preserved

List exact labels.

Napkin must not rename them.

# Things To Avoid

List common mistakes and simplifications to avoid.

# Napkin Prompt

Produce a final concise prompt between 250 and 600 words.

The prompt should:

* be self-contained
* preserve ASPIRE terminology
* focus on diagram generation
* avoid implementation details not relevant to the diagram
* avoid PRD references
* avoid explaining unrelated parts of ASPIRE

The output should be optimized specifically for Napkin AI.


One shot example
```
# Diagram Type

Flowchart

# Purpose

Show how money moves through the ASPIRE platform from candidate funding through member rewards, commission cascades, raffle pools, and fallback handling.

# Required Entities

* Candidate
* Provisioning Wallet
* Physical Goods Account
* ASPIRE Operations Account
* Provisioning Pool
* Trigger 1: OTP Confirmed
* Trigger 2: NIN/BVN Verified
* Airtime Disbursement
* Fulfilment Order
* Member Wallet
* Commission Cascade
* Tenant General Pool
* Raffle Pools

# Required Relationships

* Candidate funds Provisioning Wallet
* Wallet routes funds to accounts
* Trigger 1 creates airtime and fulfilment actions
* Trigger 2 creates cash, commission and raffle actions
* Missing uplines route to Tenant General Pool

# Visual Structure

Three vertical stages:

Wallet Load → OTP Confirmed → NIN/BVN Verified

Financial destinations beneath each stage.

Decision diamond for upline existence.

# Terminology That Must Be Preserved

* Provisioning Wallet
* Physical Goods Account
* ASPIRE Operations Account
* Provisioning Pool
* Member Wallet
* Tenant General Pool
* Commission Cascade
* Trigger 1
* Trigger 2

# Things To Avoid

* Do not show members paying money
* Do not collapse accounts together
* Do not replace account names with generic finance terms
* Do not remove fallback routing

# Napkin Prompt

Create a financial flow diagram for ASPIRE, a political mobilisation platform.

The diagram begins with a Candidate funding a Provisioning Wallet.

The wallet immediately routes funds into three destinations:

1. Physical Goods Account
2. ASPIRE Operations Account
3. Provisioning Pool

Show these as separate accounts.

The next stage is Trigger 1: OTP Confirmed.

This produces:

* Airtime Disbursement to the member
* Fulfilment Order for physical welcome items

The next stage is Trigger 2: NIN/BVN Verified.

This produces:

* Cash credit to Member Wallet
* Commission Cascade
* Raffle Pool Distribution

Commission Cascade must include a decision point:

"Upline Exists?"

If yes:
Commission routes to uplines.

If no:
Commission routes to Tenant General Pool.

Raffle Pool Distribution splits into:

* Ward Pool
* LGA Pool
* National Pool

Use a clean left-to-right flow.

Visually emphasize the progression from funding to rewards.

Use distinct visual groups for:

* Accounts
* Triggers
* Member rewards
* Fallback handling

Preserve all account names exactly as written.
```
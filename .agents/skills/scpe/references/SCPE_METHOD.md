<!-- Copy of SCPE_METHOD.md from https://github.com/m4rc0s/scpe at commit b43ad96 + technical_deal.md rename. Update this file when the method changes. -->

# Spec-Compiled Product Engineering (SCPE)

> **A product engineering method in which living Markdown documentation is the contract and AI agents compile it into software — grounded in incremental feature discovery, business intent modeled by everyone on the team, and traceability between specification and code.**

---

## 📋 Executive Summary & Manifesto

AI agents now write code with a quality that often matches experienced developers. The bottleneck has moved: it is no longer writing syntax, but **specifying business intent precisely enough for an agent to build it**.

In SCPE, living documentation — written in **Markdown and natural language** — is the contract and the single source of truth. Code is its consequence.

The method stands on five pillars:

* **Everyone is a Developer:** Product managers, designers, engineers, and domain experts all model intent. Nobody just "hands off requirements".
* **Incremental Feature Discovery:** Build the macro vision of the most critical feature for the business *now*, deliver it, learn, and iterate. Never map the entire system upfront.
* **Product Workspace:** One isolated workspace per product, with a manifest (`app.md`) for each application in `apps/`.
* **Wave Pipeline:** Agents execute the specification in a standard, asynchronous flow — Upstream → Readiness Gate → Downstream → Audit — with a formal state per epic.
* **DDD as a Language:** Ubiquitous language, bounded contexts, invariants, and concrete examples shape the model before any code exists.

### Why Markdown and Natural Language?

* **Anyone can read and write it:** Technical and business people collaborate on the same files; agents read them without conversion.
* **Git handles it:** Clean diffs show how intent and architecture evolve.
* **No tool lock-in:** The specification lives in the product repository, not in a proprietary platform.

---

## 0. Where SCPE Fits

SCPE belongs to the **Spec-Driven Development** movement and borrows from its neighbors:

| Reference | What SCPE Takes | Where SCPE Differs |
|---|---|---|
| [GitHub Spec-Kit](https://github.com/github/spec-kit), OpenSpec | Spec → Plan → Tasks → Implementation | The unit is the whole **product**, not an isolated code repository or a single change |
| [Product Definition as Code](https://github.com/product-definition-as-code/spec) | The product definition lives as Markdown in Git; references by name; tools check structure, humans decide truth | Lighter: plain Markdown files in a fixed workspace, no schemas required |
| [specdriven.com](https://specdriven.com/) | Intent → Specification → Implementation → Evidence; business rules plus *Given/When/Then* examples | Adds a workspace layout and an epic state machine an agent can follow |
| [AI-Native Agile Manifesto](https://arxiv.org/html/2605.07717v2) | Shared living context over meetings; humans in control, agents executing; verification first | Applied to one product team, not a whole organization |

### Core Practices

* **One Workspace per Product:** The agent's scope is restricted to the active product folder, which protects it from unrelated context.
* **Clone the Product, Not the Service:** You clone the entire product. Documentation guides the agents; code is the subordinate asset that is validated, run, tested, and deployed.
* **Product → Apps:** The product owns everything: `Product → Apps → {Backend, Client, …}`.
* **Agreed Engineering Standards:** Generated code follows the standards the team agrees in `technical_deal.md` — for example SOLID, Clean Architecture, Hexagonal (Ports and Adapters), and established design patterns.
* **Application Manifest (`app.md`):** A short, generic Markdown descriptor that tells the agent the responsibility, stack, and boundaries of each application in `apps/`.

---

## 1. Everyone is a Developer — DDD Without the Ritual

Eric Evans' central idea in Domain-Driven Design was not about mapping tables, but that **the heart of software is solving problems in the user's domain**. With implementation delegated to agents, a team can focus on exactly that.

That is why, in SCPE, **"Developer" has a universal meaning**. Everyone who shapes the product — Product Managers, Designers, Engineering Managers, Staff Engineers, domain experts — is a Developer. Each contributes their own expertise, and together they guide agents in writing epic scopes (`index.md`) and plans (`plan.md`), anchored to the team's technical agreements.

**Humans decide, agents execute.** Developers define intent, review, and approve. Agents draft documents, break plans into tasks, and write code. No agent approves its own work or settles an open business question on its own.

When modeling, Developers focus on five things:

* **Ubiquitous Language:** A shared glossary (`glossary.md`). If the business says a "Contract" differs from a "Proposal", the team and the agent use exactly those terms.
* **Bounded Contexts:** Clear boundaries for where each business rule starts and ends, so the agent stays focused on one part of the domain.
* **Entities and Business Rules (Invariants):** Entities have identity and a lifecycle governed by invariants — rules that must never be broken.
* **Domain Events:** Reactions and important state changes (*"When an Order is paid, notify Shipping"*).
* **Examples:** Every rule and behavior is illustrated with a concrete *Given / When / Then* scenario. If you cannot write the example, the decision has not been made yet.

---

## 2. Architecture: The Product Workspace

Keep your portfolio organized in one place (e.g., `$HOME/product_design/`), with one rule: **one dedicated workspace per product**.

### Product Root:

```text
myproduct/
├── index.md                 # Master guide: structure, summary, and navigation
├── product_vision.md        # Macro vision, business goals, profitability, and core problem
├── roadmap.md               # Strategic direction and product milestones
├── glossary.md              # Ubiquitous language (domain terms)
├── architecture.md          # C4 model, systemic choices, and integrations
├── technical_deal.md         # Technical agreements, AI guardrails, constraints, and approved stack
├── team_playbook.md         # Rules of engagement, workflow, and team culture
├── quick_status.md          # Product-wide status panel
├── assets/                  # Reference documents, wireframes, and visual assets
├── apps/                    # The software
└── features/                # Development lifecycle, sliced by domain and functionality
```

---

## 3. Applications (`apps/`) and the Manifest (`app.md`)

The `apps/` folder holds the software generated from the specification. Each application has a Markdown manifest: **`app.md`**.

```text
apps/
├── api-core/
│   ├── app.md               # Application manifest
│   └── src/ ...             # Source code generated by the agent
└── desktop-client/
    ├── app.md               # Application manifest
    └── src/ ...             # Source code generated by the agent
```

### Standard `app.md`:

A generic manifest that works for any kind of application — backend, web, mobile, worker, library, data pipeline.

```markdown
# App: api-core

- **name:** api-core
- **type:** backend-api
- **description:** Core service for transaction processing and the financial ledger.
- **stack:** Kotlin, Spring Boot, PostgreSQL
- **standards:** as agreed in technical_deal.md (e.g., Clean Architecture, SOLID)
- **entrypoint:** src/main/kotlin/com/myproduct/Main.kt
- **depends_on:** web-checkout (HTTP), message broker (domain events)
- **run:** ./gradlew bootRun
- **test:** ./gradlew test

## Boundaries
What this app must not do or own (e.g., does not render UI, does not store card data).
```

---

## 4. Protocol: The Wave Pipeline

### 4.1. Incremental Feature Discovery

SCPE rejects mapping an entire system upfront. The team builds the **macro vision of the most critical feature for the business right now**:

* **Learn as you build:** Validate hypotheses quickly, iterate on real deliveries, fail early, adjust course.
* **AI as co-pilot:** Developers use agents to draft and structure the intent of the priority feature into the standard files (`index.md`, `plan.md`).

### 4.2. Features, Epics, and `index.md`

Work is organized in the directory tree. **`index.md`** is always the entry point of a folder, for people and agents alike.

```text
features/
└── [feature_name]/
    ├── index.md             # Feature overview, business scope, and value
    ├── feat_roadmap.md      # Feature milestones
    ├── quick_status.md      # Feature status summary
    └── epics/               # The feature broken into deliverable packages
        └── [epic_name]/
            ├── index.md         # Epic scope and bounded contexts
            ├── plan.md          # Conceptual model: rules, examples, and slices
            ├── tasks.md         # Atomic task queue for the agent
            ├── quick_status.md  # Epic state and log
            └── epic_roadmap.md  # Tactical execution planning
```

The content of each file is standardized in [Section 5](#5-standard-file-formats).

#### 4.2.1. Vertical Slicing (Recommended)

How an epic is sliced decides when users see value — and when the team starts learning.

* **Horizontal slicing** builds one layer at a time (all data, then all API, then all UI). Nothing is usable until every layer is done.
* **Vertical slicing** (the default) makes each epic — and, when possible, each task — cut across every layer needed to deliver a small but real increment to the user. Each slice is a chance to validate, learn, and pivot.

This is a recommendation, not a rule. When an epic cannot be sliced vertically — an infrastructure migration, for instance — the Tech Lead may accept another approach at the Readiness Gate. The choice and its reason are written in `plan.md`.

### 4.3. The Wave Pipeline (Non-Waterfall)

Work flows through four waves. Different features move through them at the same time — never as a single cascade for the whole product.

| Wave | Who | What Happens | Files |
|---|---|---|---|
| **1. Upstream** | Developers (any role) with an agent as co-pilot | Align on the current priority, scope the feature and epic, and model the plan: terms, rules, examples, slices | `product_vision.md`, `glossary.md`, `index.md`, `plan.md` |
| **2. Readiness Gate** | Tech Lead | Check the model is consistent, complete, and fits the architecture; mark the epic `Ready` | `plan.md`, `quick_status.md` |
| **3. Downstream** | Agent, reviewed by Developers | Turn the plan into atomic tasks and write code and tests, slice by slice | `tasks.md`, `apps/` |
| **4. Audit** | Everyone | Track progress, learnings, and blockers as they happen | `quick_status.md` |

Before any of this, the product root files (`product_vision.md`, `architecture.md`, `technical_deal.md`) set the base context and the engineering standards the agent must follow.

### 4.4. Epic State Machine

The epic's `quick_status.md` declares exactly one of these states — no other value is valid:

```text
Draft → Ready → WIP → Done
  ↑        ↓      ↓
  └──── Blocked  Stale
           │        │
           └───→ (returns to Ready after resolution)
```

| State | Meaning | Set by |
|---|---|---|
| `Draft` | Being modeled in Upstream; not yet submitted to the Readiness Gate | Developers |
| `Ready` | Approved at the Readiness Gate; can be built | Tech Lead |
| `WIP` | Being built by an agent | Agent |
| `Blocked` | Stopped by an external dependency or a discovered defect; returns to `Ready` once resolved | Agent or Developer |
| `Done` | Code in `apps/` matches the current `plan.md` | Agent, confirmed by review |
| `Stale` | `plan.md` was changed after `Done`; only a new Readiness Gate returns it to `Ready` | Automatic, on any edit to `plan.md` |

Each transition is written by whoever performs the action that causes it. This keeps the protocol unambiguous for an agent.

### 4.5. Spec Drift

**Rule:** No change to the `plan.md` of a `Done` epic is silent.

When `plan.md` changes on a `Done` epic, the epic becomes `Stale` and goes back to the Readiness Gate — not to Upstream, because the model was already validated once. Only the difference needs review.

The Tech Lead reviews the difference and records one of two decisions in `epic_roadmap.md`:

- **Re-execute** — the change affects implemented behavior. The epic returns to `Ready`, and a new Downstream cycle updates the affected code.
- **Accept the drift** — the change is cosmetic or does not affect behavior. The epic goes back to `Done`, with a note explaining why the difference between `plan.md` and `apps/` is acceptable.

This keeps "documentation as single source of truth" an actively maintained property, not an aspiration.

### 4.6. Multi-Repository Applications

When an application's code lives in a separate repository, its manifest stays in `apps/<app>/` — the specification never leaves the product workspace — and the code is replaced by a pointer:

```text
apps/
└── api-core/
    ├── app.md               # Stays inside the product workspace
    └── repo_pointer.md      # Repository URL, integration branch, reference commit
```

`repo_pointer.md` holds only location data, never code. The agent writes code to the external repository, but status (`quick_status.md`) and specification (`plan.md`, `app.md`) always stay in the product workspace. What is distributed is the *code*, never the *specification*.

---

## 5. Standard File Formats

Every file below is plain Markdown that any Developer can write — no special syntax, no tool required. Short fields use the same `- **field:** value` style as `app.md`, so both people and agents can scan them.

**Naming inside an epic:** rules are numbered `R1, R2…` and examples `S1, S2…`, local to the epic. Tasks and tests cite them as `<epic_name>#S2`. This is the traceability link from specification to code.

### Feature `index.md`

```markdown
# Feature: Checkout

- **goal:** Customers complete a purchase quickly and safely.
- **users:** Customer, Store Operator
- **bounded_context:** Payments
- **business_value:** Reduce checkout abandonment from 38% to 25%.

## Scope
What this feature covers.

## Out of Scope
What it deliberately does not cover.

## Epics
- [pix-payment](epics/pix-payment/index.md) — Ready
```

### Epic `index.md`

```markdown
# Epic: Pix Payment

- **feature:** checkout
- **bounded_context:** Payments
- **apps:** api-core, web-checkout

## Scope
Customers pay an order with Pix and see the confirmation.

## Out of Scope
Pix refunds.
```

### Epic `plan.md`

```markdown
# Plan: Pix Payment

## Intent
Customers pay instantly with Pix instead of typing card details.

## Domain Model
- **Terms:** Order, Pix Charge (see glossary.md)
- **Entities:** Order (total, status), Pix Charge (amount, expires_at)
- **Domain events:** OrderPaid → notify Shipping

## Business Rules
- **R1:** A Pix charge expires 30 minutes after it is created.
- **R2:** An order is paid only when the confirmed amount equals the order total.

## Examples
- **S1 — Paid order:** Given an order of R$ 120 awaiting payment, When the customer pays R$ 120 with Pix, Then the order is marked paid.
- **S2 — Expired charge:** Given a Pix charge created 31 minutes ago and unpaid, When the customer opens the order, Then the charge shows as expired and a new one can be created.

## Slices
1. Pay an order with Pix — S1, R2
2. Expired charges — S2, R1

## Open Questions
- Should differences under R$ 0,01 be accepted? (Finance)
```

Rules for a good plan:

- One `When` per example; `Then` states something a user or another system can observe.
- An epic cannot pass the Readiness Gate with blocking open questions.
- If slices are not vertical, say why under **Slices**.

### Epic `tasks.md`

```markdown
# Tasks: Pix Payment

- [x] T1 — Create a Pix charge for an order (S1)
- [ ] T2 — Confirm payment from the bank webhook and mark the order paid (S1, R2)
- [ ] T3 — Expire unpaid charges after 30 minutes (S2, R1)
```

The agent writes this file from `plan.md` at the start of Downstream. Every task cites the examples or rules it implements, and every example is covered by at least one test named after it (e.g., `pix-payment#S2 expired charge can be recreated`).

### Epic `quick_status.md`

```markdown
# Status: Pix Payment

- **state:** WIP

## Log
- 2026-09-20 — T1 done. Bank sandbox rejects amounts with 3 decimals; no plan impact.

## Blockers
None.
```

### `glossary.md` Entry

```markdown
## Contract
- **bounded_context:** Sales
- **definition:** A signed agreement that obliges both parties.
- **not_to_be_confused_with:** Proposal — an unsigned offer that can still change.
```

---

## Trade-offs

- **More states to audit.** Six formal states instead of four informal ones mean more protocol for the Tech Lead to watch — but an agent can follow them without ambiguity.
- **`Stale` forces reviews.** Every edit to `plan.md` after delivery goes back through the Readiness Gate, even when it is trivial. This is deliberate: an unnecessary review costs far less than undetected drift between specification and code.
- **`repo_pointer.md` can go out of date.** If a repository moves and the pointer is not updated, it points to the wrong place. This is accepted, because putting code from several repositories inside the product workspace would break its isolation. Automatic pointer checks are out of scope for now.

# SYNC mode design (epoch-replicated dial) — MCC D

Goal: sub-floor adaptive exploration at high K (best teams hit 0.68·E/W at K=50; scans cap at ~0.95).

## Core idea
Every agent replicates the SAME exploration state machine (dial + DSU + blobs +
deduction) deterministically. Work proceeds in epochs:

- State S_n is identical on all agents (results of epochs ≤ n applied).
- Epoch plan(n): pop L·A edges from the replicated dial (A = querying agents,
  L = epoch length in turns). Agent with rank r takes edges r, A+r, 2A+r, ...
  (deterministic; no dispatch messages at all).
- Each agent queries its L edges over L turns; results = L bits.
- Upward sync: binary tree (parent j>>1, children 2j/2j+1, root 0): leaves send
  L-bit reports; internal nodes merge children + own into subtree bitmaps
  (deterministic order), forward up. Root merges → epoch result bitmap (A·L bits).
- Downward: root sends merged bitmap to children; each internal node forwards to
  its children (2 sends). Everyone applies → S_{n+1}.
- Claim: any agent whose replicated state connects start-goal claims (all
  detect the same turn; merlin takes the first).

## Budgets (K=50, N=501)
- Bitmap density: results in epoch-plan order → 1 bit/edge. A·L ≈ 50·24 = 1200
  bits ≈ 200 chars ≈ 1 msg. Internal-node overhead per epoch: 2 up-recv +
  1 up-send + 2 down-send + 1 down-recv ≈ 6 turns of L=24 (75% query efficiency;
  leaves ~96%).
- Staleness ≈ 1-2 epochs ≈ 24-48 edges/agent ≈ sim B=24-48 ⇒ Q ≈ 0.35-0.40·E.
- T ≈ Q/(K·0.8) + epoch sync tails. Single-buffered: sync dead time ≈ 2·depth
  per epoch; double-buffered (plan n+1 from S_{n-1}): no dead time, staleness 2
  epochs.
- Estimate N=501 K=50: ~1100-1600 vs current 2442. N=441 K=50: ~900-1300 vs 1899.

## Deterministic replication requirements
- Same binary, same startup (N,K,MSGLEN) → same init.
- Dial/DSU/deduction code MUST be shared verbatim (reuse coordinator's
  processResult/dial but strip messaging).
- Plan pops must validate identically (est/same-comp checks against S_n only).
- Edge case: dial dry during pop (all in-flight) → plan shorter; pad with '.'.
- CAUTION: any float/rand/iteration-order divergence breaks everything. DSU
  path compression is deterministic given same op order ✓.

## Failure tolerance
- Internal node timeout on child (like relay): forward partial with present-mask;
  missing agents' bits = unknown → those edges stay est=0, re-enter dial next epoch.
- Dead agent = its plan slots wasted per epoch; root can reassign rank map by
  observed masks (v2).

## Milestones
- M1: mode 6, K≥22 (vs regional/static A/B), single-buffered epochs, binary tree.
- M2: double-buffer pipeline (hide sync latency).
- M3: extend down to K≥14 if wins; tune L (16/24/32); non-binary tree arity.

## Message formats
- Up: 'U' + epoch(1ch) + presentMask(9ch) + bits (subtree agents ascending rank,
  each L bits, only present ones).
- Down: 'W' + epoch(1ch) + presentMask(9ch) + full bitmap (A·L bits, absent
  agents' segments omitted).
- epoch counter mod 94 (1 char).

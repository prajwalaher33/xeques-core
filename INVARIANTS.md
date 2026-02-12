# XEQUES Protocol Invariants

This document maps formal protocol invariants
to concrete enforcement points in the Rust implementation.

---

## Invariant I1 — Authorization Before Execution

**Statement:**  
A device MUST NOT execute a command unless it verifies
a valid authority signature.

**Enforced in:**  
- src/security/verify.rs  
- src/protocol/command.rs

---

## Invariant I2 — Execution Before Receipt

**Statement:**  
A receipt MUST NOT exist unless execution has occurred.

**Formal (RFC-0001):**  
Receipt generation iff δ(s, c) evaluated.

**Enforced in:**  
- src/protocol/receipt.rs  
- src/protocol/arbitration.rs

---

## Invariant I3 — Receipt Unforgeability

**Statement:**  
Producing a valid receipt without execution implies
private key compromise.

**Enforced by:**  
- Post-quantum signatures
- src/security/vote_crypto.rs

---

## Invariant I4 — Monotonic Command Sequencing

**Statement:**  
Commands MUST be processed in strictly increasing sequence order.

**Enforced in:**  
- src/protocol/state.rs  
- src/protocol/finality.rs

---

## Reviewer Note

Each invariant corresponds to a lemma in RFC-0001.
Breaking an invariant is a protocol violation.

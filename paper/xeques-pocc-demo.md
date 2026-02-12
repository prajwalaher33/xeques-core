# XEQUES — Proof-of-Command-Correctness (PoCC)

## Abstract
XEQUES introduces a Layer-1 primitive where the unit of trust is a cryptographically verifiable execution receipt created by the device itself.

## Protocol (high level)
1. Ground authority signs a command with a post-quantum signature.
2. Device verifies, executes, and emits a signed execution receipt (PQ).
3. Receipts are stored in an append-only log; auditors verify receipts independently.

## Threat model
- Replay / re-ordering attacks
- Signature forgery (classical & quantum)
- Compromised authority impersonation
- Device lifetime and offline verification

## Novelty
First to treat execution receipts as a Layer-1 primitive for autonomous infrastructure.

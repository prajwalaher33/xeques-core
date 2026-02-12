# XEQUES RFC-0001
## Proof-of-Command-Correctness (PoCC)

**Category:** Standards Track  
**Layer:** 1  
**Status:** Draft  
**Author:** Prajwal Aher  
**Year:** 2026  

---

## Abstract

This document defines XEQUES, a Layer-1 protocol introducing
**Proof-of-Command-Correctness (PoCC)** — a cryptographic primitive
where autonomous devices emit verifiable execution receipts
after executing authorized commands.

Unlike authorization systems or blockchains,
XEQUES treats *execution itself* as the unit of finality.

---

## 1. Scope

This specification defines:
- Command structure
- Receipt semantics
- Verification rules

It does **not** define:
- Consensus mechanisms
- Transport protocols
- Policy engines

---

## 2. Protocol Overview

1. An authority issues a signed command.
2. A device verifies authorization.
3. The device executes the command.
4. The device emits a signed execution receipt.

Only step (4) finalizes state.

---

## 3. Normative Language

The keywords **MUST**, **SHOULD**, and **MAY** are to be interpreted
as described in RFC 2119.

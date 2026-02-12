# XEQUES RFC-0003
## Reference Execution Specification

**Layer:** 1  
**Status:** Draft  
**Author:** Prajwal Aher  

---

## 1. State Machine

Each device maintains:

- DeviceID
- Sequence counter
- Execution log

---

## 2. Reference Algorithm


---

## 3. Determinism Requirement

For identical inputs, execution MUST produce identical receipts.

---

## 4. Failure Semantics

If execution fails:
- No receipt MUST be emitted
- Failure MAY be logged locally

---

## 5. Compliance

Any implementation following this spec
is considered XEQUES-compatible.

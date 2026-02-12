# XEQUES RFC-0002
## Security Model and Properties

**Layer:** 1  
**Status:** Draft  

---

## 1. Adversary Model

An adversary MAY:
- Observe all network traffic
- Replay or reorder commands
- Compromise command authorities
- Possess quantum computational capability

An adversary MUST NOT:
- Forge device execution receipts
- Produce receipts without execution

---

## 2. Security Properties

### SP-1: Execution Unforgeability
No valid receipt can exist without execution.

### SP-2: Authorization ≠ Execution
Authorization alone has no finality.

### SP-3: Replay Resistance
Receipts MUST bind monotonic sequence numbers.

### SP-4: Offline Verifiability
Verification MUST require no network access.

### SP-5: Post-Quantum Safety
All signatures MUST be PQ-secure.

---

## 3. Trust Shift

XEQUES relocates trust from controllers to devices.

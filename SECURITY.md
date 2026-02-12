# XEQUES Security Model

## Threat Model

Adversaries MAY:
- Observe all network traffic
- Replay previously issued commands
- Attempt command reordering
- Compromise issuing authorities
- Perform classical and quantum cryptanalysis

Adversaries MAY NOT:
- Forge post-quantum signatures without key compromise
- Produce valid execution receipts without execution
- Modify device-internal execution state

## Trust Assumptions

- Devices securely store private keys
- Cryptographic primitives are correctly implemented
- Execution environments are deterministic

## Non-Goals

- Byzantine fault tolerance
- Global ordering or consensus
- Smart contracts
- Financial settlement

XEQUES is an execution-finality layer, not a blockchain.

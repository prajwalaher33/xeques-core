# XEQUES Core

Quantum-Resilient Command Authentication Layer  
for Autonomous and Critical Infrastructure Systems.

## Overview

XEQUES Core implements:

- Post-Quantum Device Identity (Dilithium5)
- Post-Quantum Command Signing & Verification
- Replay Protection
- Federated Arbitration
- 2/3 Majority Consensus
- Receipt Verification
- CLI-configurable Node Deployment

Designed for:

- Satellites
- Drone Fleets
- Defense Systems
- Energy Infrastructure
- Autonomous Robotics

## Why Post-Quantum?

Long-lived autonomous systems must remain secure beyond the classical cryptographic era.

XEQUES prepares infrastructure for quantum-era resilience.

## Build

cargo build

## Example Run

cargo run -- --node-id node1 --bind 127.0.0.1:7001 --peers 127.0.0.1:7002,127.0.0.1:7003

---

Early-stage research and reference implementation.

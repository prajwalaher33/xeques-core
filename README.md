# XEQUES Core

![Rust](https://img.shields.io/badge/Rust-100%25-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![Version](https://img.shields.io/badge/Version-v0.1.0-green)
![Post-Quantum](https://img.shields.io/badge/Crypto-Post--Quantum-purple)

Quantum-Resilient Identity & Command Authentication Layer  
for Autonomous and Critical Infrastructure Systems.

---

## Overview

XEQUES Core implements:

- Post-Quantum Device Identity (Dilithium5)
- Post-Quantum Command Signing & Verification
- Replay Protection
- Federated Arbitration
- 2/3 Majority BFT Consensus
- Receipt Verification
- CLI-configurable Node Deployment

Designed for:

- Satellites  
- Drone Fleets  
- Defense Systems  
- Energy Infrastructure  
- Autonomous Robotics  

---

## Architecture

Ground Station → PQ Signed Command  
Node → Signature Verification  
Federation → 2/3 BFT Majority  
Receipt → Immutable Log  

---

## Why Post-Quantum?

Long-lived autonomous systems must remain secure beyond the classical cryptographic era.

XEQUES prepares infrastructure for quantum-era resilience.

---

## Build

cargo build

---

## Example Run

Start a 3-node local federation:

cargo run -- --node-id node1 --bind 127.0.0.1:7001 --peers 127.0.0.1:7002,127.0.0.1:7003

---

Reference implementation of a Quantum-Resilient Infrastructure Security Layer.

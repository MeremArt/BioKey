# Biokey 

Biokey is a secure on-chain identity protocol designed to empower users with decentralized identity management using biometrics. Built on the Solana blockchain, Biokey enables users to create accounts, validate their identity through fingerprint data, and retrieve stored information securely and efficiently.

---

## Table of Contents
1. [Features](#features)
2. [Getting Started](#getting-started)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Program Details](#program-details)


---

## Features

- **Account Creation:** Securely store fingerprint data on Solana via hashed representations.
- **Validation:** Confirm identity using stored fingerprint data.
- **Data Retrieval:** Fetch and verify user biometric data seamlessly.
- **Decentralized Protocol:** Designed to leverage Solana's high-speed, low-cost infrastructure.
- **Rust Program:** Secure, fast, and efficient backend implementation.


---

## Getting Started

### Prerequisites
- **Rust:** Install Rust and Cargo ([Rust installation guide](https://www.rust-lang.org/tools/install)).
- **Anchor Framework:** Ensure Anchor CLI is installed. Follow the [official guide](https://book.anchor-lang.com/getting_started/installation.html).
- **Solana CLI:** Install the Solana CLI for blockchain interactions ([Solana CLI installation guide](https://docs.solana.com/cli/install-solana-cli-tools)).

---

### Installation

1. **Clone the Repository**
   ```bash
   git clone https://github.com/MeremArt/biokey.git
   cd biokey


### Usage

Create User Account

Validate Fingerprint

Fetch Fingerprint

### Program Details

Program ID: HXEPyKzidcNAbnNWY1DMu433eBcN4nP1m381QdVKQe2S

IDL: Generated during anchor build and located in target/idl/biokey.json.

Cluster: Devnet (default).

### Key Methods

create_user_account: Initialize a new user account with biometric data.

validate_fingerprint: Check if the provided fingerprint matches stored data.

fetch_user_fingerprint: Retrieve the user's stored fingerprint.

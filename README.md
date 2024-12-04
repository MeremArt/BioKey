# Biokey Identity Protocol

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
- **User Account Creation:** Create accounts using hashed fingerprint data stored securely on Solana.
- **Fingerprint Validation:** Verify user identity through stored biometric data.
- **Data Retrieval:** Fetch and display user-specific biometric data.
- **Decentralized Architecture:** Leverages Solana's high-performance blockchain for reliability and scalability.


---

## Getting Started

### Prerequisites
- **Node.js:** Ensure you have Node.js (>=16) installed.  
- **Solana CLI:** Install the Solana CLI for blockchain interactions.
- **Wallet Adapter:** Use a supported Solana wallet.

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

IDL Location: ./biokey.json

Cluster: Devnet (Default)

### Program Structure

Account Creation: Stores hashed fingerprint data on-chain.
Fingerprint Validation: Matches provided fingerprint data against stored data.
Fingerprint Fetching: Retrieves fingerprint data for UI display.


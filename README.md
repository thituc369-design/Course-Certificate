# Project Title

Course Certificate – An Educational Certificate Issuing Soroban Smart Contract dApp on Stellar

## Project Vision

This project demonstrates how to issue, verify, and manage educational certificates on the Stellar blockchain using Soroban smart contracts. It provides:
- How to write a Soroban smart contract in Rust
- How to manage persistent storage for certificate records
- How to handle certificate verification on-chain
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a reliable system for educational institutions to issue tamper-proof certificates on Stellar.

---

## Description

A Soroban smart contract dApp that allows educational institutions to issue, verify, and revoke certificates on Stellar Testnet. Each certificate contains student info, course name, grade, and issue date stored permanently on-chain.

---

## Features

### 1. Issue Certificates
- Institution can issue new certificates with unique cert_id
- Each certificate stores: student address, course name, grade, issue date
- On-chain storage for permanent record keeping

### 2. Verify Certificates
- Anyone can verify a certificate's validity
- Returns student info, course name, grade, and issue date
- Automatically checks if certificate has been revoked

### 3. Revoke Certificates
- Institution can revoke previously issued certificates
- Revocation status is stored on-chain
- Verification fails for revoked certificates

### 4. Get Certificate Details
- Retrieve full certificate information by cert_id
- Includes revoked status for complete verification

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CDNTTFL2DB6HRHZXWCQOTEI247TBAIQOA4QJEOMSO5UYT45ID4QP5I3D](https://stellar.expert/explorer/testnet/tx/63639ac37c0a3c70f56a9db5cc104add9ef7c4079dffd7cb150b166e2fa24450)

![screenshot](https://i.ibb.co/nMcnxxkw/image.png)

---

## Future Scopes

### 1. Institution Authentication
- Add multi-signature support for institution authorization
- Role-based access control for different operations

### 2. Certificate Metadata
- Add more fields like completion date, instructor name, credits
- IPFS integration for storing certificate documents

### 3. Batch Operations
- Support batch issuing and revocation of certificates
- Bulk verification for multiple certificates

### 4. Frontend dApp
- Build a web interface for institutions to manage certificates
- Student portal for viewing and sharing certificates

### 5. Certificate Standards
- Implement standard certificate schemas
- Support for different certificate types and formats

### 6. Analytics Dashboard
- Track total certificates issued, revoked
- Institution statistics and reporting

---

## Profile

- **Name:** thituc369

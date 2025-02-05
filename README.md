# Token Escrow Full Stack Dapp

## Project Description

The **Token Escrow Full Stack Dapp** is a decentralized application (DApp) built on the **MultiversX blockchain**. This project enables secure and trustless token exchanges between users using an escrow system. It ensures that funds or assets are only released when predefined conditions are met, eliminating the need for intermediaries in P2P transactions.

Participants can create, accept, or cancel trade offers, making it a robust solution for safe token exchange.

## Project Objectives

- Develop a smart contract to manage token escrow transactions.
- Provide a secure and decentralized mechanism for exchanging tokens between users.
- Enable users to create trade offers, accept them, or cancel them.
- Deploy and test the application on the **MultiversX Testnet**.
- Build a user-friendly interface for interacting with the token escrow system.

## Project Features

### Token Escrow Smart Contract
- **Create Offer**: Users deposit tokens and define conditions for release, specifying the recipient and required exchange terms.
- **Cancel Offer**: Users can cancel an offer and recover their deposited tokens if the offer is not yet accepted.
- **Accept Offer**: The recipient can accept the offer, meet the terms, complete the transaction, and claim the tokens.
- **View Offers**: Users can view a list of pending trade offers created by or targeted at them.

### Smart Contract Functions
- **createOffer**: Stores the offer on the blockchain and locks the deposited tokens in escrow.
- **cancelOffer**: Cancels the specified offer and returns the locked tokens to the creator.
- **acceptOffer**: Completes the trade and transfers assets to both parties.
- **getOffers**: Retrieves all open offers for a given user (either created by or targeted at them).

### Basic User Interface (UI)

- **Wallet Integration**: Connect a **MultiversX-compatible wallet** for seamless interaction with the blockchain.
- **Offer Management**: Interface for creating, viewing, and canceling offers.
- **Trade Dashboard**: Displays open offers and transaction history for transparency.
- **Notifications**: Users will be notified of trade updates (e.g., offer acceptance or cancellation).
- **Security Checks**: Users must confirm each action (e.g., creating or accepting an offer).

## MultiversX Testnet Deployment

This application will be deployed on the **MultiversX Testnet** for testing purposes. The DApp's functionalities, including **fund locking**, **offer cancellation**, and **trade completion**, should be verified on the testnet before finalizing the project.

## Installation and Setup

To run this project locally, follow these steps:

### Prerequisites
- Node.js (v16+ recommended)
- npm (or yarn)
- MultiversX-compatible wallet (e.g., MultiversX Wallet)

### 1. Clone the repository

```bash
git clone https://github.com/AdilHaydar/mx-escrow.git
cd token-escrow-dapp

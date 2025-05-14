This program contains the voting dapp and multisig

# voting_dapp
The decentralized polling application built on the Solana blockchain, allowing users to create polls, register candidates, and vote in a transparent and secure manner. 

# A Decentralized Polling dApp

**Voting Dapp** is a decentralized polling application built on the Solana blockchain, allowing users to create polls, register candidates, and vote in a transparent and secure manner. It leverages modern technologies and blockchain infrastructure to provide an interactive and decentralized voting experience.

## Features

- **Create Polls**: Users can create polls with descriptions, start dates and end dates.
- **Register Candidates**: Candidates can register to participate in active polls.
- **Vote on Polls**: Users can cast votes for registered candidates in active polls.


## License
This project is licensed under the MIT License. See the LICENSE file for more details.


# multisig program

The Multisig Program is a Solana program built with Anchor, enabling secure multi-signature (multisig) governance for managing SOL transfers and configuration changes. It allows a group of authorized signers, coordinated by an admin, to propose, approve, and execute transactions (transfers or threshold changes) with a configurable approval threshold. Funds are held in a program-derived escrow account, ensuring trustless execution once conditions are met.

## Features
**Multisig Initialization:** Set up a multisig with an admin, a list of authorized signers, and a configurable approval threshold. An associated escrow account is created to hold funds.

**Transaction Proposals:** Authorized signers or the admin can propose two types of transactions:
- Transfer: Move SOL from the escrow to a specified recipient.
- Threshold Change: Update the number of approvals required for execution.

**Flexible Approvals:** Admin or signers can approve proposed transactions. Approvals can be auto-added during proposal or submitted separately.

**Approval Management:** The admin can revoke (delete) approvals to reset or correct the process.

**Secure Execution:** Transactions execute only when the approval threshold is met, with nonce-based replay protection and PDA validation.

**Unified Instructions:** Single propose, approve, and execute instructions handle both transfer and threshold change operations, optimized for efficiency.




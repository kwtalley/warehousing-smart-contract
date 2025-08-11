# Warehousing Smart Contract

A CosmWasm smart contract for the Provenance blockchain that facilitates warehousing and lending operations through a pledge-based system. This contract manages the lifecycle of pledges from creation through approval and paydown processes.

## Overview

The Warehousing Smart Contract enables users to create pledges against assets held in Provenance markers, with a complete workflow that includes:

- **Pledge Creation**: Originators can create pledges against their assets
- **Pledge Approval**: Lenders can approve pledges by providing the required funds
- **Paydown Request**: Originators can request to pay down their pledges
- **Paydown Approval**: Lenders can approve paydown requests

## Contract Architecture

### State Management

The contract maintains the following state:

- **Contract Configuration**: Denomination and NFT class ID settings
- **Pledges**: Map of pledge IDs to pledge data including amounts, marker addresses, and status
- **NFT Management**: Originator and lender NFTs that represent ownership and rights

### Pledge Status Flow

```
Pledged → Approved → PaydownRequested → PaydownApproved
```

## Message Flow

### Instantiate

Initialize the contract with the required configuration:

```rust
InstantiateMsg {
    denom: String,           // The denomination for pledges
    nft_class_id: String,    // The NFT class ID for representing pledges
}
```

### Execute Messages

#### 1. Pledge

Create a new pledge against assets in a marker:

```rust
ExecuteMsg::Pledge {
    amount: Coin,           // Amount to pledge
    id: String,            // Unique pledge identifier
    marker_addr: String,   // Address of the marker holding assets
}
```

**Flow:**
- Validates marker permissions and holdings
- Creates pledge record with `Pledged` status
- Mints originator NFT and transfers to sender
- Stores pledge in contract state

#### 2. Approve Pledge

Approve a pledge by providing the required funds:

```rust
ExecuteMsg::ApprovePledge {
    pledge_id: String,     // ID of the pledge to approve
}
```

**Flow:**
- Validates pledge status is `Pledged`
- Verifies sent funds match pledge amount
- Updates pledge status to `Approved`
- Mints lender NFT and transfers to sender

#### 3. Paydown

Request to pay down an approved pledge:

```rust
ExecuteMsg::Paydown {
    pledge_id: String,     // ID of the pledge to pay down
}
```

**Flow:**
- Validates pledge status is `Approved`
- Updates pledge status to `PaydownRequested`

#### 4. Approve Paydown

Approve a paydown request:

```rust
ExecuteMsg::ApprovePaydown {
    pledge_id: String,     // ID of the pledge to approve paydown for
}
```

**Flow:**
- Validates pledge status is `PaydownRequested`
- Updates pledge status to `PaydownApproved`
- Processes the paydown transaction

### Query Messages

Currently, no query messages are implemented, but the contract state can be queried directly.

## Transaction Examples

### Instantiate Contract

```bash
provenanced tx wasm instantiate <CODE_ID> \
  '{
    "denom": "nhash",
    "nft_class_id": "warehouse-pledge-nft"
  }' \
  --label "warehousing-contract" \
  --admin <ADMIN_ADDRESS> \
  --from <SENDER_ADDRESS> \
  --gas auto \
  --gas-adjustment 1.3 \
  --chain-id <CHAIN_ID>
```

### Execute Transactions

#### 1. Create a Pledge

```bash
provenanced tx wasm execute <CONTRACT_ADDRESS> \
  '{
    "pledge": {
      "amount": {
        "amount": "1000000000",
        "denom": "nhash"
      },
      "id": "pledge-001",
      "marker_addr": "tp1marker1234567890abcdef"
    }
  }' \
  --amount 1000000000nhash \
  --from <ORIGINATOR_ADDRESS> \
  --gas auto \
  --gas-adjustment 1.3 \
  --chain-id <CHAIN_ID>
```

#### 2. Approve a Pledge

```bash
provenanced tx wasm execute <CONTRACT_ADDRESS> \
  '{
    "approve_pledge": {
      "pledge_id": "pledge-001"
    }
  }' \
  --amount 1000000000nhash \
  --from <LENDER_ADDRESS> \
  --gas auto \
  --gas-adjustment 1.3 \
  --chain-id <CHAIN_ID>
```

#### 3. Request Paydown

```bash
provenanced tx wasm execute <CONTRACT_ADDRESS> \
  '{
    "paydown": {
      "pledge_id": "pledge-001"
    }
  }' \
  --from <ORIGINATOR_ADDRESS> \
  --gas auto \
  --gas-adjustment 1.3 \
  --chain-id <CHAIN_ID>
```

#### 4. Approve Paydown

```bash
provenanced tx wasm execute <CONTRACT_ADDRESS> \
  '{
    "approve_paydown": {
      "pledge_id": "pledge-001"
    }
  }' \
  --from <LENDER_ADDRESS> \
  --gas auto \
  --gas-adjustment 1.3 \
  --chain-id <CHAIN_ID>
```

### Query Contract State

#### Get Contract Configuration

```bash
provenanced query wasm contract-state smart <CONTRACT_ADDRESS> \
  '{"contract_config": {}}' \
  --chain-id <CHAIN_ID>
```

#### Get Pledge Information

```bash
provenanced query wasm contract-state smart <CONTRACT_ADDRESS> \
  '{"pledges": {"key": "pledge-001"}}' \
  --chain-id <CHAIN_ID>
```

## Development

### Prerequisites

- Rust 1.70+
- Cargo
- Provenance blockchain access

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Optimization

The project includes a Docker-based optimization script for CosmWasm contracts:

```bash
cargo run-script optimize
```

## Deployment

This contract is designed for deployment on the Provenance blockchain. Ensure you have:

1. Access to a Provenance node
2. Proper marker permissions configured
3. NFT class created for pledge representation

## License

This project is licensed under the terms specified in the LICENSE file.

## Contributing

Please follow the existing code style and ensure all tests pass before submitting changes.

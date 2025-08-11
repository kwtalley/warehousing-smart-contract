#!/bin/bash

# Smart Contract Deployment and Instantiation Demo
# This script stores and instantiates the warehousing smart contract using the validator account

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration for local instance (make run)
CHAIN_ID="testing"
NODE="http://localhost:26657"
GAS_PRICES="1nhash"
GAS_ADJUSTMENT="1.5"

# Contract configuration
CONTRACT_NAME="warehousing-smart-contract"
CONTRACT_DENOM="nhash"
CONTRACT_NFT_CLASS_ID="warehouse-nft-class"

VALIDATOR_ADDRESS=$("provenanced" keys show -a validator --keyring-backend test --testnet )

# Function to print section headers
print_section() {
    echo -e "${YELLOW}$1${NC}"
    echo -e "${YELLOW}$(printf '=%.0s' {1..${#1}})${NC}"
    echo ""
}

# Function to print command information
print_cmd() {
    local cmd="$1"
    local description="$2"
    
    echo -e "${GREEN}Executing:${NC} $description"
    echo -e "${BLUE}Command:${NC} $cmd"
    echo ""
}

# Function to execute commands
execute_cmd() {
    local cmd="$1"
    
    # Execute the command and capture output
    local output
    output=$(eval "$cmd")
    local exit_code=$?
    
    # Return the output
    echo "$output"
    return $exit_code
}

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Smart Contract Deployment Demo${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Step 1: Build the contract
print_section "Step 1: Build the Contract"

echo "Building and Optimizing the smart contract..."
BUILD_CMD="cargo run-script optimize"
print_cmd "$BUILD_CMD" "Build and Optimize the smart contract WASM binary"
execute_cmd "$BUILD_CMD"

# Step 2: Store the contract and get code ID
print_section "Step 2: Store the Contract and Get Code ID"

echo "Storing the smart contract on the blockchain..."
STORE_CMD="provenanced tx wasm store artifacts/warehousing_smart_contract.wasm \
        --instantiate-anyof-addresses "$VALIDATOR_ADDRESS" \
        --from="$VALIDATOR_ADDRESS" \
        --keyring-backend test \
        --chain-id $CHAIN_ID \
        --node $NODE \
        --gas auto \
        --gas-prices $GAS_PRICES \
        --gas-adjustment $GAS_ADJUSTMENT \
        --testnet \
        --yes | provenanced q wait-tx"
print_cmd "$STORE_CMD" "Store the smart contract using validator account"
STORE_OUTPUT=$(execute_cmd "$STORE_CMD")

echo "the output=========$STORE_OUTPUT"

CODE_ID=$(echo "$STORE_OUTPUT" | yq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
echo -e "${GREEN}Contract stored with code ID:${NC} $CODE_ID"

# Step 3: Instantiate the contract
print_section "Step 3: Instantiate the Contract"

echo "Instantiating the smart contract..."
INSTANTIATE_CMD="provenanced tx wasm instantiate $CODE_ID \
        '{\"denom\": \"$CONTRACT_DENOM\", \"nft_class_id\": \"$CONTRACT_NFT_CLASS_ID\"}' \
        --admin validator \
        --label \"$CONTRACT_NAME\" \
        --from validator \
        --keyring-backend test \
        --home $PIO_HOME \
        --chain-id $CHAIN_ID \
        --node $NODE \
        --gas auto \
        --gas-prices $GAS_PRICES \
        --gas-adjustment $GAS_ADJUSTMENT \
        --testnet \
        --yes | provenanced q wait-tx"
print_cmd "$INSTANTIATE_CMD" "Instantiate the smart contract with denom and nft_class_id parameters"
execute_cmd "$INSTANTIATE_CMD"

# Step 4: Get the contract address
print_section "Step 4: Get Contract Address"

echo "Getting the contract address..."
CONTRACT_ADDRESS=$(provenanced query wasm list-contract-by-code $CODE_ID --node $NODE --home $PIO_HOME --testnet --output json | jq -r '.contracts[0]')
echo -e "${GREEN}Contract instantiated at address:${NC} $CONTRACT_ADDRESS"

# Step 5: Query the contract
print_section "Step 5: Query the Contract"

echo "Querying the contract configuration..."
QUERY_CMD="provenanced query wasm contract-state smart \
        $CONTRACT_ADDRESS \
        '{}' \
        --node $NODE \
        --home $PIO_HOME \
        --testnet"
print_cmd "$QUERY_CMD" "Query the contract to verify instantiation"
execute_cmd "$QUERY_CMD"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Contract Deployment Completed!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${BLUE}Summary of operations performed:${NC}"
echo "✓ Built the smart contract WASM binary"
echo "✓ Optimized the WASM binary for deployment"
echo "✓ Stored the contract on the blockchain using validator account"
echo "✓ Retrieved the contract code ID"
echo "✓ Instantiated the contract with proper parameters"
echo "✓ Retrieved the contract address"
echo "✓ Verified the contract deployment with a query"
echo ""
echo -e "${YELLOW}Note:${NC} This script is configured to run against a local Provenance instance started with 'make run'."
echo -e "${YELLOW}Contract Parameters:${NC}"
echo "  - Denom: $CONTRACT_DENOM"
echo "  - NFT Class ID: $CONTRACT_NFT_CLASS_ID"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  - Use the contract address to interact with the deployed contract"
echo "  - Implement and test the contract's execute and query functions" 
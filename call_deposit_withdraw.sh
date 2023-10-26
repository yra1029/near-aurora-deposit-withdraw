#!/bin/bash
set -e -x

near_key_path=~/.near-credentials/testnet/cross-con.testnet.json
aurora_key=$(cat aurora-key.txt)

cat aurora-deposit-withdraw/out/DepositWithdrawContract.sol/DepositWithdrawContract.json | jq '.abi' > aurora-deposit-withdraw/out/DepositWithdrawContract.sol/DepositWithdrawContract.abi
cat aurora-deposit-withdraw/out/IERC20.sol/IERC20.json | jq '.abi' > aurora-deposit-withdraw/out/IERC20.sol/IERC20.abi

# BalanceOf call
#aurora-cli --network testnet  view-call --address 0x08d8D2D845BC68DFd30b399F5D72623baB216C2E --function balanceOf --abi-path aurora-deposit-withdraw/out/IERC20.sol/IERC20.abi --args '{"account": "0x785cDb4d8f0C360037ca523b659EAa18287bA592"}'

# Approval call for wbtc
#aurora-cli --network testnet --near-key-path $near_key_path call --aurora-secret-key $aurora_key --address 0x3080bB2F96Bc364c657Bc6aEd45B4FFE324b03b4 --function approve --abi-path aurora-deposit-withdraw/out/IERC20.sol/IERC20.abi --args '{"spender":"0x4f314158b813f600e6579b33630cd07d761c8464", "amount":"10000000000"}'

# Approval call for wNEAR
#aurora-cli --network testnet --near-key-path $near_key_path call --aurora-secret-key $aurora_key --address 0x4861825E75ab14553E5aF711EbbE6873d369d146 --function approve --abi-path aurora-deposit-withdraw/out/IERC20.sol/IERC20.abi --args '{"spender":"0x4f314158b813f600e6579b33630cd07d761c8464", "amount":"20000000000000000000000000"}'
# Transaction for approval https://explorer.testnet.aurora.dev/tx/0xd4f2c4e7ea4a226a478f6e69334f9bbde17f6108688fcd5779a1c97d68f05585

# Check allowance call
#aurora-cli --network testnet  view-call --address 0x08d8D2D845BC68DFd30b399F5D72623baB216C2E --function allowance --abi-path aurora-deposit-withdraw/out/IERC20.sol/IERC20.abi --args '{"owner": "0x785cDb4d8f0C360037ca523b659EAa18287bA592", "spender":"0x4f314158b813f600e6579b33630cd07d761c8464"}'
# Transaction for approval https://explorer.testnet.aurora.dev/tx/0x95ea05a861131762fe013305233c07495fd71d47f11ddc490be3a49fe63ce7f3

# Storage deposit call
#aurora-cli --network testnet --near-key-path $near_key_path call --aurora-secret-key $aurora_key --address 0x4f314158b813f600e6579b33630cd07d761c8464 --function storageDeposit --abi-path aurora-deposit-withdraw/out/DepositWithdrawContract.sol/DepositWithdrawContract.abi --args '{"registrationOnly": "true"}' --value 100000000000000
# Transaction for storage deposit https://explorer.testnet.aurora.dev/tx/0x4026eec0defc77529d76d6c3ad679be642419abcf8ba33d81063cc37cbad770f

# Deposit call
#aurora-cli --network testnet --near-key-path $near_key_path call --aurora-secret-key $aurora_key --address 0x4f314158b813f600e6579b33630cd07d761c8464 --function deposit --abi-path aurora-deposit-withdraw/out/DepositWithdrawContract.sol/DepositWithdrawContract.abi --args '{"token":"0x3080bB2F96Bc364c657Bc6aEd45B4FFE324b03b4", "tokenIdOnNear":"wbtc.test-token.orderly-spot.testnet", "amount":"10000000000"}'

# Withdraw call
#aurora-cli --network testnet --near-key-path $near_key_path call --aurora-secret-key $aurora_key --address 0x4f314158b813f600e6579b33630cd07d761c8464 --function withdraw --abi-path aurora-deposit-withdraw/out/DepositWithdrawContract.sol/DepositWithdrawContract.abi --args '{"tokenIdOnNear":"wbtc.test-token.orderly-spot.testnet", "amount":"10000000000"}'
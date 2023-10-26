#!/bin/bash
set -e -x

near_key_path=~/.near-credentials/testnet/xytest.testnet.json
aurora_key=$(cat aurora-key.txt)

# near --network_id testnet deploy xytest.testnet ./res/near_deposit_withdraw.wasm
# Deployment transaction https://explorer.testnet.near.org/transactions/5kX23v4eCvDtB7Eiwa6FHDRkhRP8SWGFHPPHhmZnvHE1

# near --network_id testnet call xytest.testnet new '{"aurora_engine": "aurora"}' --account-id xytest.testnet
# Initialization transaction https://explorer.testnet.near.org/transactions/GyDQ6ZERPPig9VZP7DBrkBXf4XavzHumennBmWKStxXa

# cd aurora-deposit-withdraw
# Deploy Codec library
# code=$(cat out/Codec.sol/Codec.json | jq '.bytecode .object' | cut -c 4- | rev | cut -c 2- | rev)
# aurora-cli --network testnet --near-key-path $near_key_path deploy --code $code --aurora-secret-key $aurora_key
# Codec address: 0xd5215ba7808c642c02aa4643b640d87b165e2831

# Deploy Utils library
# code=$(cat out/Utils.sol/Utils.json | jq '.bytecode .object' | cut -c 4- | rev | cut -c 2- | rev)
# aurora-cli --network testnet --near-key-path $near_key_path deploy --code $code --aurora-secret-key $aurora_key
# # Utils address: 0x1c3474bbd890351eaf85a582f28d63333f5178da

# Deploy AuroraSdk library
# forge build --libraries aurora-sdk/Codec.sol:Codec:0xd5215ba7808c642c02aa4643b640d87b165e2831 --libraries aurora-sdk/Utils.sol:Utils:0x1c3474bbd890351eaf85a582f28d63333f5178da
# code=$(cat out/AuroraSdk.sol/AuroraSdk.json | jq '.bytecode .object' | cut -c 4- | rev | cut -c 2- | rev)
# aurora-cli --network testnet --near-key-path $near_key_path deploy --code $code --aurora-secret-key $aurora_key
# AuroraSdk address: 0xc864444ae098057998bb94c9952ac899aea1ab4f

# Deploy the contract
# forge clean && forge build --libraries aurora-sdk/Codec.sol:Codec:0xd5215ba7808c642c02aa4643b640d87b165e2831 --libraries aurora-sdk/AuroraSdk.sol:AuroraSdk:0xc864444ae098057998bb94c9952ac899aea1ab4f
# code=$(cat out/DepositWithdrawContract.sol/DepositWithdrawContract.json | jq '.bytecode .object' | cut -c 4- | rev | cut -c 2- | rev)
# cat out/DepositWithdrawContract.sol/DepositWithdrawContract.json | jq '.abi' > out/DepositWithdrawContract.sol/DepositWithdrawContract.abi
# aurora-cli --network testnet --near-key-path $near_key_path deploy --code $code --abi-path out/DepositWithdrawContract.sol/DepositWithdrawContract.abi --args '{"_nearContractId":"xytest.testnet", "_wNEAR": "4861825E75ab14553E5aF711EbbE6873d369d146"}' --aurora-secret-key $aurora_key
# DepositWithdrawContract address 0x4f314158b813f600e6579b33630cd07d761c8464
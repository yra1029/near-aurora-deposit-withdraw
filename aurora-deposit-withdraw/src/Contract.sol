// SPDX-License-Identifier: CC-BY-1.0
pragma solidity ^0.8.17;

import 'openzeppelin-contracts/access/AccessControl.sol';
import 'openzeppelin-contracts/token/ERC20/IERC20.sol';
import 'openzeppelin-contracts/utils/Strings.sol';
import './IEvmErc20.sol';
import { AuroraSdk, Codec, NEAR, PromiseCreateArgs, PromiseResult, PromiseResultStatus, PromiseWithCallback } from 'aurora-sdk/AuroraSdk.sol';

uint64 constant DEPOSIT_NEAR_GAS = 40_000_000_000_000;
uint64 constant WITHDRAW_NEAR_GAS = 50_000_000_000_000;
uint64 constant CALLBACK_NEAR_GAS = 15_000_000_000_000;

contract DepositWithdrawContract is AccessControl {
  using AuroraSdk for NEAR;
  using AuroraSdk for PromiseCreateArgs;
  using AuroraSdk for PromiseWithCallback;
  using Codec for bytes;

  bytes32 public constant CALLBACK_ROLE = keccak256('CALLBACK_ROLE');

  IERC20 public wNEAR;
  string public nearContractId;
  NEAR public near;

  uint128 private _weiToYoctoNearMultiplier;

  constructor(string memory _nearContractId, IERC20 _wNEAR) {
    nearContractId = _nearContractId;
    near = AuroraSdk.initNear(_wNEAR);
    wNEAR = _wNEAR;
    _weiToYoctoNearMultiplier = 1_000_000_000; // 1k Near per Eth; Near decimals 24; Eth decimals 18
    _grantRole(
      CALLBACK_ROLE,
      AuroraSdk.nearRepresentitiveImplicitAddress(address(this))
    );
  }

  function deposit(
    IEvmErc20 token,
    string memory tokenIdOnNear,
    uint128 amount
  ) public {
    token.transferFrom(msg.sender, address(this), amount);
    token.withdrawToNear(
      abi.encodePacked(AuroraSdk.nearRepresentative(address(this))),
      uint(amount)
    );

    bytes memory data = abi.encodePacked(
      '{',
      '"receiver_id": "',
      nearContractId,
      '",',
      '"amount": "',
      Strings.toString(amount),
      '",',
      '"msg": "{\\"xcc_account_id\\": {\\"chain\\": \\"Aurora\\", \\"account_id\\": \\"',
      Strings.toHexString(uint160(msg.sender), 20),
      '\\"}',
      '}'
    );
    PromiseCreateArgs memory callFtTransfer = near.call(
      tokenIdOnNear,
      'ft_transfer_call',
      data,
      1,
      DEPOSIT_NEAR_GAS
    );
    PromiseCreateArgs memory callback = near.auroraCall(
      address(this),
      abi.encodePacked(this.depositCallback.selector),
      0,
      CALLBACK_NEAR_GAS
    );

    callFtTransfer.then(callback).transact();
  }

  function depositCallback() public onlyRole(CALLBACK_ROLE) {
    if (AuroraSdk.promiseResult(0).status != PromiseResultStatus.Successful) {
      revert('Call to ft_transfer_call failed');
    }
  }

  function withdraw(string memory tokenIdOnNear, uint128 amount) public {
    bytes memory data = abi.encodePacked(
      '{',
      '"token_id": "',
      tokenIdOnNear,
      '", "amount": "',
      Strings.toString(amount),
      '", "xcc_account_id": { "chain": "Aurora", "account_id": "',
      Strings.toHexString(uint160(msg.sender), 20),
      '"}}'
    );
    PromiseCreateArgs memory callWithdraw = near.call(
      nearContractId,
      'withdraw_to_aurora_acc',
      data,
      0,
      WITHDRAW_NEAR_GAS
    );
    PromiseCreateArgs memory callback = near.auroraCall(
      address(this),
      abi.encodePacked(this.withdrawCallback.selector),
      0,
      CALLBACK_NEAR_GAS
    );

    callWithdraw.then(callback).transact();
  }

  function withdrawCallback() public onlyRole(CALLBACK_ROLE) {
    if (AuroraSdk.promiseResult(0).status != PromiseResultStatus.Successful) {
      revert('Call to withdraw failed');
    }
  }
}

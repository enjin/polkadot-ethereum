// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6;
pragma experimental ABIEncoderV2;

abstract contract InboundChannel {
    uint64 public nonce;
    uint32 public blockNumber;

    struct Message {
        address target;
        uint64 nonce;
        bytes payload;
    }

    event MessageDispatched(uint64 nonce, bool result);

    function submit(uint32 _blockNumber, Message[] calldata _messages, bytes32 _commitment)
        public
        virtual;
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Hello {
    string public greeting;

    constructor() {
        greeting = "Hello, World!";
    }

    function setGreeting(string memory newGreeting) public {
        greeting = newGreeting;
    }

    function getGreeting() public view returns (string memory) {
        return greeting;
    }
}
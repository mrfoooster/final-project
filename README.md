Web3 Voting app ? - README

Hello, this is a basic voting contract, atleast that is what i aimed while attempting on this

    Overview
    Getting Started
        Prerequisites
        Installation
    Usage
    Smart Contracts
    Testing
    Frontend
    Contributing
    License

Overview

This small contract provides the users having a voting process, followed by vote count to determine how many votes received by the options

Getting Started

Prerequisites

    Rust

    Codigo account

    TypeScript

Installation

    Clone the repository:

  git clone https://github.com/mrfoooster/final-project.git

    Navigate to the project directory:

  cd /program

    Type, codigo solana generate <file_name>.yaml to the terminal

    After the backend files are created based on the yaml file, modify them with the files in program/src 

    When the backend .rs files are established, redirect folder to /program and type "cargo build-sbf" for cargo build

    If there are errors, debug them. When you no longer receive errors, type the terminal "solana config set --url devnet" 

    To get some balance to the solana, type "solana airdrop 1"

    Check balance with 'solana balance'

    To deploy: " solana program deploy target/deploy/nft.so "

    For front end, 
    

Smart Contracts

The smart contracts in this project facilitate voting process. It handles, voting decisions, and their results.

    initialize.rs: starts the voting process
    vote.rs      : takes the votes of the users
    get_results  : gives the results of the voting process

Testing

Smart contract tests are located in the test folder. These tests ensure the correct functioning of the smart contract. 

Frontend

This small smart contract was created by usage of Rust for backend, typescript for frontend tests and solana for contract deployment. Codigo was used for the yaml deployment


    Rust.rs
    Typescript.ts
    Solana
    Codigo
    
Contributing

Contributions to this project are welcome! To contribute:

    Fork the repository.
    Create a new branch for your feature/bug fix.
    Make changes and test thoroughly.
    Commit with clear and concise messages.
    Push changes to your fork.
    Submit a pull request describing your changes.

License

This project is not licensed.

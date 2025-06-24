# Week 2 Exercise 8: Solana Programs

Using the examples in the [repo](https://github.com/ExtropyIO/SolanaBootcamp/tree/main/examples_baremetal)

1. Modify the existing msg! in example1-helloworld to log the program ID. 

Adding "program_id.to_string()" to line 14 of the [example](https://github.com/circaplastic/SolanaBootcamp/blob/main/examples_baremetal/example1-helloworld/rust/src/lib.rs) produced the following logs:

```bash
~$ solana logs
Streaming transaction logs. Confirmed commitment
Transaction executed in slot 733:
  Signature: 22eoWV28DcqugVEDosHMpuhSJjz7wBwTm2i45Bzczgb9Jumivej8ERvqERK7PFBMqfWEedCQHq3uKyx1cC9YyuyP
  Status: Ok
  Log Messages:
    Program DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5 invoke [1]
    **Program log: [lib] Hello World Rust program entrypoint, Program ID is: DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5**
    Program DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5 consumed 11851 of 200000 compute units
    Program DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5 success
```

2. Retrieve the total program size of example1-helloworld:

**Data Length: 39776 (0x9b60) bytes**

```bash
$ solana program show DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5
Program Id: DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: A5C4PWscQjjiNtRxqxjgyKJdEigE9f9s8uzzdwqTbFEe
Authority: DwFgED8ZcztuT4FourTdcDu5tAGrZPMXfjVbLbcMBCHf
Last Deployed In Slot: 576
**Data Length: 39776 (0x9b60) bytes**
Balance: 0.27804504 SOL
```
It is important to notice that the command to obtain data about the program itself is solana program show <program_id> which produces the data related to the ProgramData Account, if the command solana account <program_id> is used the information retrieved will be about the Program Account that contains metadata and executable code of the program, but not about the program itself.

3. Retrieve the lamport balance of example2-counter.

**Once it has been deployed the lamport balance of the program is 433280880 lamports (1 SOL == 1000,000,000 lamports)**
```bash
$ solana program show CUWoY9MMnFQNBx3UtF6JY8WpJNxnh6kg7JzxYWzrzGQ6
Program Id: CUWoY9MMnFQNBx3UtF6JY8WpJNxnh6kg7JzxYWzrzGQ6
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: 4mJbRry9279puCf9pbCUvnbR81pYemjZgMei3zHsfgnN
Authority: DwFgED8ZcztuT4FourTdcDu5tAGrZPMXfjVbLbcMBCHf
Last Deployed In Slot: 21367
Data Length: 62080 (0xf280) bytes
Balance: 0.43328088 SOL
```

4. Modify the client for example2-counter to feed an incorrect address for the greeting account to the program.

The modification to the client was an insertion in lines 85 and 88 of a different seed named WRONG_SEED which can be found [here](https://github.com/circaplastic/SolanaBootcamp/blob/main/examples_baremetal/example2-counter/client/main.ts); the error it produces is the following:

```bash
$ npm run call:2

> solana-course@0.0.1 call:2
> ts-node examples_baremetal/example2-counter/client/main.ts

Let's increment counter for an account!

local system client config location:  /home/pvepn/.config/solana/cli/config.yml
(node:8874) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)

local system client config location:  /home/pvepn/.config/solana/cli/config.yml
Connection to cluster established: http://localhost:8899 { 'feature-set': 607245837, 'solana-core': '2.0.16' }
Program ID account:  CUWoY9MMnFQNBx3UtF6JY8WpJNxnh6kg7JzxYWzrzGQ6
Account A8ReBk7yjpt9BsxnXavWY923rRFrsKUJYxSkiPXoxs9w not deployed, deploying now
Creating account A8ReBk7yjpt9BsxnXavWY923rRFrsKUJYxSkiPXoxs9w to say hello to
SendTransactionError: Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x5.
Logs:
[
  "Program 11111111111111111111111111111111 invoke [1]",
  "Create: address A8ReBk7yjpt9BsxnXavWY923rRFrsKUJYxSkiPXoxs9w does not match derived address 34Do48RDmjj1YDxBf5B3QXeXc5ZufoWjhYRWzXD7xVyJ",
  "Program 11111111111111111111111111111111 failed: custom program error: 0x5"
].
Catch the `SendTransactionError` and call `getLogs()` on it for full details.
    at Connection.sendEncodedTransaction (/home/pvepn/SolanaBootcamp/node_modules/@solana/web3.js/src/connection.ts:6047:13)
    at processTicksAndRejections (node:internal/process/task_queues:105:5)
    at async Connection.sendRawTransaction (/home/pvepn/SolanaBootcamp/node_modules/@solana/web3.js/src/connection.ts:6003:20)
    at async Connection.sendTransaction (/home/pvepn/SolanaBootcamp/node_modules/@solana/web3.js/src/connection.ts:5991:12)
    at async sendAndConfirmTransaction (/home/pvepn/SolanaBootcamp/node_modules/@solana/web3.js/src/utils/send-and-confirm-transaction.ts:36:21)
    at async deployGreetAccount (/home/pvepn/SolanaBootcamp/examples_baremetal/example2-counter/client/main.ts:123:5)
    at async main (/home/pvepn/SolanaBootcamp/examples_baremetal/example2-counter/client/main.ts:54:5) {
  signature: '',
  transactionMessage: 'Transaction simulation failed: Error processing Instruction 0: custom program error: 0x5',
  transactionLogs: [
    'Program 11111111111111111111111111111111 invoke [1]',
    'Create: address A8ReBk7yjpt9BsxnXavWY923rRFrsKUJYxSkiPXoxs9w does not match derived address 34Do48RDmjj1YDxBf5B3QXeXc5ZufoWjhYRWzXD7xVyJ',
    'Program 11111111111111111111111111111111 failed: custom program error: 0x5'
  ]
}
```

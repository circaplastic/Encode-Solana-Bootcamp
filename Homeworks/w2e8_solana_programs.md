# Week 2 Exercise 8: Solana Programs

Using the examples in the [repo](https://github.com/ExtropyIO/SolanaBootcamp/tree/main/examples_baremetal)

1. Modify the existing msg! in example1-helloworld to log the program ID: "program_id.to_string()" was added to line 14 of the [code](https://github.com/circaplastic/SolanaBootcamp/blob/main/examples_baremetal/example1-helloworld/rust/src/lib.rs)

'''bash

~$ solana logs

Streaming transaction logs. Confirmed commitment
Transaction executed in slot 733:

  Signature: 22eoWV28DcqugVEDosHMpuhSJjz7wBwTm2i45Bzczgb9Jumivej8ERvqERK7PFBMqfWEedCQHq3uKyx1cC9YyuyP
  
  Status: Ok
  
  Log Messages:
  
    Program DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5 invoke [1]
    Program log: [lib] Hello World Rust program entrypoint, Program ID is: DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5
    Program DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5 consumed 11851 of 200000 compute units
    Program DLvDA1a3KscdVG5hPsNyMoRocQCoSmjbpDLUgVwdLSP5 success
    

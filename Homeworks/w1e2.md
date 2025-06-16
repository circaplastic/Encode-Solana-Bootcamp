# W1-E2: Solana Ecosystem

## 1. How many validators are currently active?  
As of June 14, 2025, at 5:40 P.M. (UTC -6), the Solana mainnet has 1,129 active validators.  
*Source*: [solana.com/validators](https://solana.com/validators)

## 2. What makes this block unique?  
This is Solana’s genesis block (Slot 0, Epoch 0), created on March 16, 2020, at 14:29 UTC—the very first block in the network’s history.

## 3. What’s special about this address?  
Known as the **Solana Incinerator**, this address is purpose-built to burn SOL tokens, effectively taking them out of circulation forever. Any SOL sent here—like the 11,365,066.99 SOL noted in Question 4—is gone for good, since the address lacks a private key, rendering the tokens irretrievable.

## 4. What’s happening in this transaction?  
This transaction burned 11,365,066.99 SOL by sending it to the Solana Incinerator. That exact figure ties into an article exploring uncertainties around SOL’s total supply.  
*Source*: [blockworks.co/news/what-is-solana-everything-you-need-to-know-about-the-ethereum-rival](https://blockworks.co/news/what-is-solana-everything-you-need-to-know-about-the-ethereum-rival)  
- **From**: `3o6xgkJ9sTmDeQWyfj3sxwon18fXJB9PV5LDc8sfgR4a`  
  *Signature*: `2E7CDMTssxTYkdetCKVWQv9X2KNDPiuZrT2Y7647PhFEXuAWWxmHJb3ryCmP29ocQ1SNc7VyJjjm4X3jE8xWDmGY`  
- **To**: `1nc1nerator11111111111111111111111111111111` (Solana Incinerator—tokens burned)  
- **Fee Payer**: `6yKHERk8rsbmJxvMpPuwPs1ct3hRiP7xaJF2tvnGU6nK`  
  *Signature*: `45pGoC4Rr3fJ1TKrsiRkhHRbdUeX7633XAGVec6XzVdpRbzQgHhe6ZC6Uq164MPWtiqMg7wCkC6Wy3jy2BqsDEKf`  
- **Fee Receiver**: `11111111111111111111111111111111` (System Program)

## 5. What’s the largest possible balance for a Solana account?  
The maximum theoretical balance is **18,446,744,073,709,551,615 lamports**, or roughly **18.446 billion SOL**.  

Solana uses a 64-bit unsigned integer (u64) to track account balances in lamports, capping the max at `(2^64) - 1 = 18,446,744,073,709,551,615 lamports`. That’s equivalent to 18,446,744,073.709551615 SOL. In practice, this is just a theoretical ceiling—way beyond the current total supply. As of June 14, 2025 (5:40 P.M. UTC -6), the numbers look like this:  

```bash
~$ solana supply --url https://api.mainnet-beta.solana.com  
Total Supply: 602,996,252.3778731 SOL  
Circulating Supply: 527,628,422.0916775 SOL  
Non-Circulating Supply: 75,367,830.28619547 SOL  
```

The entire supply—602.996 million SOL—could fit in one account with room to spare. Even if the supply grew massively, a single account could still hold up to 18.446 billion SOL.

## 6. What benefits does Solana offer end users over other blockchains?  
Solana delivers standout advantages for users:  
- **Blazing-fast transactions**: With a throughput of up to 65,000 TPS, Solana confirms transactions in a flash—far outpacing Ethereum’s 15-30 TPS.  
- **Near-zero fees**: Expect to pay less than a penny per transaction (often under $0.01), a game-changer compared to Ethereum’s unpredictable gas costs.  
- **Scalability without compromise**: Thanks to its Proof of History (PoH) and Proof of Stake (PoS) combo, Solana keeps latency low and performance high, making dApps—like DeFi tools or NFT platforms—feel seamless and responsive.

## 7. Install Rust if needed.  
Rust is already installed and ready to go.

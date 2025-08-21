The program should replicate the functionality of EVM Universal NFT with the capabilities to:

send NFT to other connected chains (identified by their ZRC-20 gas tokens) and to ZetaChain
mint incoming NFTs

The Universal NFT program on Solana must implement on_call.

Architect and implement a Solana NFT program that enables cross-chain minting, transfers, and ownership verification via ZetaChain’s protocol-contracts-solana gateway here: https://github.com/zeta-chain/protocol-contracts-solana

document use protocol-contracts-solana gateway: https://www.zetachain.com/docs/developers/chains/solana/

> High level program flow
>
> # Universal NFT Protocol - Solana Contract Documentation
>
> ## Background
>
> Universal NFT is a cross-chain NFT protocol which allows users to freely transfer NFTs across chains connected to Zeta.
>
> - The NFTs retain their metadata and ownership information. Irrespective of the current chain it exists on, the NFT can always be connected to its metadata URL.
> - NFTs can be minted on any connected chain, including zEVM.
> - Token IDs are unique, i.e., no two NFTs can have the same token ID across all chains.
> - Transfers follow a burn and mint mechanism. This means during a transfer, the NFT is burned on the source chain and a new NFT is minted on the destination chain.
>
> The Universal NFT protocol consists of the following components:
>
> - **zEVM NFT contract**: This is the router contract which allows transfer of tokens from one connected chain to another, or from zEVM to a connected chain.
> - **EVM NFT contract**: This is the contract on the connected chain with EVM compatibility. It allows minting and transfer of NFTs.
> - **Solana NFT contract**: This is the contract on the Solana chain which allows minting and transfer of NFTs.
>
> The following document provides details on the Solana NFT contract and how it works with the Universal NFT protocol.
>
> Why select Burn instead of Lock
>
> - Lock would mean we implement some sort of a registry / escrow . To maintain the NFT data . Which is good but adds additional logic and us having to maintain the data
> - The existing universal NFT, on EVM, which is already present, uses lock and mint, so this fits in well with the existing setup.
>
> ## Solana NFT Contract
>
> ### NFTs on Solana
>
> - NFTs are treated as SPL tokens on Solana, with the special property that there is only one copy of the specific token.
> - Every SPL token on Solana is identified using the public key of the mint account. In the case of NFTs, the mint account is the unique identifier for the NFT; there is no need for a separate token ID.
> - Solana provides a standard program for NFT metadata, called the Metaplex program, which along with the URI is also capable of storing rich context about the NFT. The metadata PDA is derived from the mint key.
>
> ### How This Affects the Universal NFT Protocol
>
> - Essentially, on Solana, NFTs cannot be retrieved(minted again) once burned, although the metadata is still available on the metadata account.
> - Every time an NFT is minted on Solana, it is a new token (as we would need to create a new mint account).
> - The main issue that we need to solve is how to link an NFT to its original metadata and mint key.
>
> ### Minting NFTs on Solana
>
> - The options available are
>
>   - Lock and mint , unlock the NFT when trasnferring back to Solana
>   - Link the NFT to the orginal metadata whenever minting
>     This document describes the second option, which is to link the NFT to the original metadata whenever minting.
>
> There are two scenarios for minting NFTs on Solana:
>
> - Mint a new NFT on Solana
> - Transfer from a connected chain to Solana
>
> ## Proposed Solution
>
> ### Mint New NFT on Solana and Transfer
>
> #### Step 1: Mint
>
> - Mint a new NFT using a new mint account.
> - Create the metadata account using the mint account as a seed. (Metaplex standard format )
> - Create the master edition account using the mint account as a seed.
> - Create a `token ID` from [mint pubkey + block.number + _nextTokenId++] (Next token ID can be maintained in the program state).
> - Create a new PDA on the Solana Universal NFT program which can be used to store the origin information:
>
>   - The seed for the PDA is the `token ID` generated above and a constant `nft_origin` (we need to use the bytes version of the token ID, but bytes and numbers are interchangeable).
>   - The PDA stores the original mint key. This was used to create the original metadata account and master edition account.
>   - The `token ID`
>
> #### Step 2: Transfer to a Connected Chain
>
> - When transferring the NFT to a connected chain, we burn the NFT on Solana.
> - The token ID is used in the cross-chain message to identify the NFT. Always fetch the token ID from the `nft_orgin` PDA.
> - The token ID is minted on the connected chain.
>
> ### Transfer from a Connected Chain to Solana
>
> - When transferring the NFT from a connected chain to Solana, we burn the NFT on the connected chain.
> - Use the `token ID` + `nft_origin` to find the origin metadata account:
>
>   - If it exists, we know that the NFT has already been minted on Solana before. We can fetch the original mint key and use that to fetch the original metadata. The token can be minted as a new token, but we would have the link to the original token as well. We still need the new token to create a fresh metadata account which would have the same URI as the original metadata account.
>   - If it does not exist, this means that the NFT was minted on a connected chain and this is the first time it's being sent to Solana. We can create a new mint account and metadata account for the NFT from the data available in the cross-chain message, and then mint a new token as before.
>
>     - The token ID saved to the origin account in this case would be the origin token ID from the connected chain (we need to use the bytes version of the token ID, but bytes and numbers are interchangeable).
>     - The original information would be saved using `token ID` + `nft_origin` as the seed for the PDA. [Note: this token ID is generated on the connected chain]
>
> ### Things to Note
>
> - We assume that the token ID generated by the Universal NFT program is unique across all chains. The metadata link is based on this fact.
> - We are creating a customized PDA `nft_origin`. This is not recognized by all wallets and explorers; however, the necessary NFT information should always be available in the Metaplex metadata account created with the current keypair.

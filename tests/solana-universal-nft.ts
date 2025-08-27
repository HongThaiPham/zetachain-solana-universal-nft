import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SolanaUniversalNft } from '../target/types/solana_universal_nft';
import { expect } from 'chai';
import { keccak256, sha256 } from 'ethereumjs-util';

describe('solana-universal-nft', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .solanaUniversalNft as Program<SolanaUniversalNft>;
  const GATEWAY_PROGRAM_ID = new anchor.web3.PublicKey(
    'ZETAjseVjuFsxdRxo6MmTCvqFwb3ZHUx56Co3vCmGis'
  );
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
  );

  const mintKeypair = new anchor.web3.Keypair();

  const [configAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('config')],
    program.programId
  );

  const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeypair.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const [masterEditionAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeypair.publicKey.toBuffer(),
      Buffer.from('edition'),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  console.table({
    payer: program.provider.wallet.publicKey.toBase58(),
    mint: mintKeypair.publicKey.toBase58(),
    configAddress: configAddress.toBase58(),
    metadataAddress: metadataAddress.toBase58(),
    masterEditionAddress: masterEditionAddress.toBase58(),
  });

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(GATEWAY_PROGRAM_ID)
      .accounts({
        authority: program.provider.wallet.publicKey,
      })
      .rpc();
    console.log('Your transaction signature', tx);
    const configAccount = await program.account.programConfig.fetch(
      configAddress
    );

    expect(configAccount.authority.equals(program.provider.wallet.publicKey)).to
      .be.true;
    expect(configAccount.gatewayProgram.equals(GATEWAY_PROGRAM_ID)).to.be.true;
    expect(configAccount.nextTokenNonce.eq(new anchor.BN(1))).to.be.true;
  });

  it('Creates a new NFT', async () => {
    const configAccount = await program.account.programConfig.fetch(
      configAddress
    );

    const slot = await provider.connection.getSlot();
    console.log('slot:', slot);
    console.log('configAccount:', configAccount.nextTokenNonce);
    const buffer = Buffer.concat([
      mintKeypair.publicKey.toBuffer(),
      new anchor.BN(slot).toArrayLike(Buffer, 'le', 8),
      configAccount.nextTokenNonce.toArrayLike(Buffer, 'le', 8),
    ]);

    const [originNftAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [sha256(buffer), Buffer.from('nft_origin')],
      program.programId
    );
    console.log('originNftAddress:', originNftAddress.toBase58());
    const tx = await program.methods
      .newNft(
        new anchor.BN(slot),
        'Solana Universal NFT',
        'SUN',
        'https://example.com/nft'
      )
      .accounts({
        payer: program.provider.wallet.publicKey,
        mint: mintKeypair.publicKey,
        recipient: program.provider.wallet.publicKey,
        masterEdition: masterEditionAddress,
        metadata: metadataAddress,
        originNft: originNftAddress,
      })
      .signers([mintKeypair])
      .rpc();
    console.log('Your transaction signature', tx);
  });
});

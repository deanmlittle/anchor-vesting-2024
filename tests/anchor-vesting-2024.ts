import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { randomBytes } from "crypto";
import { AnchorVesting2024 } from "../target/types/anchor_vesting_2024";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, MINT_SIZE, TOKEN_2022_PROGRAM_ID, createAssociatedTokenAccountIdempotentInstruction, createInitializeMint2Instruction, createMintToInstruction, createTransferCheckedInstruction, getAssociatedTokenAddressSync, getMinimumBalanceForRentExemptMint } from "@solana/spl-token";
import { assert } from "chai";
describe("anchor-vesting-2024", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVesting2024 as Program<AnchorVesting2024>;

  const provider = anchor.getProvider();

  const connection = provider.connection;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  const seed = new BN(randomBytes(8));
  const NOW = new BN(Math.floor(new Date().getTime() / 1000));
  const LATER = NOW.add(new BN(1000));
  const EVEN_LATER = LATER.add(new BN(1000));

  const admin = Keypair.generate();
  const vester = Keypair.generate();
  const mint = Keypair.generate();
  const config = PublicKey.findProgramAddressSync(
    [
      Buffer.from("config"), 
      admin.publicKey.toBuffer(),
      mint.publicKey.toBuffer(),
      seed.toBuffer("le", 8)
    ],
    program.programId
  )[0];
  const vault = getAssociatedTokenAddressSync(mint.publicKey, config, true, TOKEN_2022_PROGRAM_ID);
  const vesterTa = getAssociatedTokenAddressSync(mint.publicKey, vester.publicKey, false, TOKEN_2022_PROGRAM_ID);
  const adminAta = getAssociatedTokenAddressSync(mint.publicKey, admin.publicKey, false, TOKEN_2022_PROGRAM_ID);
  const vestNow = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vest"), 
      vesterTa.toBuffer(),
      NOW.toBuffer("le", 8)
    ],
    program.programId
  )[0];

  const vestLater = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vest"), 
      vesterTa.toBuffer(),
      LATER.toBuffer('le', 8)
    ],
    program.programId
  )[0];

  const vestEvenLater = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vest"), 
      vesterTa.toBuffer(),
      EVEN_LATER.toBuffer('le', 8)
    ],
    program.programId
  )[0];

  const accounts = {
    admin: admin.publicKey,
    payer: admin.publicKey,
    mint: mint.publicKey,
    config,
    vault,
    vester: vester.publicKey,
    vesterTa,
    adminAta,
    recovery: adminAta,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId
  }
  
  it("Airdrop", async () => {
    let lamports = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new Transaction();
    tx.instructions = [
      ...[admin, vester].map((k) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: k.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
      ),
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mint.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_2022_PROGRAM_ID
      }),
      createInitializeMint2Instruction(
        mint.publicKey,
        6,
        admin.publicKey,
        undefined,
        TOKEN_2022_PROGRAM_ID
      ),
      createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, adminAta, admin.publicKey, mint.publicKey, TOKEN_2022_PROGRAM_ID),
      createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, vesterTa, vester.publicKey, mint.publicKey, TOKEN_2022_PROGRAM_ID),
      createMintToInstruction(mint.publicKey, adminAta, admin.publicKey, 1e11, undefined, TOKEN_2022_PROGRAM_ID),
    ];
    await provider.sendAndConfirm(tx, [admin, mint]).then(log);
  });

  it("Initialize config", async () => {
    const tx = await program.methods
      .initialize(seed)
      .accounts({...accounts})
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Create a matured vest", async () => {
    const tx = await program.methods
      .createVesting(NOW, new BN(1337e6))
      .accounts({...accounts, vest: vestNow})
      .signers([admin])
      .rpc({
        skipPreflight:true
      })
      .then(confirm)
      .then(log);
  });

  it("Create an unmatured vest", async () => {
    const tx = await program.methods
      .createVesting(LATER, new BN(1337e6))
      .accounts({...accounts, vest: vestLater})
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Create another unmatured vest", async () => {
    const tx = await program.methods
      .createVesting(EVEN_LATER, new BN(1337e6))
      .accounts({...accounts, vest: vestEvenLater})
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Fail to claim a vest before finalization", async () => {
    try { 
      const tx = await program.methods
        .claimVesting()
        .accounts({...accounts, vest: vestNow})
        .signers([vester])
        .rpc()
        .then(confirm)
        .then(log);
        throw new Error("Shouldn't have made it to here!")
      } catch(e) {
        assert(e.error?.errorCode?.code === "VestingUnfinalized")
      }
  });

  it("Cancel a vest", async () => {
    const tx = await program.methods
      .cancelVesting()
      .accounts({...accounts, vest: vestLater})
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Deposits vesting tokens", async () => {
    const tx = new Transaction();
    tx.add(createTransferCheckedInstruction(
      adminAta,
      mint.publicKey,
      vault,
      admin.publicKey,
      1339e7,
      6,
      undefined,
      TOKEN_2022_PROGRAM_ID
    ))
    await provider.sendAndConfirm(tx, [admin]).then(log);
  });

  it("Finalizes the vest", async () => {
    const tx = await program.methods
      .finalize()
      .accounts({...accounts})
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Claim a vest after activation", async () => {
    const tx = await program.methods
      .claimVesting()
      .accounts({...accounts, vest: vestNow})
      .signers([vester])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Fail to claim an unmatured vest", async () => {
    try {
      const tx = await program.methods
        .claimVesting()
        .accounts({...accounts, vest: vestEvenLater})
        .signers([vester])
        .rpc()
        .then(confirm)
        .then(log);
    } catch(e) {
      assert(e.error?.errorCode?.code === "NotFullyVested")
    }
  });

  it("Withdraw surplus tokens", async () => {
    const tx = await program.methods
      .withdrawSurplus()
      .accounts({...accounts})
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });
});

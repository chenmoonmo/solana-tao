import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTao } from "../target/types/solana_tao";

describe("solana-tao", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();

  let connection = provider.connection;

  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTao as Program<SolanaTao>;

  let user: anchor.web3.Keypair;
  let systemPDA: anchor.web3.PublicKey;
  let subnet1PDA: anchor.web3.PublicKey;
  let validator1PDA: anchor.web3.PublicKey;
  let miner1PDA: anchor.web3.PublicKey;

  it("Is initialized bittensor!", async () => {
    user = anchor.web3.Keypair.generate();
    // airdrop some SOL to the user
    const sig = await connection.requestAirdrop(
      user.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: sig,
    });

    [systemPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(Buffer.from("system"))],
      program.programId
    );

    await program.methods
      .initializeSystem()
      .accounts({
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        bittensorState: systemPDA,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const state = await program.account.bittensorState.fetch(systemPDA);

    console.log("State: ", state);
  });

  it("Is initlialized subnet", async () => {
    [subnet1PDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_state"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeSubnet()
      .accounts({
        subnetState: subnet1PDA,
        bittensorState: systemPDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const subnet = await program.account.subnetState.fetch(subnet1PDA);
    const bittensor = await program.account.bittensorState.fetch(systemPDA);

    console.log("Subnet state: ", subnet);
    console.log("Bittensor state: ", bittensor);
  });

  it("Is initlialized Validator", async () => {
    [validator1PDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("validator_state"),
        subnet1PDA.toBuffer(),
        user.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeSubnetValidator()
      .accounts({
        validatorState: validator1PDA,
        subnetState: subnet1PDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const validator = await program.account.validatorState.fetch(validator1PDA);
    const subnet = await program.account.subnetState.fetch(subnet1PDA);

    console.log("Validator state: ", validator);
    console.log("Subnet state: ", subnet);
  });
  it("Is initlialized Miner", async () => {
    [miner1PDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("miner_state"),
        subnet1PDA.toBuffer(),
        user.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeSubnetMiner()
      .accounts({
        minerState: miner1PDA,
        subnetState: subnet1PDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const miner = await program.account.minerState.fetch(miner1PDA);
    const subnet = await program.account.subnetState.fetch(subnet1PDA);

    console.log("miner state: ", miner);
    console.log("Subnet state: ", subnet);
  });
});

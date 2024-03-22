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

    // Add your test here.
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
    const [subnetState] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_state")],
      program.programId
    );

    await program.methods
      .initializeSubnet()
      .accounts({
        subnetState,
        bittensorState: systemPDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const subnet = await program.account.subnetState.fetch(subnetState);
    const bittensor = await program.account.bittensorState.fetch(systemPDA);


    console.log("Subnet state: ", subnet);
    console.log("Bittensor state: ", bittensor);
  });
});

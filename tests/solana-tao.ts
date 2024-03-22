import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTao } from "../target/types/solana_tao";

describe("solana-tao", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();

  let connection = provider.connection;

  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTao as Program<SolanaTao>;

  let user = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
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

    const [systemPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(Buffer.from("system"))],
      program.programId
    );

    console.log("System PDA: ", systemPDA.toBase58());
    console.log("User: ", user.publicKey.toBase58());

    // Add your test here.
    await program.methods
      .initializeSystem()
      .accounts({
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        systemState: systemPDA,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      }
    );

    const state = await program.account.systemInfoState.fetch(systemPDA);

    console.log("State: ", state);
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Carsino } from "../target/types/carsino";

describe("Carsino", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.Carsino as Program<Carsino>;

  it("Create the account",async () => {
    const newKeyPair = anchor.web3.Keypair.generate();
    await program.methods.initialize()
      .accounts({
        payer: wallet.publicKey,
        newAccount: newKeyPair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([wallet.payer, newKeyPair])
      .rpc();
  });
});

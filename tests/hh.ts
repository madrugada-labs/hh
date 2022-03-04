import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Hh } from "../target/types/hh";

describe("hh", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Hh as Program<Hh>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

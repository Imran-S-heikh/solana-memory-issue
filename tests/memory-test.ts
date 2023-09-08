import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaMemoryIssue } from "../target/types/solana_memory_issue";

describe("solana-memory-issue", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .SolanaMemoryIssue as Program<SolanaMemoryIssue>;
  const [userInfoAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user-info"), program.provider.publicKey.toBuffer()],
    program.programId
  );

  it("Init User", async () => {
    // await program.provider.connection.requestAirdrop(
    //   program.provider.publicKey!,
    //   anchor.web3.LAMPORTS_PER_SOL * 30
    // );
    // Add your test here.
    const tx = await program.methods
      .initUser(23)
      .accounts({
        signer: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        userInfo: userInfoAddress,
      })
      .rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });

  it("Increase Data", async () => {
    for (let index = 2; index <= 3; index++) {
      const tx = await program.methods
        .increaseSpace(10240 * index)
        .accounts({
          signer: program.provider.publicKey!,
          userInfo: userInfoAddress,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      console.log("Space Increase Tx: ", tx);
    }

    const account = await program.provider.connection.getAccountInfo(
      userInfoAddress
    );
    console.log("Account Size: ", account.data.byteLength);
  });

  it("Add Friend", async () => {
    // Size For Each Friend = 33
    // So in 30KB we should be able to store
    // (30720 - UserInfoSize(45)) / 33 = 929 Friends
    try {
      for (let index = 0; index < 929; index++) {
        const keypair = new anchor.web3.Keypair();
        const tx = await program.methods
          .addFriend(keypair.publicKey, Math.round(Math.random() * 12 + 18))
          .accounts({
            signer: program.provider.publicKey,
            userInfo: userInfoAddress,
          })
          .rpc();
        console.log(index, " Your transaction signature", tx);
      }
    } catch (error) {
      console.log(error);
    }

    const userInfo = await program.account.userInfo.fetch(userInfoAddress);
    console.log("Total Friends: ", userInfo.friends.length);
  });
});

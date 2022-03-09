import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { TicTacToe } from "../target/types/tic_tac_toe";

describe("tic-tac-toe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>;

  it("setup game!", async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const playerOne = program.provider.wallet;
    const playerTwo = anchor.web3.Keypair.generate();
    // Add your test here.
    const tx = await program.rpc.setupGame(playerTwo.publicKey, {
      accounts: {
        game: gameKeypair.publicKey,
        playerOne: playerOne.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [gameKeypair]
    });

    const gameState = await program.account.game.fetch(gameKeypair.publicKey);
    console.log(gameState);
    expect(gameState.turn).to.equal(1);
    expect(gameState.players).to.eql([playerOne.publicKey, playerTwo.publicKey]);
    expect(gameState.state).to.eql({active: {}});
    expect(gameState.board).to.eql(
      [
        [null, null, null],
        [null, null, null],
        [null, null, null]
      ]
      )
    console.log("Your transaction signature", tx);
  });
});

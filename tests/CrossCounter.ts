import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { CrossCounter } from "../target/types/cross_counter";
import { CrossCounter2 } from "../target/types/cross_counter2";
import { MockWormhole } from "../target/types/mock_wormhole";
const fs = require("fs");

const admin = anchor.web3.Keypair.fromSecretKey(
  Uint8Array.from(
    fs
      .readFileSync("FRANKC3ibsaBW1o2qRuu3kspyaV4gHBuUfZ5uq9SXsqa.json", {
        encoding: "utf8",
        flag: "r",
      })
      .slice(1, -1)
      .split(",")
  )
);

describe("CrossCounter", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CrossCounter as Program<CrossCounter>;
  const program2 = anchor.workspace.CrossCounter2 as Program<CrossCounter2>;
  const wormhole = anchor.workspace.MockWormhole as Program<MockWormhole>;

  let counter1; let counter1Bump;
  let counter2; let counter2Bump;
  it("Initialize Mainnet Counter", async () => {

    // Is this the program on mainnet
    let mainnet = true;

    [counter1, counter1Bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("counter"),
      ],
      program.programId
    );
    // Initialize the program
    const tx = await program.methods.initialize(mainnet)
    .accounts({
      counter: counter1,
    })
    .signers([admin])
    .rpc();

    // Check values
    let counterAccount = await program.account.counter.fetch(counter1);
    assert.equal(counterAccount.isActive, true);
    assert.equal(counterAccount.value, 0);
  });

  it("Initialize Shadow Realm Counter", async () => {

    // Is this the program on mainnet
    let mainnet = false;

    [counter2, counter2Bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("counter"),
      ],
      program2.programId
    );
    // Initialize the program
    const tx = await program2.methods.initialize(mainnet)
    .accounts({
      counter: counter2,
    })
    .signers([admin])
    .rpc();

    // Check values
    let counterAccount = await program2.account.counter.fetch(counter2);
    assert.equal(counterAccount.isActive, false);
    assert.equal(counterAccount.value, 0);
  });

  it("Increment Mainnet Counter", async () => {

    // Initialize the program
    const tx = await program.methods.increment()
      .accounts({
        counter: counter1
      })
      .signers([admin])
      .rpc();

    // Check values
    let counterAccount = await program.account.counter.fetch(counter1);
    assert.equal(counterAccount.isActive, true);
    assert.equal(counterAccount.value, 1);
  });


  it("Attempt Incrementing Shadow Realm Counter", async () => {

    // Initialize the program
    console.log("Attempting to increment inactive shadow realm program")
    try {
      let tx = await program2.methods.increment()
        .accounts({
          counter: counter2
        })
        .signers([admin])
        .rpc();
    } catch (error) {
      console.error(error.error);
    }

    // Check values
    let counterAccount = await program2.account.counter.fetch(counter2);
    assert.equal(counterAccount.isActive, false);
    assert.equal(counterAccount.value, 0);
  });

  it("Post Message to Wormhole from Mainnet", async () => {

    let [message, messageBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("fake-wormhole"),
          program.programId.toBytes(),
        ],
        wormhole.programId
      );


    // post message
    try {
      let tx = await program.methods.freezeAndSend()
        .accounts({
          mockWormholeProgram: wormhole.programId,
          messageData: message,
        })
        .signers([admin])
        .rpc()
    } catch (error) {
      console.error(error);
    }

    // Check values
    let messageAccount = await wormhole.account.messageData.fetch(message);
    assert(Buffer.from(messageAccount.payload).equals(
      (Buffer.from([0x1,0x0]))));
  });
  
  it("Read Message From Shadow Realm", async () => {

    let [message, messageBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("fake-wormhole"),
          program.programId.toBytes(),
        ],
        wormhole.programId
      );


    // read message
    // try {
      let tx = await program2.methods.updateAndActivate()
        .accounts({
          mockWormholeProgram: wormhole.programId,
          messageData: message,
        })
        .signers([admin])
        .rpc()
    // } catch (error) {
    //   console.error(error);
    // }

    // Check active program, value
    let counterAccount2 = await program2.account.counter.fetch(counter2);
    assert.equal(counterAccount2.isActive, true);
    assert.equal(counterAccount2.value, 1)

    // Check inactive program, value
    let counterAccount1 = await program.account.counter.fetch(counter1);
    assert.equal(counterAccount1.isActive, false);
  });

  it("Attempt Increment Mainnet Counter", async () => {

    // Initialize the program
    console.log("Attempting to increment inactive mainnet program")
    try {
    const tx = await program.methods.increment()
      .accounts({
        counter: counter1
      })
      .signers([admin])
      .rpc();
    } catch (error) {
      console.error(error.error)
    }

     // Check value didn't change
     let counterAccount1 = await program.account.counter.fetch(counter1);
     assert.equal(counterAccount1.value, 1);

  });


  it("Incrementing Shadow Realm Counter", async () => {

    // Initialize the program
      let tx = await program2.methods.increment()
        .accounts({
          counter: counter2
        })
        .signers([admin])
        .rpc();

    // Check values
    let counterAccount = await program2.account.counter.fetch(counter2);
    assert.equal(counterAccount.isActive, true);
    assert.equal(counterAccount.value, 2);
  });

})

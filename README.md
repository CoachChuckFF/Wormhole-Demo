# CrossCounter: A Cross-Chain Counter Program

## Context
I can see three modes for a cross-chain program. There are possibly more but these are the obvious ones.

1) One which is concurrent with local independent states. Imagine a cross-chain jupiter that checks all USDC-USDT routes on all chains, incl shadow realms. This doesn't requires any synchronization of state because every pool involved is independently doing their own accounting.

2) One which is concurrent with global state. Imagine a single USDC-USDT non-oracle-based swap that has fragmented liquidity across all chains so as to facilitate giving/taking user funds. USDC amount = sum(fragmented pools). USDT amount = sum(fragmented pools). The price is determined by total amount. Like, a single ORCA pool that you can interact with either on mainnet OR a shadow realm. This one would require every interaction (swap) to synchronize state across all chains before the next can go. 

3) One which is intending to live on a shadow realm solo for a while, or even periodically swap between. Such a program can have a boolean that allows ix's to run if on mainnet if true, and on a shadow realm if false. When the switch goes on/off, we send a big message through Wormhole which updates the other program's state entirely. Any tokens that were produced or traded as a result of the program just use a token bridge.

We provide a proof of concept for 3). If a Shadow Realm is to be a Solana Fork, then we can mock cross-chain interactions by mocking Wormhole and pretend two programs that live on a localnet live on two separate chains, communicating only through the Wormhole. Here, we deploy two independent programs to a localnet that communicate solely through the mock Wormhole and show how one can migrate programs on the protocol level without any changes required to core solana consensus or validator mechanisms.

## Code
This repo provides an anchor workspace with three anchor programs:

1. `CrossCounter`
2. `CrossCounter2`
3. `MockWormhole`

`CrossCounter` and `CrossCounter2` are identical, except for the names and the declared program id. They contain the following anchor instructions:

![cross_counter](https://user-images.githubusercontent.com/93507302/167511538-a289f816-8afb-43e9-90fb-4d05861af5f3.png)

1. `initialize`: initializes a counter PDA containing 
    ```rust
    #[account]
    pub struct Counter {
        pub value: u16,
        pub is_active: bool,
        pub last_update: i64,
    }
    ```
    with `value` at zero. The mainnet program is initialized with `is_active` set to true, while the shadow realm version is initialized with `is_active` set to false. The `last_update` is to track the last time this PDA was updated by `update_and_activate`.

2. `increment`: increments the counter `value` by 1. Fails if `is_active` is false, as should all program instructions not related to the migration demonstrated here.

3. `freeze_and_send`: executes a cpi to the mock Wormhole program, which creates a PDA containing the state of `counter`.

4. `update_and_activate`: reads the message data, updating the state of `counter` to that contained in the Wormhole message.



`MockWormhole` contains the following anchor instructions


![mock_wormhole](https://user-images.githubusercontent.com/93507302/167511553-151d4163-fa89-423b-af03-f81b0b724cc6.png)


1. `post_message`: posts a message containing a PDA's state in the `payload`.
2. `read_message`: not currently used, but was intended to demo a cpi which returns aforementioned `payload`.


## Test
The mocha tests are self explanatory. When running `anchor test` the three programs are deployed on a localnet, and the usage and migration of a program is tested:
```

CrossCounter
    ✔ Initialize Mainnet Counter (461ms)
    ✔ Initialize Shadow Realm Counter (402ms)
    ✔ Increment Mainnet Counter (403ms)
Attempting to increment inactive shadow realm program
{
  errorCode: { code: 'InactiveProgram', number: 6000 },
  errorMessage: 'This program is currently inactive',
  comparedValues: undefined,
  origin: {
    file: 'programs/CrossCounter2/src/instructions/increment.rs',
    line: 11
  }
}
    ✔ Attempt Incrementing Shadow Realm Counter
    ✔ Post Message to Wormhole from Mainnet (383ms)
    ✔ Read Message From Shadow Realm (405ms)
Attempting to increment inactive mainnet program
{
  errorCode: { code: 'InactiveProgram', number: 6000 },
  errorMessage: 'This program is currently inactive',
  comparedValues: undefined,
  origin: {
    file: 'programs/CrossCounter/src/instructions/increment.rs',
    line: 11
  }
}
    ✔ Attempt Increment Mainnet Counter
    ✔ Incrementing Shadow Realm Counter (386ms)


  8 passing (2s)

Done in 8.13s.
```

In other words, the timeline of the test is:

1 and 2) Mock Mainnet and Shadow Realm counters are initialized with proper `is_active` flags.

3 and 4) Mainnet counter can be (and is) incremented from 0 to 1, while Shadow Realm counter cannot (and is not) incremented.

5 and 6) Mainnet state is sent, and program is frozen. Shadow Realm state is updated, and program is activated.

7 and 8) Mainnet counter cannot (and is not) incremented, while Shadow Realm counter can be (and is) incremented from 1 to 2.


# Next Steps
Next order of business here is to create a way to freeze and migrate programs with multiple PDAs and handle token account updates.


For the former:
1. Isolate freeze program instruction
2. freeze, then use concensus mechanism to count number of PDAs --> N
4. Post N messages to wormhole containing state of N PDAs
5. Other program reads N messages, unfreezing only after N unique updates are made

For the latter: similar thing, but we have one token PDA per SPL or SOL on The Other Side™️ that the program sends funds to and then that PDA reads some other M number of messages and routes the proper number of tokens to the proper token PDAs (total of N+M unique program updates before activating program)
# Wormhole-Demo

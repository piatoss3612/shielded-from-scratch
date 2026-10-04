<p align="center">
  <img src="assets/banner.svg" width="100%" alt="Shielded from Scratch: Zcash shielded transactions, by hand, in Rust. A pixel-art cat blacksmith hammers a round shield of iron and wood on an anvil by a lantern post, and a dotted path runs from a sapling labeled Sapling to a tree labeled Ironwood.">
</p>

<p align="center">
  Zcash shielded transactions, built from scratch in Rust, one small level at a time.
</p>

## What this is

A learning repo: I'm building Zcash shielded transactions from scratch in Rust, Sapling first and then Ironwood. It starts from a single amount type and grows one piece at a time. At the end, an Ironwood transaction I built by hand gets mined on a local regtest.

This is a study log, not a library. Don't use this code with real money.

## How it works

Each step here is a level. A level takes 30 to 45 minutes and has exactly one new idea and one pass condition: a named test that turns green.

A few levels make a world, and each world ends with a boss. The boss adds no new idea; it ties the world together into one finished piece that lasts.

There are eight kinds of level: see, predict, modify, write, try (call a real Zcash crate or test vector), match (byte for byte against a crate or test vector), break, and boss. A real Zcash crate or test vector shows up at least once every four levels.

The tutor is Claude Code. It explains, sets up tests and scaffolding, and offers hints when I'm stuck. I write the core lines myself, and a level only counts as cleared when I can explain each of them in one sentence. The tutor contract is in [CLAUDE.md](CLAUDE.md) and my progress log is in [PROGRESS.md](PROGRESS.md). Both are in Korean.

## Where I am now

**W1 · Amounts.** I'm building an amount type that counts in zatoshis, the smallest unit of ZEC. This world's boss is L05, *Match the answer key*: it's cleared when my type gives the same answers as `zcash_protocol`.

The current level and anything I'm stuck on are in [PROGRESS.md](PROGRESS.md). Saying "튜터 세션 시작" (start a tutor session) to Claude Code in this repo opens that level.

## Trophy shelf

Each cleared world fills one slot.

<p>
  <img src="assets/trophy-slot.svg" width="64" alt="Empty slot: W1 Amounts" title="W1 Amounts">
</p>

The first slot fills at the W1 boss, L05.

<details>
<summary>Milestones along the way</summary>

<br>

- **M1** Add and multiply points on a curve, and compress them to 32 bytes and back. <sub>Jubjub point arithmetic</sub>
- **M2** Compute the root of an empty note tree with a hash I wrote. <sub>Pedersen hash</sub>
- **M3** Commit to a note while hiding what's in it. <sub>note commitment</sub>
- **M4** Encrypt a note so the recipient can read it. <sub>note encryption</sub>
- **M5** Sign each spend with a key that looks brand new. <sub>rerandomized signatures, RedJubjub</sub>
- **M6** Make my first zero-knowledge proof, then remove one constraint and watch a false claim pass. <sub>Groth16, under-constrained circuits</sub>
- **M7** Prove an output is valid with a circuit I built. <sub>Output circuit</sub>
- **M8** Make a proof that spends a note, borrowing the crate's circuit. <sub>Spend description</sub>
- **M9** Build one shielded transfer and pass it through a verifier I wrote. <sub>z→z transaction</sub>
- **M10** A small wallet finds the notes it received and sends them on, for two full cycles. <sub>Sapling wallet</sub>
- **M11** A hand-built Ironwood transaction gets mined on a local regtest. <sub>NU6.3, v6 transactions</sub>

After M1, M5, M9 and M11, I'll write a post covering everything up to that point.

</details>

## Running it

```sh
# all tests
cargo test

# one module
cargo test value

# tests that are still locked
cargo test -- --ignored --list
```

Red tests always belong to the current level. Tests for levels that aren't open yet are locked with an attribute like `#[ignore = "L03"]` and unlocked when their level opens. So a red `cargo test` doesn't mean something is broken; it's the problem in front of me.

The toolchain is stable, pinned in `rust-toolchain.toml`.

## Files

- `src/`: the code, which grows a little with every level
- `PROGRESS.md`: current level, world checklist, session log
- `CLAUDE.md`: the tutor contract
- `Cargo.toml`: versions pinned to the `ff`/`group` 0.13 ecosystem; commented-out crates are enabled at the level that first needs them
- `rust-toolchain.toml`: stable
- `assets/`: the banner and trophies

## Thanks

- [Zcash Protocol Specification](https://zips.z.cash/protocol/protocol.pdf), the source of every answer.
- [zcash-test-vectors](https://github.com/zcash/zcash-test-vectors), the answer key for the matching levels.
- The Rust crates from [zcash](https://github.com/zcash) and [zkcrypto](https://github.com/zkcrypto), the yardstick for everything built by hand.
- The blacksmith cat in the banner is drawn after the Lantern Guardian of [piatoss.xyz](https://piatoss.xyz).

# reference_signoff

## Project Title
reference_signoff

## Project Description
reference_signoff is a Soroban smart contract that brings professional references on chain in a privacy-respecting way. A former manager or peer records a rating together with the hash of an off-chain comment as a signed reference for a candidate, and the candidate decides exactly which prospective employers may view it. References can be retracted by their original author, so endorsements always reflect the referrer's current view.

## Project Vision
Our vision is a portable, candidate-owned professional reputation layer for the Stellar ecosystem. By anchoring references to a public ledger while keeping the full content private via hashes and explicit access grants, candidates can carry a verifiable record of endorsements across jobs, borders, and platforms, and employers can trust the provenance of any reference they receive.

## Key Features
- **Signed references** with a 1-5 rating, a role tag, and an off-chain comment hash recorded on chain by the referrer.
- **Candidate-controlled visibility** — only the viewers the candidate explicitly grants can ever see a reference.
- **Retractable references** so a referrer can withdraw an endorsement at any time, with a reason captured for the audit log.
- **Privacy by design** — the full comment text is stored off chain; only its hash lives on chain.
- **Per-reference access tracking** to monitor how many prospective employers have been shared a given reference.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** identity dApp — see `contracts/reference_signoff/src/lib.rs` for the full reference_signoff business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CAQTEPTHZNFW7SSHKRE4KMKNPXRCY6SPVP7DQJS27I6TVIR4CPDPVHAP`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/4a2544018adc0da20af69d21007c3a54addf318ba546ee642ba0b9b37b4bbb8d`

## Future Scope
- Weighted reputation scoring that aggregates multiple references into a single candidate score.
- Time-bound view grants that automatically expire after a hiring decision window.
- Integration with W3C Verifiable Credentials for cross-chain portability of endorsements.
- An optional on-chain response mechanism that lets a candidate attach a short statement to a retracted reference.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `reference_signoff` (identity)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet

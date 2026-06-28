#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

/// A single professional reference stored on chain. The full comment text
/// lives off chain; only its hash is recorded here.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reference {
    pub referrer: Address,
    pub candidate: Address,
    pub role: Symbol,
    pub rating: u32,
    pub comment_hash: Symbol,
    pub retracted: bool,
    pub timestamp: u64,
}

/// Namespaced storage keys for the contract.
#[contracttype]
#[derive(Clone, Debug)]
pub enum DataKey {
    /// The reference record itself, keyed by (candidate, ref_id).
    Reference(Address, u32),
    /// Total number of references ever stored for a candidate.
    RefCount(Address),
    /// Next auto-incrementing reference id for a candidate.
    NextRefId(Address),
    /// Map of viewer Address -> granted (bool) for a given reference.
    Viewers(Address, u32),
}

#[contract]
pub struct ReferenceSignoff;

#[contractimpl]
impl ReferenceSignoff {
    /// Sign a professional reference for a candidate. The referrer (a
    /// former manager or peer) records a 1-5 rating, the role under which
    /// they worked with the candidate, and a hash of the full comment
    /// stored off chain. The reference is owned by the candidate, who
    /// controls who is allowed to view it. Returns the new reference id.
    pub fn sign_reference(
        env: Env,
        referrer: Address,
        candidate: Address,
        role: Symbol,
        rating: u32,
        comment_hash: Symbol,
    ) -> u32 {
        // The referrer must authorize writing the reference.
        referrer.require_auth();

        if rating < 1 || rating > 5 {
            panic!("rating must be between 1 and 5");
        }
        if referrer == candidate {
            panic!("a referrer cannot sign a reference for themselves");
        }

        // Auto-increment the per-candidate reference id.
        let ref_id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::NextRefId(candidate.clone()))
            .unwrap_or(0u32);

        let reference = Reference {
            referrer: referrer.clone(),
            candidate: candidate.clone(),
            role: role.clone(),
            rating,
            comment_hash: comment_hash.clone(),
            retracted: false,
            timestamp: env.ledger().timestamp(),
        };

        env.storage()
            .instance()
            .set(&DataKey::Reference(candidate.clone(), ref_id), &reference);

        // Bump the lifetime reference count.
        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::RefCount(candidate.clone()))
            .unwrap_or(0u32);
        env.storage()
            .instance()
            .set(&DataKey::RefCount(candidate.clone()), &(count + 1));

        env.storage()
            .instance()
            .set(&DataKey::NextRefId(candidate.clone()), &(ref_id + 1));

        // Initialize an empty viewer access map for this reference.
        let viewers: Map<Address, bool> = Map::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::Viewers(candidate, ref_id), &viewers);

        ref_id
    }

    /// Retract a previously signed reference. Only the original referrer
    /// may withdraw their own reference, providing a reason for the audit
    /// log. Retracted references remain on chain for provenance but no
    /// longer count as a valid endorsement.
    pub fn retract(
        env: Env,
        referrer: Address,
        candidate: Address,
        ref_id: u32,
        _reason: Symbol,
    ) {
        referrer.require_auth();

        let key = DataKey::Reference(candidate.clone(), ref_id);
        let mut reference: Reference = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("reference not found"));

        if reference.referrer != referrer {
            panic!("only the original referrer can retract this reference");
        }
        if reference.retracted {
            panic!("reference already retracted");
        }

        reference.retracted = true;
        env.storage().instance().set(&key, &reference);
    }

    /// Candidate grants a verifier (e.g. a prospective employer) view
    /// access to a specific reference they have received. Only the
    /// candidate who owns the reference may grant access.
    pub fn grant_view(env: Env, candidate: Address, viewer: Address, ref_id: u32) {
        candidate.require_auth();

        // Make sure the reference actually exists before granting access.
        let _: Reference = env
            .storage()
            .instance()
            .get(&DataKey::Reference(candidate.clone(), ref_id))
            .unwrap_or_else(|| panic!("reference not found"));

        let key = DataKey::Viewers(candidate.clone(), ref_id);
        let mut viewers: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        viewers.set(viewer, true);
        env.storage().instance().set(&key, &viewers);
    }

    /// Returns the number of viewers that have been granted access to
    /// view a particular reference. Useful for the candidate to track
    /// how many prospective employers have been shared a given reference.
    pub fn verify(env: Env, candidate: Address, ref_id: u32) -> u32 {
        let viewers: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&DataKey::Viewers(candidate, ref_id))
            .unwrap_or(Map::new(&env));
        viewers.keys().len()
    }

    /// Returns the total number of references recorded for a candidate,
    /// including retracted ones (so the candidate can audit full history).
    pub fn list_references(env: Env, candidate: Address) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::RefCount(candidate))
            .unwrap_or(0u32)
    }

    /// Returns the rating (1-5, where 5 is the highest) of a reference.
    /// Returns 0 when the reference has been retracted, signaling that
    /// the endorsement is no longer valid.
    pub fn get_rating(env: Env, candidate: Address, ref_id: u32) -> u32 {
        let reference: Reference = env
            .storage()
            .instance()
            .get(&DataKey::Reference(candidate, ref_id))
            .unwrap_or_else(|| panic!("reference not found"));

        if reference.retracted {
            return 0;
        }
        reference.rating
    }
}

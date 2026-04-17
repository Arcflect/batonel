# Partner Preset Review Operations

This document defines the process for Batonel governance reviewers to validate and merge partner preset submissions.

## Objective

To maintain the integrity of Batonel's trusted ecosystem, all partner public keys added to `.github/trust/allowed_signers` must be carefully vetted. This procedure ensures origin authenticity while mitigating risks associated with supply chain attacks.

## 1. Submission Review

When a new partner requests inclusion via the **Partner Preset Submission** issue template:

1. **Verify Formatting:** Ensure the provided key is exclusively an `ssh-ed25519` key. Our CI validation scripts (`validate_allowed_signers.sh`) will automatically enforce this natively.
2. **Review Target Identity:** The email or identifier must logically align with the organization name and the preset they are publishing.

## 2. Out-of-band Verification

You MUST NOT merge a request solely because the PR checks pass.

1. Take the contact method provided in the **Out-of-band Verification Contact** field.
2. Validate the identity of the submitter by reaching out to them via a trusted, independent communication channel (e.g., organizational email signed by PGP, a shared trust channel).
3. Confirm exactly the fingerprint of the Ed25519 key they submitted.

## 3. Merge Procedure

Once verified:
1. Ensure the key is added to `.github/trust/allowed_signers` as follows:
   `target_identity ssh-ed25519 PUBLIC_KEY_STRING`
2. Label the corresponding PR with `trust-anchor-update`.
3. An assigned `governance-admin` (or equivalent RBAC role) MUST approve the PR.
4. Merge the pull request.

## 4. Revocation

If a partner's private key is compromised, or they are no longer maintaining their preset:
1. Do not delete the key entirely. Instead, prepend the string `revoked ` to the key's entry in `.github/trust/allowed_signers`.
   Example: `revoked target_identity ssh-ed25519 PUBLIC_KEY_STRING`
2. This ensures that any CI validations attempting to verify older signed bundles will explicitly reject it as revoked, maintaining the immutable history of trust rejection.

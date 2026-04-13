#!/usr/bin/env bash
set -eou pipefail

# verify_trust.sh
# Verifies the origin authenticity of an Archflow asset using SSH Ed25519 signatures.
# 
# Usage:
#   ./scripts/verify_trust.sh <identity> <signature-file> <target-file> [allowed-signers-file]
#
# Example:
#   ./scripts/verify_trust.sh governance@arcflect.com generic-layered.tar.gz.sig generic-layered.tar.gz

IDENTITY=${1:-}
SIGNATURE_FILE=${2:-}
TARGET_FILE=${3:-}
ALLOWED_SIGNERS_FILE=${4:-".github/trust/allowed_signers"}

if [[ -z "$IDENTITY" || -z "$SIGNATURE_FILE" || -z "$TARGET_FILE" ]]; then
    echo "Error: Missing required arguments."
    echo "Usage: $0 <identity> <signature-file> <target-file> [allowed-signers-file]"
    exit 1
fi

if [[ ! -f "$ALLOWED_SIGNERS_FILE" ]]; then
    echo "Error: Allowed signers file not found at $ALLOWED_SIGNERS_FILE"
    exit 1
fi

if [[ ! -f "$SIGNATURE_FILE" ]]; then
    echo "Error: Signature file not found at $SIGNATURE_FILE"
    exit 1
fi

if [[ ! -f "$TARGET_FILE" ]]; then
    echo "Error: Target file not found at $TARGET_FILE"
    exit 1
fi

echo "Verifying trust for $TARGET_FILE using identity $IDENTITY..."

# Perform verification
if ssh-keygen -Y verify -f "$ALLOWED_SIGNERS_FILE" -I "$IDENTITY" -n file -s "$SIGNATURE_FILE" < "$TARGET_FILE"; then
    echo "[SUCCESS] Verification passed: Origin authenticity confirmed."
    exit 0
else
    echo "[ERROR] Verification failed: Invalid signature, identity mismatch, or revoked key."
    exit 1
fi

#!/usr/bin/env bash
set -eou pipefail

# test_verify_trust.sh
# Validates the governance operations for trust verification.

echo "Running Trust Verification Tests..."

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

# Generate an ephemeral core identity
ssh-keygen -t ed25519 -N "" -C "governance@arcflect.io" -f "$TMP_DIR/core_key" -q
CORE_PUB=$(cat "$TMP_DIR/core_key.pub")

# Generate an ephemeral partner identity
ssh-keygen -t ed25519 -N "" -C "partner@example.com" -f "$TMP_DIR/partner_key" -q
PARTNER_PUB=$(cat "$TMP_DIR/partner_key.pub")

# Create mock allowed_signers
ALLOWED_SIGNERS="$TMP_DIR/allowed_signers"
echo "governance@arcflect.io $CORE_PUB" > "$ALLOWED_SIGNERS"
echo "partner@example.com $PARTNER_PUB" >> "$ALLOWED_SIGNERS"

# Create a mock preset bundle
MOCK_PRESET="$TMP_DIR/preset.tar.gz"
echo "dummy preset contents" > "$MOCK_PRESET"

# 1. Sign with valid core key
ssh-keygen -Y sign -f "$TMP_DIR/core_key" -n file "$MOCK_PRESET" >/dev/null 2>&1
MOCK_SIG="$MOCK_PRESET.sig"

echo "TEST 1: Validating legitimate signature..."
if ./scripts/verify_trust.sh governance@arcflect.io "$MOCK_SIG" "$MOCK_PRESET" "$ALLOWED_SIGNERS" >/dev/null; then
    echo "  [PASS] Legitimate signature verified."
else
    echo "  [FAIL] Legitimate signature failed verification."
    exit 1
fi

# 2. Try validating the core signature with the partner identity
echo "TEST 2: Validating wrong identity bounds..."
if ./scripts/verify_trust.sh partner@example.com "$MOCK_SIG" "$MOCK_PRESET" "$ALLOWED_SIGNERS" >/dev/null 2>&1; then
    echo "  [FAIL] Signature wrongly accepted under different identity."
    exit 1
else
    echo "  [PASS] Signature rejected for wrong identity."
fi

# 3. Modify the bundle (tamper)
echo "tampered" >> "$MOCK_PRESET"

echo "TEST 3: Validating tampered asset..."
if ./scripts/verify_trust.sh governance@arcflect.io "$MOCK_SIG" "$MOCK_PRESET" "$ALLOWED_SIGNERS" >/dev/null 2>&1; then
    echo "  [FAIL] Tampered asset was accepted."
    exit 1
else
    echo "  [PASS] Tampered asset rejected."
fi
echo "dummy preset contents" > "$MOCK_PRESET" # restore

# 4. Revocation Test
echo "revoked governance@arcflect.io $CORE_PUB" > "$ALLOWED_SIGNERS"

echo "TEST 4: Validating revoked key..."
if ./scripts/verify_trust.sh governance@arcflect.io "$MOCK_SIG" "$MOCK_PRESET" "$ALLOWED_SIGNERS" >/dev/null 2>&1; then
    echo "  [FAIL] Revoked key signature was accepted."
    exit 1
else
    echo "  [PASS] Revoked key signature rejected."
fi

echo ""
echo "All governance assurance tests passed successfully."
exit 0

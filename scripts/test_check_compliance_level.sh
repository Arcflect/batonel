#!/usr/bin/env bash
set -eou pipefail

# test_check_compliance_level.sh
# Tests the ecosystem compliance maturity level checker across fixture scenarios.
#
# Scenarios:
#   1. Full L4 compliance  — all controls in place
#   2. Partial L2 only     — allowed_signers + verify_trust.sh, no review docs
#   3. L1 only             — allowed_signers valid, no signing pipeline
#   4. L0 (empty repo)     — no governance files

echo "Running check_compliance_level.sh tests..."
echo ""

CHECKER="./scripts/check_compliance_level.sh"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

# ─────────────────────────────────────────────────────────────
# Fixture helpers
# ─────────────────────────────────────────────────────────────

setup_base_dirs() {
    local root="$1"
    mkdir -p "$root/.github/trust"
    mkdir -p "$root/.github/workflows"
    mkdir -p "$root/docs"
    mkdir -p "$root/scripts"
}

write_valid_allowed_signers() {
    local root="$1"
    # Generate a fresh ed25519 key for this fixture (unique path per call)
    local key_file
    key_file=$(mktemp "$TMP_DIR/fixture_key_XXXXXX")
    rm -f "$key_file"
    ssh-keygen -t ed25519 -N "" -f "$key_file" -q
    local pub
    pub=$(cat "$key_file.pub")
    # Format: identity keytype key [comment]
    local keytype keypart rest
    read -r keytype keypart rest <<< "$pub"
    echo "governance@arcflect.io $keytype $keypart" > "$root/.github/trust/allowed_signers"
}

write_validate_script() {
    local root="$1"
    cp ./scripts/validate_allowed_signers.sh "$root/scripts/validate_allowed_signers.sh"
    chmod +x "$root/scripts/validate_allowed_signers.sh"
}

write_verify_script() {
    local root="$1"
    cp ./scripts/verify_trust.sh "$root/scripts/verify_trust.sh"
    chmod +x "$root/scripts/verify_trust.sh"
    # Write a minimal CI workflow referencing verify_trust.sh
    cat > "$root/.github/workflows/trust.yml" <<'EOF'
name: Trust
on: [push]
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - run: bash scripts/verify_trust.sh
EOF
}

write_review_docs() {
    local root="$1"
    # Minimal partner review doc with out-of-band mention
    cat > "$root/docs/partner-preset-review.md" <<'EOF'
# Partner Preset Review Operations
## 2. Out-of-band Verification
You MUST perform out-of-band verification before merging.
EOF
    # Minimal RBAC doc
    cat > "$root/docs/governance-rbac.md" <<'EOF'
# Governance RBAC
Roles: policy_admin, architect, auditor.
EOF
}

write_compliance_script() {
    local root="$1"
    cp ./scripts/check_compliance_level.sh "$root/scripts/check_compliance_level.sh"
    chmod +x "$root/scripts/check_compliance_level.sh"
    cp ./scripts/test_check_compliance_level.sh "$root/scripts/test_check_compliance_level.sh"
    # Add CI workflow referencing check_compliance_level.sh
    cat > "$root/.github/workflows/compliance.yml" <<'EOF'
name: Compliance
on: [push]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - run: bash scripts/check_compliance_level.sh
EOF
}

# ─────────────────────────────────────────────────────────────
# Test 1: Full L4 compliance
# ─────────────────────────────────────────────────────────────
echo "TEST 1: Full L4 compliance (expect exit 0)"
REPO1="$TMP_DIR/repo_l4"
setup_base_dirs "$REPO1"
write_valid_allowed_signers "$REPO1"
write_validate_script "$REPO1"
write_verify_script "$REPO1"
write_review_docs "$REPO1"
write_compliance_script "$REPO1"

if bash "$CHECKER" "$REPO1" > /dev/null 2>&1; then
    echo "  [PASS] L4 repository correctly assessed as L4."
else
    echo "  [FAIL] L4 repository was not assessed as L4."
    exit 1
fi

# ─────────────────────────────────────────────────────────────
# Test 2: L2 only — no review docs, no compliance script
# ─────────────────────────────────────────────────────────────
echo "TEST 2: L2-only repository (expect exit 1, highest level L2)"
REPO2="$TMP_DIR/repo_l2"
setup_base_dirs "$REPO2"
write_valid_allowed_signers "$REPO2"
write_validate_script "$REPO2"
write_verify_script "$REPO2"
# No review docs, no compliance script

if ! bash "$CHECKER" "$REPO2" > /dev/null 2>&1; then
    echo "  [PASS] L2 repository correctly reported as non-L4 (exit 1)."
else
    echo "  [FAIL] L2 repository was incorrectly reported as L4."
    exit 1
fi

# ─────────────────────────────────────────────────────────────
# Test 3: L1 only — valid allowed_signers, no signing pipeline
# ─────────────────────────────────────────────────────────────
echo "TEST 3: L1-only repository (expect exit 1)"
REPO3="$TMP_DIR/repo_l1"
setup_base_dirs "$REPO3"
write_valid_allowed_signers "$REPO3"
write_validate_script "$REPO3"
# No verify_trust.sh, no CI workflow, no review docs

if ! bash "$CHECKER" "$REPO3" > /dev/null 2>&1; then
    echo "  [PASS] L1 repository correctly reported as non-L4 (exit 1)."
else
    echo "  [FAIL] L1 repository was incorrectly reported as L4."
    exit 1
fi

# ─────────────────────────────────────────────────────────────
# Test 4: L0 — empty repo, no governance files
# ─────────────────────────────────────────────────────────────
echo "TEST 4: L0 (empty) repository (expect exit 1)"
REPO4="$TMP_DIR/repo_l0"
setup_base_dirs "$REPO4"
# No files written

if ! bash "$CHECKER" "$REPO4" > /dev/null 2>&1; then
    echo "  [PASS] L0 repository correctly reported as non-L4 (exit 1)."
else
    echo "  [FAIL] L0 repository was incorrectly reported as L4."
    exit 1
fi

# ─────────────────────────────────────────────────────────────
# Test 5: This repository itself must pass at least L4
# ─────────────────────────────────────────────────────────────
echo "TEST 5: This repository self-assessment (expect L4, exit 0)"
if bash "$CHECKER" "." > /dev/null 2>&1; then
    echo "  [PASS] This repository passes its own L4 compliance check."
else
    echo "  [FAIL] This repository does not pass its own L4 compliance check."
    bash "$CHECKER" "." || true
    exit 1
fi

echo ""
echo "All check_compliance_level.sh tests passed successfully."
exit 0

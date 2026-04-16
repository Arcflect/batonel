#!/usr/bin/env bash
set -eou pipefail

# check_compliance_level.sh
# Evaluates the Archflow ecosystem compliance maturity level of the current repository.
#
# Usage:
#   ./scripts/check_compliance_level.sh [repo_root]
#
# Arguments:
#   repo_root  Optional. Path to the repository root (default: current directory).
#
# Exit codes:
#   0  All levels pass (L4 — Audit Continuous)
#   1  One or more levels failed; output identifies the failing criteria
#
# Each level is cumulative. Failing a lower level automatically fails higher levels.

REPO_ROOT="${1:-.}"

PASS=0
FAIL=1

# Track the highest achieved level
HIGHEST_LEVEL=-1
FAILURE_MESSAGES=()

_pass() {
    echo "  [PASS] $1"
}

_fail() {
    echo "  [FAIL] $1"
    FAILURE_MESSAGES+=("$1")
}

_header() {
    echo ""
    echo "$1"
}

# ─────────────────────────────────────────────────────────────
# L1 — Trust Anchored
# ─────────────────────────────────────────────────────────────
check_l1() {
    _header "L1 — Trust Anchored"
    local ok=true

    local allowed_signers="$REPO_ROOT/.github/trust/allowed_signers"

    if [[ ! -f "$allowed_signers" ]]; then
        _fail ".github/trust/allowed_signers is missing"
        ok=false
    else
        _pass ".github/trust/allowed_signers exists"

        # Check file is non-empty (ignoring comments and blank lines)
        local active_count
        active_count=$(grep -v '^\s*$' "$allowed_signers" | grep -v '^\s*#' | wc -l | tr -d ' ')
        if [[ "$active_count" -eq 0 ]]; then
            _fail "allowed_signers contains no active key entries"
            ok=false
        else
            _pass "allowed_signers has $active_count active key entry/entries"
        fi

        # Check all keys are ssh-ed25519 and no duplicates
        if [[ -f "$REPO_ROOT/scripts/validate_allowed_signers.sh" ]]; then
            if bash "$REPO_ROOT/scripts/validate_allowed_signers.sh" "$allowed_signers" > /dev/null 2>&1; then
                _pass "validate_allowed_signers.sh passes against allowed_signers"
            else
                _fail "validate_allowed_signers.sh reports an error against allowed_signers"
                ok=false
            fi
        else
            _fail "scripts/validate_allowed_signers.sh is missing (required for L1 check)"
            ok=false
        fi
    fi

    $ok && return $PASS || return $FAIL
}

# ─────────────────────────────────────────────────────────────
# L2 — Signing Operational
# ─────────────────────────────────────────────────────────────
check_l2() {
    _header "L2 — Signing Operational"
    local ok=true

    local verify_script="$REPO_ROOT/scripts/verify_trust.sh"

    if [[ ! -f "$verify_script" ]]; then
        _fail "scripts/verify_trust.sh is missing"
        ok=false
    elif [[ ! -x "$verify_script" ]]; then
        _fail "scripts/verify_trust.sh exists but is not executable"
        ok=false
    else
        _pass "scripts/verify_trust.sh exists and is executable"
    fi

    # Check at least one CI workflow references verify_trust.sh
    local ci_dir="$REPO_ROOT/.github/workflows"
    if [[ -d "$ci_dir" ]]; then
        if grep -rl "verify_trust.sh" "$ci_dir" > /dev/null 2>&1; then
            _pass "A CI workflow references verify_trust.sh"
        else
            _fail "No CI workflow references verify_trust.sh"
            ok=false
        fi
    else
        _fail ".github/workflows directory is missing"
        ok=false
    fi

    $ok && return $PASS || return $FAIL
}

# ─────────────────────────────────────────────────────────────
# L3 — Review Controlled
# ─────────────────────────────────────────────────────────────
check_l3() {
    _header "L3 — Review Controlled"
    local ok=true

    # Partner review workflow document
    local review_doc="$REPO_ROOT/docs/partner-preset-review.md"
    if [[ ! -f "$review_doc" ]]; then
        _fail "docs/partner-preset-review.md is missing"
        ok=false
    else
        _pass "docs/partner-preset-review.md exists"

        # Check for out-of-band verification requirement
        if grep -qi "out-of-band" "$review_doc"; then
            _pass "partner-preset-review.md documents out-of-band verification"
        else
            _fail "partner-preset-review.md does not document out-of-band verification"
            ok=false
        fi
    fi

    # Governance RBAC document
    local rbac_doc="$REPO_ROOT/docs/governance-rbac.md"
    if [[ ! -f "$rbac_doc" ]]; then
        _fail "docs/governance-rbac.md is missing"
        ok=false
    else
        _pass "docs/governance-rbac.md exists"
    fi

    # Revocation procedure — check validate_allowed_signers.sh handles 'revoked' marker
    local validate_script="$REPO_ROOT/scripts/validate_allowed_signers.sh"
    if [[ -f "$validate_script" ]]; then
        if grep -q "revoked" "$validate_script"; then
            _pass "validate_allowed_signers.sh supports the 'revoked' marker"
        else
            _fail "validate_allowed_signers.sh does not appear to handle the 'revoked' marker"
            ok=false
        fi
    else
        _fail "scripts/validate_allowed_signers.sh is missing (required for L3 revocation check)"
        ok=false
    fi

    $ok && return $PASS || return $FAIL
}

# ─────────────────────────────────────────────────────────────
# L4 — Audit Continuous
# ─────────────────────────────────────────────────────────────
check_l4() {
    _header "L4 — Audit Continuous"
    local ok=true

    # This script itself must exist
    local this_script="$REPO_ROOT/scripts/check_compliance_level.sh"
    if [[ ! -f "$this_script" ]]; then
        _fail "scripts/check_compliance_level.sh is missing"
        ok=false
    else
        _pass "scripts/check_compliance_level.sh exists"
    fi

    # Test script must exist
    local test_script="$REPO_ROOT/scripts/test_check_compliance_level.sh"
    if [[ ! -f "$test_script" ]]; then
        _fail "scripts/test_check_compliance_level.sh is missing"
        ok=false
    else
        _pass "scripts/test_check_compliance_level.sh exists"
    fi

    # A CI workflow must reference check_compliance_level.sh
    local ci_dir="$REPO_ROOT/.github/workflows"
    if [[ -d "$ci_dir" ]]; then
        if grep -rl "check_compliance_level.sh" "$ci_dir" > /dev/null 2>&1; then
            _pass "A CI workflow references check_compliance_level.sh"
        else
            _fail "No CI workflow references check_compliance_level.sh"
            ok=false
        fi
    else
        _fail ".github/workflows directory is missing"
        ok=false
    fi

    $ok && return $PASS || return $FAIL
}

# ─────────────────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────────────────
echo "Checking Archflow ecosystem compliance maturity..."
echo "Repository root: $REPO_ROOT"

# L0 is always met (it is the absence of controls)
_header "L0 — Unregistered"
_pass "Baseline satisfied (L0 is the starting state)"
HIGHEST_LEVEL=0

if check_l1; then
    HIGHEST_LEVEL=1
    if check_l2; then
        HIGHEST_LEVEL=2
        if check_l3; then
            HIGHEST_LEVEL=3
            if check_l4; then
                HIGHEST_LEVEL=4
            fi
        fi
    fi
fi

echo ""
echo "─────────────────────────────────────────────"

LEVEL_NAMES=("L0 — Unregistered" "L1 — Trust Anchored" "L2 — Signing Operational" "L3 — Review Controlled" "L4 — Audit Continuous")

echo "Result: ${LEVEL_NAMES[$HIGHEST_LEVEL]}"

if [[ ${#FAILURE_MESSAGES[@]} -gt 0 ]]; then
    echo ""
    echo "Criteria not met:"
    for msg in "${FAILURE_MESSAGES[@]}"; do
        echo "  - $msg"
    done
fi

echo ""

if [[ "$HIGHEST_LEVEL" -eq 4 ]]; then
    exit 0
else
    exit 1
fi

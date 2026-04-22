#!/usr/bin/env bash
set -e

echo "============================================================"
echo " Batonel Parity Verification"
echo "============================================================"

# List of all presets that must have a documented parity with an example
PRESETS=("minimal" "generic-layered" "rust-clean-hexagonal")

# List of YAML configuration defaults that must be completely identical
SHARED_FILES=(
  "project.baton.yaml"
  "placement.rules.yaml"
  "contracts.template.yaml"
  "artifacts.plan.yaml"
  "guard.sidecar.yaml"
  "policy.profile.yaml"
)

PARITY_ERRORS=0

echo ""
echo "1. Verifying Example-to-Preset Parity"
echo "------------------------------------------------------------"

for PRESET in "${PRESETS[@]}"; do
  for FILE in "${SHARED_FILES[@]}"; do
    PRESET_TARGET="presets/$PRESET/$FILE"
    EXAMPLE_TARGET="examples/$PRESET/batonel/$FILE"

    # Only check if both files exist (e.g. some examples might omit optional files)
    if [ -f "$PRESET_TARGET" ] && [ -f "$EXAMPLE_TARGET" ]; then
      
      # Use cmp to verify byte-for-byte correctness
      if ! cmp -s "$PRESET_TARGET" "$EXAMPLE_TARGET"; then
        echo "[ERROR] Parity mismatch: $PRESET_TARGET and $EXAMPLE_TARGET differ."
        
        # Display the diff to help contributors
        diff -u "$PRESET_TARGET" "$EXAMPLE_TARGET" || true
        
        PARITY_ERRORS=$((PARITY_ERRORS + 1))
      else
        echo "[OK] $PRESET/$FILE parity matches."
      fi
    fi
  done
done

echo ""
echo "2. Verifying Output Parity (Prompts Sync)"
echo "------------------------------------------------------------"

# Re-run the prompt sync script
python3 scripts/sync_example_prompts.py

# Check if regenerating the prompts causes any git diffs on the expected outputs
if ! git diff --exit-code examples/**/expected/.batonel/prompts/ > /dev/null; then
  echo "[ERROR] Expected outputs are out of sync."
  echo "        Run './scripts/sync_example_prompts.py' and commit the generated changes."
  PARITY_ERRORS=$((PARITY_ERRORS + 1))
else
  echo "[OK] All checked-in expected prompts are fully synchronized."
fi

echo ""
echo "3. Verifying Schema Declarations"
echo "------------------------------------------------------------"

SCHEMA_ERRORS=0
for PRESET in "${PRESETS[@]}"; do
  PRESET_FILE="presets/$PRESET/project.baton.yaml"
  EXAMPLE_FILE="examples/$PRESET/batonel/project.baton.yaml"
  
  if [ -f "$PRESET_FILE" ]; then
    if ! grep -q 'schema_version: "1"' "$PRESET_FILE"; then
      echo "[ERROR] Invalid schema_version inside $PRESET_FILE"
      SCHEMA_ERRORS=$((SCHEMA_ERRORS + 1))
    fi
  fi
  
  if [ -f "$EXAMPLE_FILE" ]; then
    if ! grep -q 'schema_version: "1"' "$EXAMPLE_FILE"; then
      echo "[ERROR] Invalid schema_version inside $EXAMPLE_FILE"
      SCHEMA_ERRORS=$((SCHEMA_ERRORS + 1))
    fi
  fi
done

if [ "$SCHEMA_ERRORS" -eq 0 ]; then
  echo "[OK] All root configs use supported schema_version: \"1\""
else
  PARITY_ERRORS=$((PARITY_ERRORS + SCHEMA_ERRORS))
fi

echo ""
echo "============================================================"
if [ "$PARITY_ERRORS" -gt 0 ]; then
  echo "[FAILED] Verification found $PARITY_ERRORS parity violation(s)."
  exit 1
else
  echo "[SUCCESS] All Parity rules correctly followed."
  exit 0
fi

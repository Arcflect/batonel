#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

PRESET=""
PROJECT_NAME=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --preset)
      PRESET="$2"
      shift 2
      ;;
    --project-name)
      PROJECT_NAME="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

if [[ -z "$PROJECT_NAME" ]]; then
  echo "--project-name is required" >&2
  exit 1
fi

CASE_ID="${PRESET:-default}"
LOG_DIR="$ROOT_DIR/target/onboarding-e2e/$CASE_ID"
mkdir -p "$LOG_DIR"

TMP_WORKDIR="$(mktemp -d)"
trap 'rm -rf "$TMP_WORKDIR"' EXIT

INIT_ARGS=(--project-name "$PROJECT_NAME")
if [[ -n "$PRESET" ]]; then
  INIT_ARGS=(--preset "$PRESET" --project-name "$PROJECT_NAME")
fi

echo "[e2e] case=$CASE_ID"
echo "[e2e] workspace=$TMP_WORKDIR"

pushd "$TMP_WORKDIR" >/dev/null

# 1) Dry-run should not create files.
cargo run --quiet --manifest-path "$ROOT_DIR/Cargo.toml" -- init "${INIT_ARGS[@]}" --dry-run | tee "$LOG_DIR/init-dry-run.log"
if find "$TMP_WORKDIR" -mindepth 1 -maxdepth 1 | grep -q .; then
  echo "dry-run must not write files" >&2
  exit 1
fi

# 2) Real init should generate root config files.
cargo run --quiet --manifest-path "$ROOT_DIR/Cargo.toml" -- init "${INIT_ARGS[@]}" | tee "$LOG_DIR/init.log"

required_files=(
  "project.arch.yaml"
  "placement.rules.yaml"
  "contracts.template.yaml"
  "artifacts.plan.yaml"
)
for file in "${required_files[@]}"; do
  if [[ ! -f "$file" ]]; then
    echo "missing expected file: $file" >&2
    exit 1
  fi
done

if ! grep -Eq '^\s*schema_version:\s*['"'"']?1['"'"']?\s*$' project.arch.yaml; then
  echo "project.arch.yaml must contain archflow.schema_version=1" >&2
  exit 1
fi

if [[ -n "$PRESET" ]]; then
  if ! grep -Eq "^\s*id:\s*['\"]?$PRESET['\"]?\s*$" project.arch.yaml; then
    echo "project.arch.yaml must contain archflow.preset.id=$PRESET" >&2
    exit 1
  fi
fi

# 3) plan output should be deterministic for identical inputs.
PLAN_1="$LOG_DIR/plan-1.log"
PLAN_2="$LOG_DIR/plan-2.log"
cargo run --quiet --manifest-path "$ROOT_DIR/Cargo.toml" -- plan | tee "$PLAN_1"
cargo run --quiet --manifest-path "$ROOT_DIR/Cargo.toml" -- plan | tee "$PLAN_2"

if ! cmp -s "$PLAN_1" "$PLAN_2"; then
  echo "plan output changed between identical runs" >&2
  diff -u "$PLAN_1" "$PLAN_2" || true
  exit 1
fi

popd >/dev/null

echo "[e2e] passed case=$CASE_ID"

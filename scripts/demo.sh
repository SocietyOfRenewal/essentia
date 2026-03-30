#!/usr/bin/env bash
set -euo pipefail

NODE="${NODE:-http://127.0.0.1:7001}"
CLI="cargo run -q -p essentia-cli --"
KEYS="examples/bootstrap/keys"

ALICE="did:essentia:47b8d1e1a7fba6a704a084fda774a3e1d5247d4ce252e0c860307ee9ec29dbd4"
ATTESTOR1="did:essentia:70033d757583938297c0e03c7406374bf5bb705ad839b215c2947c00bb263faf"
ATTESTOR2="did:essentia:bbd3918a7a5e44a404034f81d7247cc78685e7e695de2ec2ecdd90e9e10376ed"
ATTESTOR3="did:essentia:e16c6ded267a5d39d872c6ee6603dd528bfdf5302d737e4e03effe9a6d22e91f"
VENDOR="did:essentia:025c6ae2773d8da687fa43c7ab00918fed8f64ae8e19dd3f12342416bc109ee5"

ADMIN_KEY="$KEYS/validator1.json"

register() {
  local key="$1"
  $CLI tx register-did --node "$NODE" --key "$key"
}

issue_personhood() {
  local did="$1"
  local nullifier="$2"
  $CLI tx issue-personhood \
    --node "$NODE" \
    --key "$ADMIN_KEY" \
    --did "$did" \
    --nullifier "$nullifier" \
    --eligible true
}

issue_role() {
  local did="$1"
  local role="$2"
  $CLI tx issue-role --node "$NODE" --key "$ADMIN_KEY" --did "$did" --role "$role"
}

echo "== registering sample actors =="
register "$KEYS/alice.json"
register "$KEYS/attestor1.json"
register "$KEYS/attestor2.json"
register "$KEYS/attestor3.json"
register "$KEYS/vendor.json"

sleep 6

echo "== issuing personhood and roles =="
issue_personhood "$ALICE" "alice-nullifier-v1"
issue_personhood "$ATTESTOR1" "attestor1-nullifier-v1"
issue_personhood "$ATTESTOR2" "attestor2-nullifier-v1"
issue_personhood "$ATTESTOR3" "attestor3-nullifier-v1"
issue_personhood "$VENDOR" "vendor-nullifier-v1"
issue_role "$ATTESTOR1" attestor
issue_role "$ATTESTOR2" attestor
issue_role "$ATTESTOR3" attestor
issue_role "$VENDOR" vendor
$CLI tx transfer-essent --node "$NODE" --key "$ADMIN_KEY" --to "$ALICE" --amount 5000 --memo "bootstrap bond"

sleep 6

echo "== creating purpose and quest =="
$CLI tx create-purpose \
  --node "$NODE" \
  --key "$ADMIN_KEY" \
  --purpose-id community-garden \
  --epoch 1 \
  --name "Community Garden" \
  --description-hash "bafycommunitygardenpurpose" \
  --essent-budget 100000

$CLI tx create-quest \
  --node "$NODE" \
  --key "$ADMIN_KEY" \
  --quest-id community-garden-q1 \
  --purpose-id community-garden \
  --title "Seed and irrigation rollout" \
  --reward-ceiling 50000 \
  --challenge-window-blocks 1 \
  --audit-tail-bps 2000 \
  --audit-tail-blocks 3 \
  --risk-band low \
  --rubric impact=5000 \
  --rubric quality=3000 \
  --rubric documentation=2000

sleep 6

echo "== submitting claim and reviews =="
$CLI tx submit-claim \
  --node "$NODE" \
  --key "$KEYS/alice.json" \
  --claim-id claim-garden-001 \
  --quest-id community-garden-q1 \
  --evidence-root "bafyclaimgarden001" \
  --metadata-hash "bafyclaimgarden001meta" \
  --bond 1000

sleep 6

$CLI tx submit-review \
  --node "$NODE" \
  --key "$KEYS/attestor1.json" \
  --claim-id claim-garden-001 \
  --summary-hash "review-1" \
  --score impact=9000 \
  --score quality=8500 \
  --score documentation=9500

$CLI tx submit-review \
  --node "$NODE" \
  --key "$KEYS/attestor2.json" \
  --claim-id claim-garden-001 \
  --summary-hash "review-2" \
  --score impact=8800 \
  --score quality=8300 \
  --score documentation=9100

$CLI tx submit-review \
  --node "$NODE" \
  --key "$KEYS/attestor3.json" \
  --claim-id claim-garden-001 \
  --summary-hash "review-3" \
  --score impact=9200 \
  --score quality=8700 \
  --score documentation=9300

sleep 8

echo "== finalizing claim =="
$CLI tx finalize-claim --node "$NODE" --key "$ADMIN_KEY" --claim-id claim-garden-001

sleep 6

echo "== issuing and spending essential units =="
$CLI tx issue-freedom-floor \
  --node "$NODE" \
  --key "$ADMIN_KEY" \
  --to "$ALICE" \
  --epoch 1 \
  --amount 2500 \
  --expires-at 250

sleep 6

$CLI tx spend-units \
  --node "$NODE" \
  --key "$KEYS/alice.json" \
  --vendor "$VENDOR" \
  --amount 600 \
  --memo groceries

sleep 6

$CLI tx redeem-vendor \
  --node "$NODE" \
  --key "$KEYS/vendor.json" \
  --epoch 1 \
  --amount 600

sleep 6

echo "== final account state =="
$CLI query account --node "$NODE" --did "$ALICE"
$CLI query account --node "$NODE" --did "$VENDOR"
$CLI query state --node "$NODE"

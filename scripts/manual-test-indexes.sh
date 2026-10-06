#!/usr/bin/env bash
# Manually exercise @@index @unique / @fulltext / @vector (HNSW).
#
# Offline (default): validate + generate SurrealQL and print the DEFINE INDEX lines.
# Live (--live): docker compose up SurrealDB 3.3, DEFINE ANALYZER, db push, INFO, pull.
#
# Usage:
#   ./scripts/manual-test-indexes.sh
#   ./scripts/manual-test-indexes.sh --live
#   ./scripts/manual-test-indexes.sh --live --keep
#
# Prerequisites for --live: Docker + docker compose (see docker-compose.yml).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

LIVE=0
KEEP=0
PORT="${SURREAL_PORT:-8000}"
NS="${SURREAL_NS:-test}"
DB="${SURREAL_DB:-indexes}"
USER="${SURREAL_USER:-root}"
PASS="${SURREAL_PASS:-root}"
IMAGE="surrealdb/surrealdb:v3.3.0"
COMPOSE_STARTED=0

for arg in "$@"; do
  case "$arg" in
    --live) LIVE=1 ;;
    --keep) KEEP=1 ;;
    -h|--help)
      sed -n '2,14p' "$0"
      exit 0
      ;;
    *)
      echo "unknown arg: $arg (try --help)" >&2
      exit 1
      ;;
  esac
done

WORKDIR="$(mktemp -d "${TMPDIR:-/tmp}/awesome-index-XXXXXX")"
SCHEMA="$WORKDIR/awesome.schema"
CLI=(cargo run -q -p cli -- --schema "$SCHEMA")

compose() {
  docker compose -f "$ROOT/docker-compose.yml" "$@"
}

surreal_sql() {
  # Run SQL against the compose service from a one-shot client on the host network.
  docker run --rm -i --network host "$IMAGE" sql \
    --endpoint "http://127.0.0.1:${PORT}" \
    --user "$USER" --pass "$PASS" \
    --ns "$NS" --db "$DB" \
    "$@"
}

cleanup() {
  if [[ "$KEEP" -eq 0 ]]; then
    rm -rf "$WORKDIR"
    if [[ "$COMPOSE_STARTED" -eq 1 ]]; then
      compose stop surrealdb >/dev/null 2>&1 || true
    fi
  else
    echo "kept workdir: $WORKDIR"
    [[ "$LIVE" -eq 1 ]] && echo "kept compose service surrealdb (port $PORT)"
  fi
}
trap cleanup EXIT

cat >"$SCHEMA" <<EOF
datasource db {
  provider = "surrealdb"
  url      = "ws://127.0.0.1:${PORT}"
  namespace = "${NS}"
  database  = "${DB}"
  username  = "${USER}"
  password  = "${PASS}"
}

naming {
  tables = "snake_case"
}

model Doc {
  id        @id
  email     string
  title     string
  embedding float[]

  @@index([email]) @unique
  @@index([title]) @fulltext("english")
  @@index([embedding]) @vector(3) @dist(Cosine)
  @@table(schemafull)
}
EOF

echo "==> schema written to $SCHEMA"
echo "==> validate"
"${CLI[@]}" validate

echo "==> generate SurrealQL (index lines)"
SURQL="$WORKDIR/schema.surql"
"${CLI[@]}" generate --target schema >"$SURQL"
grep -E 'DEFINE INDEX|DEFINE FIELD|DEFINE TABLE|DEFINE ANALYZER' "$SURQL" || true
echo "---- full SurrealQL also at $SURQL ----"
if ! grep -q 'UNIQUE' "$SURQL"; then
  echo "FAIL: expected UNIQUE index" >&2
  exit 1
fi
if ! grep -q 'FULLTEXT ANALYZER english BM25' "$SURQL"; then
  echo "FAIL: expected FULLTEXT ANALYZER english BM25" >&2
  exit 1
fi
if ! grep -q 'HNSW DIMENSION 3 DIST COSINE' "$SURQL"; then
  echo "FAIL: expected HNSW DIMENSION 3 DIST COSINE" >&2
  exit 1
fi
echo "OK: offline generate looks right"

if [[ "$LIVE" -eq 0 ]]; then
  echo
  echo "Offline pass done. Re-run with --live (uses docker compose) against ${IMAGE}."
  exit 0
fi

if ! command -v docker >/dev/null 2>&1; then
  echo "Docker is required for --live" >&2
  exit 1
fi
if ! docker compose version >/dev/null 2>&1; then
  echo "docker compose is required for --live" >&2
  exit 1
fi

echo "==> docker compose up -d surrealdb (:${PORT})"
compose up -d surrealdb
COMPOSE_STARTED=1

echo "==> wait for healthy"
for _ in $(seq 1 60); do
  status="$(compose ps --status running --format '{{.Health}}' surrealdb 2>/dev/null || true)"
  if [[ "$status" == "healthy" ]] \
    || curl -sf "http://127.0.0.1:${PORT}/health" >/dev/null 2>&1; then
    break
  fi
  sleep 0.5
done

echo "==> DEFINE ANALYZER english (required; DSL does not emit it)"
surreal_sql <<'SQL'
DEFINE ANALYZER english TOKENIZERS blank,class FILTERS lowercase,snowball(english);
SQL

echo "==> db push"
"${CLI[@]}" db push

echo "==> INFO FOR TABLE doc (indexes)"
surreal_sql --pretty <<'SQL'
INFO FOR TABLE doc;
SQL

echo "==> db pull --force (round-trip DSL)"
PULL="$WORKDIR/pulled.schema"
cp "$SCHEMA" "$PULL"
cargo run -q -p cli -- --schema "$PULL" db pull --force
echo "---- pulled indexes ----"
grep -E '@@index' "$PULL" || true

echo
echo "Live pass done."
echo "  schema:  $SCHEMA"
echo "  pulled:  $PULL"
echo "  surql:   $SURQL"
[[ "$KEEP" -eq 1 ]] || echo "(temp files removed; compose service stopped; volume kept — use --keep to leave DB running)"

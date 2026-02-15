#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROTO_ROOT="$ROOT_DIR/api/protos"
BUILD_RS="$ROOT_DIR/crates/rusktop-web/build.rs"
PROTO_MOD="$ROOT_DIR/crates/rusktop-web/src/proto/mod.rs"
OPENAPI_GEN="$ROOT_DIR/api/buf.openapi.gen.yaml"

if [[ ! -f "$BUILD_RS" || ! -f "$PROTO_MOD" || ! -d "$PROTO_ROOT" ]]; then
  echo "[ERR] Expected project files are missing."
  exit 1
fi

mapfile -t PROTO_FILES < <(find "$PROTO_ROOT" -type f -name "*.proto" | sort)

if [[ ${#PROTO_FILES[@]} -eq 0 ]]; then
  echo "[ERR] No proto files found under $PROTO_ROOT"
  exit 1
fi

extract_package() {
  local file="$1"
  grep -E '^[[:space:]]*package[[:space:]]+[^;[:space:]]+[[:space:]]*;' "$file" | sed -E 's/^[[:space:]]*package[[:space:]]+([^;[:space:]]+)[[:space:]]*;.*/\1/' | head -n1
}

unique_sorted() {
  sort -u
}

mapfile -t SERVICE_PACKAGES < <(
  for f in "${PROTO_FILES[@]}"; do
    if grep -qE '^[[:space:]]*service[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' "$f"; then
      extract_package "$f"
    fi
  done | unique_sorted
)

mapfile -t HTTP_PACKAGES < <(
  for f in "${PROTO_FILES[@]}"; do
    if grep -q "google.api.http" "$f"; then
      extract_package "$f"
    fi
  done | unique_sorted
)

mapfile -t INCLUDE_PACKAGES < <(
  grep -oE 'include_proto!\("[^"]+"\)' "$PROTO_MOD" | sed -E 's/.*"([^"]+)".*/\1/' | unique_sorted
)

mapfile -t REST_PACKAGES < <(
  grep -oE '\.package\("[^"]+",' "$BUILD_RS" | sed -E 's/\.package\("([^"]+)",/\1/' | unique_sorted
)

mapfile -t SERDE_PROTO_LIST < <(
  grep -oE '"\.\.\/\.\.\/api\/protos\/[^"]+\.proto"' "$BUILD_RS" | tr -d '"' | unique_sorted
)

AUTO_SCAN_MODE=0
if grep -q "collect_proto_files" "$BUILD_RS"; then
  AUTO_SCAN_MODE=1
fi

STATUS=0

echo "Proto workflow check"
echo "- proto files: ${#PROTO_FILES[@]}"
echo "- service packages: ${#SERVICE_PACKAGES[@]}"
echo "- http packages: ${#HTTP_PACKAGES[@]}"
echo ""

for pkg in "${SERVICE_PACKAGES[@]}"; do
  if ! printf '%s\n' "${INCLUDE_PACKAGES[@]}" | grep -qx "$pkg"; then
    echo "[ERR] Missing include_proto! for package: $pkg ($PROTO_MOD)"
    STATUS=1
  fi
done

for pkg in "${HTTP_PACKAGES[@]}"; do
  if ! printf '%s\n' "${REST_PACKAGES[@]}" | grep -qx "$pkg"; then
    echo "[ERR] Missing RestCodegenConfig::package mapping for HTTP package: $pkg ($BUILD_RS)"
    STATUS=1
  fi
done

if [[ $AUTO_SCAN_MODE -eq 0 && ${#SERDE_PROTO_LIST[@]} -eq 0 ]]; then
  echo "[WARN] PROTO_FILES list appears empty; verify ProstSerdeConfig scope expectations."
fi

if [[ $AUTO_SCAN_MODE -eq 0 ]]; then
  for rel in "${SERDE_PROTO_LIST[@]}"; do
    abs="$ROOT_DIR/crates/rusktop-web/$rel"
    if [[ ! -f "$abs" ]]; then
      echo "[ERR] PROTO_FILES entry does not exist: $rel ($BUILD_RS)"
      STATUS=1
    fi
  done
else
  if ! grep -q "ends_with(\"_doc.proto\")" "$BUILD_RS"; then
    echo "[WARN] Auto-scan mode enabled but *_doc.proto exclusion not found in build.rs"
  fi
fi

if [[ -f "$OPENAPI_GEN" ]]; then
  mapfile -t OPENAPI_PATHS < <(
    sed -n 's/^[[:space:]]*-[[:space:]]*protos\/\(.*\)$/\1/p' "$OPENAPI_GEN" | sed 's:/*$::' | unique_sorted
  )

  for pkg in "${HTTP_PACKAGES[@]}"; do
    dir="${pkg//./\/}"
    covered=0
    if [[ ${#OPENAPI_PATHS[@]} -eq 0 ]]; then
      covered=1
    else
      for p in "${OPENAPI_PATHS[@]}"; do
        if [[ "$dir" == "$p" || "$dir" == "$p"/* ]]; then
          covered=1
          break
        fi
      done
    fi
    if [[ $covered -eq 0 ]]; then
      echo "[ERR] OpenAPI input paths do not cover HTTP package path: protos/$dir ($OPENAPI_GEN)"
      STATUS=1
    fi
  done
fi

if [[ $STATUS -eq 0 ]]; then
  echo "[OK] Proto workflow checks passed."
  echo "[OK] If you added a new package, now run: make openapi && cargo build -p rusktop-web"
else
  echo ""
  echo "Proto workflow checks failed."
  echo "Suggested flow for a new package:"
  echo "  1) add .proto under api/protos/..."
  echo "  2) update crates/rusktop-web/src/proto/mod.rs include_proto!"
  echo "  3) update crates/rusktop-web/build.rs RestCodegenConfig::package(...)"
  echo "  4) ensure api/buf.openapi.gen.yaml paths cover new HTTP package"
  echo "  5) run make openapi && cargo build -p rusktop-web"
fi

exit $STATUS

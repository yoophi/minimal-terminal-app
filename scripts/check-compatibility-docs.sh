#!/usr/bin/env bash
set -euo pipefail

failures=0

fail() {
  printf 'compatibility docs check failed: %s\n' "$1" >&2
  failures=$((failures + 1))
}

matrix="docs/compatibility/matrix.md"
known_gaps="docs/compatibility/known-gaps.md"
compatibility_test="crates/terminal-core/tests/compatibility.rs"

for required in "$matrix" "$known_gaps" "$compatibility_test"; do
  if [[ ! -f "$required" ]]; then
    fail "missing required file: $required"
  fi
done

if [[ -f "$matrix" ]]; then
  while IFS= read -r message; do
    fail "$message"
  done < <(
    awk -F'|' '
      function trim(value) {
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
        return value
      }

      /^\|/ && $0 !~ /^\|[[:space:]-|]+$/ {
        for (i = 1; i <= NF; i++) {
          field = trim($i)
          if (field == "supported") {
            evidence = trim($(i + 1))
            if (evidence == "" || evidence == "-") {
              print "supported row is missing evidence: " $0
            }
          }
          if (field == "unknown" || field == "partially supported" || field == "not supported") {
            if ($0 !~ /known-gaps\.md/) {
              print field " row is not linked to known-gaps.md: " $0
            }
          }
        }
      }
    ' "$matrix"
  )

  while IFS= read -r evidence; do
    test_name="${evidence##*::}"
    if ! rg -q "fn ${test_name}\\(" "$compatibility_test"; then
      fail "matrix references missing compatibility test: $evidence"
    fi
  done < <(rg -o 'tests/compatibility\.rs::[A-Za-z0-9_]+' "$matrix" | sort -u)
fi

while IFS= read -r file; do
  while IFS= read -r match; do
    line="${match%%:*}"
    link="${match#*:}"
    target="${link##*(}"
    target="${target%)}"
    target="${target%%#*}"

    [[ -z "$target" ]] && continue
    [[ "$target" == http://* || "$target" == https://* || "$target" == mailto:* ]] && continue

    resolved="$(cd "$(dirname "$file")" && pwd)/$target"
    if [[ ! -e "$resolved" ]]; then
      fail "$file:$line links to missing file: $target"
    fi
  done < <(rg -n -o '\[[^]]+\]\([^)]+\)' "$file" || true)
done < <(find docs/compatibility -maxdepth 1 -name '*.md' | sort)

if (( failures > 0 )); then
  exit 1
fi

printf 'compatibility docs check passed\n'

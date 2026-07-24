#!/usr/bin/env bash
cd /Users/wandl/workspaces/workspace-github/hutool-rust
MAX=${1:-200}
COUNT=0
COMMITTED=0
SKIP_FILE=/tmp/skip_split_files.txt
touch "$SKIP_FILE"

get_violations() {
    find crates -name '*.rs' -not -path '*/target/*' -not -path '*/tests/*' \
        -not -name 'mod.rs' -not -name 'lib.rs' \
        -exec sh -c 'c=$(grep -cE "^pub (struct|enum|trait|type) " "$1" 2>/dev/null || echo 0); [ "$c" -gt 1 ] && echo "$1"' _ {} \; 2>/dev/null | grep -v "^_:" | sort
}

while [ $COUNT -lt $MAX ]; do
    FILE=""
    v=$(get_violations)
    while IFS= read -r f; do
        [ -z "$f" ] && continue
        if ! grep -qFx "$f" "$SKIP_FILE"; then
            FILE="$f"
            break
        fi
    done <<< "$v"
    if [ -z "$FILE" ]; then
        echo "=== No more violations ==="
        break
    fi
    COUNT=$((COUNT + 1))
    echo "=== [$COUNT] Processing: $FILE ==="
    if grep -q "^macro_rules!" "$FILE" 2>/dev/null; then
        echo "  SKIP (macro)"
        echo "$FILE" >> "$SKIP_FILE"
        continue
    fi
    python3 scripts/split_rs.py "$FILE" > /tmp/split_out.log 2>&1
    if grep -qE "^Error|^Traceback" /tmp/split_out.log; then
        echo "  SCRIPT FAILED, skip"
        echo "$FILE" >> "$SKIP_FILE"
        git checkout -- . > /dev/null 2>&1 || true
        git clean -fd > /dev/null 2>&1 || true
        continue
    fi
    CRATE=$(echo "$FILE" | sed 's|crates/\([^/]*\)/.*|\1|')
    if cargo build -p "$CRATE" > /tmp/build_out.log 2>&1; then
        git add -A
        git commit -m "refactor($CRATE): split $(basename $FILE .rs) into 1:1 structure" --no-gpg-sign > /dev/null 2>&1
        COMMITTED=$((COMMITTED + 1))
        echo "  OK committed ($COMMITTED)"
    else
        echo "  BUILD FAILED, skip"
        echo "$FILE" >> "$SKIP_FILE"
        git checkout -- . > /dev/null 2>&1 || true
        git clean -fd > /dev/null 2>&1 || true
    fi
done
echo "Done. Committed: $COMMITTED, Skipped: $(wc -l < $SKIP_FILE)"

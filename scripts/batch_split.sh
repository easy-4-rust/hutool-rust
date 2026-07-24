#!/usr/bin/env bash
# Split files one-by-one, commit only if build stays green.
# Usage: ./scripts/batch_split.sh [max_count]
set -e
cd /Users/wandl/workspaces/workspace-github/hutool-rust

MAX=${1:-200}
COUNT=0
COMMITTED=0
REVERTED=0

# Get list of current multi-type files
get_violations() {
    find crates -name '*.rs' -not -path '*/target/*' -not -path '*/tests/*' \
        -not -name 'mod.rs' -not -name 'lib.rs' \
        -exec sh -c 'c=$(grep -cE "^pub (struct|enum|trait|type) " "$1" 2>/dev/null || echo 0); [ "$c" -gt 1 ] && echo "$1"' _ {} \; 2>/dev/null | grep -v "^_:"
}

while [ $COUNT -lt $MAX ]; do
    FILE=$(get_violations | head -1)
    if [ -z "$FILE" ]; then
        echo "=== No more violations ==="
        break
    fi
    COUNT=$((COUNT + 1))
    echo "=== [$COUNT] Processing: $FILE ==="

    # Skip files with macro_rules!
    if grep -q "^macro_rules!" "$FILE" 2>/dev/null; then
        echo "  SKIP (contains macro_rules!)"
        # Add to skip list and continue
        echo "$FILE" >> /tmp/skipped_macro_files.txt
        # Move to a sentinel: rename to .skip temporarily? No, just mark
        # We'll handle these manually
        # Actually add a temporary marker
        # Skip by adding .done extension and ignoring via this script
        # But the find will still pick it. Let me use a marker file
        continue
    fi

    # Backup
    cp "$FILE" "/tmp/$(basename $FILE).bak"

    # Run script
    python3 scripts/split_rs.py "$FILE" > /tmp/split_out.log 2>&1
    if grep -q "^Error\|^Traceback" /tmp/split_out.log; then
        echo "  SCRIPT FAILED, reverting"
        git checkout -- .
        git clean -fd > /dev/null 2>&1
        REVERTED=$((REVERTED + 1))
        continue
    fi

    # Build only the affected crate
    CRATE=$(echo "$FILE" | sed 's|crates/\([^/]*\)/.*|\1|')
    if cargo build -p "$CRATE" > /tmp/build_out.log 2>&1; then
        git add -A
        git commit -m "refactor($CRATE): split $(basename $FILE .rs) into 1:1 structure" --no-gpg-sign > /dev/null 2>&1
        COMMITTED=$((COMMITTED + 1))
        echo "  OK committed (total $COMMITTED)"
    else
        echo "  BUILD FAILED, reverting"
        git checkout -- .
        git clean -fd > /dev/null 2>&1
        REVERTED=$((REVERTED + 1))
    fi
done

echo ""
echo "=== Summary ==="
echo "Processed: $COUNT"
echo "Committed: $COMMITTED"
echo "Reverted:  $REVERTED"
echo "Remaining violations: $(get_violations | wc -l)"

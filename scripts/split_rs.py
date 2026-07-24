#!/usr/bin/env python3
"""
Split each pub type into its own file. O(n) single-pass scan.
"""

import re
import sys
import shutil
from pathlib import Path


def to_snake(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def scan_file(content):
    """Single-pass: identify all top-level items by tracking brace depth and pub/impl keywords.

    Returns:
        header_lines: list of lines (imports, file docs, inner attributes)
        items: list of dicts with keys: kind, name, start, end, body_text
              kind in {pub_struct, pub_enum, pub_trait, pub_type, impl, helper_const,
                       helper_fn, helper_struct, helper_static, helper_type, helper_enum,
                       test_mod, helper_fn_with_body}
    """
    lines = content.split('\n')
    n = len(lines)
    items = []
    header_end = 0

    # Find header (use/pub use/file doc/inner attrs up to first non-allowed line).
    # Track brace depth to handle multi-line `use foo::{ bar, baz };`.
    i = 0
    header_end = 0
    brace_depth = 0
    while i < n:
        s = lines[i].strip()
        if brace_depth > 0:
            # Inside multi-line use block; count braces
            brace_depth += lines[i].count('{') - lines[i].count('}')
            header_end = i + 1
            i += 1
            continue
        if (s.startswith('//!') or s.startswith('#![') or
            s.startswith('use ') or s.startswith('pub use ') or
            s.startswith('extern crate')):
            brace_depth = lines[i].count('{') - lines[i].count('}')
            header_end = i + 1
            i += 1
        elif s == '' and i <= header_end:
            header_end = i + 1
            i += 1
        elif i == 0:
            header_end = 1
            i += 1
        else:
            break

    # Macro spans must be excluded from later impl/helper detection
    macro_spans = [(it['start'], it['end']) for it in items if it['kind'] == 'macro']
    def in_macro(line_idx):
        for s, e in macro_spans:
            if s <= line_idx < e:
                return True
        return False
    # test_mod spans (so we skip impl/helper inside them)
    test_spans = [(it['start'], it['end']) for it in items if it['kind'] == 'test_mod']
    def in_test(line_idx):
        for s, e in test_spans:
            if s <= line_idx < e:
                return True
        return False
    while i < n:
        line = lines[i]
        raw = line  # unstripped, to detect top-level only
        s = line.strip()
        if s == '' or s.startswith('//') or s.startswith('#['):
            i += 1
            continue
        # pub struct/enum/trait/type
        m = re.match(r'^pub\s+(struct|enum|trait|type)\s+(\w+)', s)
        if m:
            kind, name = m.group(1), m.group(2)
            # Walk back to include doc comments and attributes
            attrs_start = i
            j = i - 1
            while j >= 0 and (lines[j].strip().startswith('///') or
                               lines[j].strip().startswith('#[') or
                               lines[j].strip() == ''):
                if lines[j].strip() != '':
                    attrs_start = j
                j -= 1
            # If we hit non-empty line, ensure we include the rightmost non-empty
            if attrs_start == i:
                j = i - 1
                while j >= 0 and lines[j].strip() == '':
                    j -= 1
                if j >= 0 and (lines[j].strip().startswith('///') or lines[j].strip().startswith('#[')):
                    attrs_start = j
                    j -= 1
                    while j >= 0 and (lines[j].strip().startswith('///') or lines[j].strip().startswith('#[')):
                        attrs_start = j
                        j -= 1
            # Check if struct/enum/trait has multi-line definition (e.g. with where clause)
            has_open = '{' in line
            if not has_open and kind in ('struct', 'enum', 'trait') and ('<' in line or '(' in line):
                j = i + 1
                found_brace = False
                while j < n and j < i + 50:
                    s_j = lines[j].strip()
                    if s_j == '':
                        j += 1
                        continue
                    if '{' in lines[j]:
                        found_brace = True
                        break
                    if re.match(r'^(pub\s+)?(fn|struct|enum|impl|const|static|type)\s', s_j) or s_j.startswith('//') or s_j.startswith('#['):
                        break
                    j += 1
                if found_brace:
                    has_open = True
            if has_open and not line.rstrip().endswith(';'):
                depth = 0
                started = False
                j = i
                while j < n:
                    for ch in lines[j]:
                        if ch == '{':
                            depth += 1
                            started = True
                        elif ch == '}':
                            depth -= 1
                    if started and depth == 0:
                        items.append({'kind': f'pub_{kind}', 'name': name, 'start': attrs_start, 'end': j + 1})
                        i = j + 1
                        break
                    j += 1
                else:
                    items.append({'kind': f'pub_{kind}', 'name': name, 'start': attrs_start, 'end': n})
                    i = n
            else:
                items.append({'kind': f'pub_{kind}', 'name': name, 'start': attrs_start, 'end': i + 1})
                i += 1
            continue
        # macro_rules! ... { ... } - skip the whole macro as opaque block
        if s.startswith('macro_rules!') or re.match(r'^macro\s+\w+', s):
            depth = 0
            started = False
            j = i
            while j < n:
                for ch in lines[j]:
                    if ch == '{':
                        depth += 1
                        started = True
                    elif ch == '}':
                        depth -= 1
                if started and depth == 0:
                    items.append({'kind': 'macro', 'name': 'macro', 'start': i, 'end': j + 1})
                    i = j + 1
                    break
                j += 1
            else:
                i += 1
            continue
        # impl X or impl<T> X (skip if inside macro)
        # Only top-level: line must start with `impl` (no leading whitespace)
        m = re.match(r'^impl(?:<[^>]*>)?\s+(\w+)\b', raw)
        if m and not in_macro(i) and not in_test(i):
            name = m.group(1)
            depth = 0
            started = False
            j = i
            while j < n:
                for ch in lines[j]:
                    if ch == '{':
                        depth += 1
                        started = True
                    elif ch == '}':
                        depth -= 1
                if started and depth == 0:
                    items.append({'kind': 'impl', 'name': name, 'start': i, 'end': j + 1})
                    i = j + 1
                    break
                j += 1
            else:
                i += 1
            continue
        if m and in_macro(i):
            # Skip impl inside macro by advancing to next balanced brace
            depth = 0
            started = False
            j = i
            while j < n:
                for ch in lines[j]:
                    if ch == '{':
                        depth += 1
                        started = True
                    elif ch == '}':
                        depth -= 1
                if started and depth == 0:
                    i = j + 1
                    break
                j += 1
            else:
                i += 1
            continue
        # #[cfg(test)] mod tests (must be at column 0, not inside another block)
        if s == '#[cfg(test)]' and i + 1 < n and lines[i+1].strip().startswith('mod tests') and not lines[i+1].startswith(' '):
            depth = 0
            started = False
            j = i + 1
            while j < n:
                for ch in lines[j]:
                    if ch == '{':
                        depth += 1
                        started = True
                    elif ch == '}':
                        depth -= 1
                if started and depth == 0:
                    items.append({'kind': 'test_mod', 'name': 'tests', 'start': i, 'end': j + 1})
                    i = j + 1
                    break
                j += 1
            else:
                i += 1
            continue
        # non-pub top-level: const, fn, static, struct, type, enum
        # Also handle `pub fn`/`pub const`/`pub static` - these are public helpers
        m = re.match(r'^(pub(?:\s*\(\s*crate\s*\))?\s+)?(async\s+)?(const|fn|static|struct|enum|type)\s+(\w+)', raw)
        if m and not in_macro(i) and not in_test(i):
            is_pub = m.group(1) is not None
            is_async = m.group(2) is not None
            kind_word, name = m.group(3), m.group(4)
            kind = f'helper_{kind_word}'
            # Public helpers need pub(crate) to be visible across files in the crate
            # We'll wrap their bodies later if needed. For now, keep them.
            # Determine if body spans multiple lines.
            has_open = '{' in line
            multi = has_open and not line.rstrip().endswith(';')
            # Detect fn/struct/enum with multi-line signature spanning to a later `{`
            if not multi and kind_word in ('fn', 'struct', 'enum') and ('(' in line or '<' in line) and not line.rstrip().endswith(';'):
                j = i + 1
                found_brace = False
                while j < n and j < i + 50:
                    s_j = lines[j].strip()
                    if s_j == '':
                        j += 1
                        continue
                    if '{' in lines[j]:
                        found_brace = True
                        break
                    if re.match(r'^(pub\s+)?(fn|struct|enum|impl|const|static|type)\s', s_j) or s_j.startswith('//') or s_j.startswith('#['):
                        break
                    j += 1
                if found_brace:
                    multi = True
            if not multi and kind_word == 'const' and '=' in line and not line.rstrip().endswith(';'):
                j = i + 1
                while j < n and lines[j].strip() == '':
                    j += 1
                if j < n and (lines[j].lstrip().startswith('"') or
                              lines[j].lstrip().startswith("'") or
                              lines[j].lstrip().startswith('&') or
                              lines[j].lstrip().startswith('[') or
                              lines[j].lstrip().startswith('(') or
                              lines[j].lstrip().startswith('-') or
                              lines[j].lstrip().startswith('b"') or
                              lines[j].lstrip().startswith("b'")):
                    multi = True
            if multi:
                # Special case: const with string/byte literal body ends at `;`
                next_line = lines[i + 1].lstrip() if i + 1 < n else ''
                if kind_word == 'const' and (next_line.startswith('b"') or next_line.startswith("b'") or next_line.startswith('"') or next_line.startswith("'")):
                    j = i + 1
                    while j < n and not lines[j].rstrip().endswith(';'):
                        j += 1
                    if j < n:
                        items.append({'kind': kind, 'name': name, 'start': i, 'end': j + 1, 'is_pub': is_pub})
                        i = j + 1
                    else:
                        items.append({'kind': kind, 'name': name, 'start': i, 'end': n, 'is_pub': is_pub})
                        i = n
                else:
                    depth = 0
                    started = False
                    j = i
                    while j < n:
                        for ch in lines[j]:
                            if ch == '{' or (kind_word == 'const' and ch in '[('):
                                depth += 1
                                started = True
                            elif ch == '}' or (kind_word == 'const' and ch in '])'):
                                depth -= 1
                        if started and depth == 0:
                            items.append({'kind': kind, 'name': name, 'start': i, 'end': j + 1, 'is_pub': is_pub})
                            i = j + 1
                            break
                        j += 1
                    else:
                        items.append({'kind': kind, 'name': name, 'start': i, 'end': n, 'is_pub': is_pub})
                        i = n
            else:
                items.append({'kind': kind, 'name': name, 'start': i, 'end': i + 1, 'is_pub': is_pub})
                i += 1
            continue
        if m and in_macro(i):
            # Skip fn/const inside macro
            depth = 0
            started = False
            j = i
            while j < n:
                for ch in lines[j]:
                    if ch == '{':
                        depth += 1
                        started = True
                    elif ch == '}':
                        depth -= 1
                if started and depth == 0:
                    i = j + 1
                    break
                j += 1
            else:
                i += 1
            continue
        if s.startswith('mod '):
            i += 1
            continue
        i += 1

    return lines, header_end, items


def collect_used_names(text):
    return set(re.findall(r'\b[A-Za-z_][A-Za-z_0-9]*\b', text))


def split_file(file_path):
    content = file_path.read_text()
    lines, header_end, items = scan_file(content)

    pub_items = [it for it in items if it['kind'].startswith('pub_')]
    if len(pub_items) < 2:
        print(f"Skip {file_path}: only {len(pub_items)} pub types")
        return

    header_text = '\n'.join(lines[:header_end]).rstrip()
    helper_items = [it for it in items if it['kind'].startswith('helper_')]

    helper_by_name = {it['name']: it for it in helper_items}
    pub_type_names = {it['name'] for it in pub_items}

    # Map impl -> closest preceding pub type
    impl_to_type = {}
    for idx, it in enumerate(items):
        if it['kind'] != 'impl':
            continue
        best = None
        best_idx = -1
        for pi_idx, pi in enumerate(pub_items):
            if pi['end'] <= it['start']:
                if pi_idx > best_idx:
                    best = pi
                    best_idx = pi_idx
        if best is not None:
            impl_to_type[idx] = best

    # For each pub type, find impls + helpers used
    type_data = {pi['name']: {'impls': [], 'helpers': set(), 'macros': []} for pi in pub_items}
    for idx, it in enumerate(items):
        if it['kind'] == 'impl':
            tgt = impl_to_type.get(idx)
            if tgt is not None:
                type_data[tgt['name']]['impls'].append(it)
        elif it['kind'] == 'macro':
            # attach macro to closest preceding pub type
            best = None
            best_idx = -1
            for pi_idx, pi in enumerate(pub_items):
                if pi['end'] <= it['start']:
                    if pi_idx > best_idx:
                        best = pi
                        best_idx = pi_idx
            if best is not None:
                type_data[best['name']]['macros'].append(it)
    # find which helpers are referenced from each type's text (incl impls)
    for pi_idx, pi in enumerate(pub_items):
        chunks = [''.join(lines[pi['start']:pi['end']])]
        for it in type_data[pi['name']]['impls']:
            chunks.append(''.join(lines[it['start']:it['end']]))
        all_text = '\n'.join(chunks)
        used = collect_used_names(all_text)
        for hname in used:
            if hname in helper_by_name and hname not in pub_type_names:
                # store by index to keep set hashable
                type_data[pi['name']]['helpers'].add(helper_by_name[hname]['name'])

    crate_root = file_path.parent
    stem = file_path.stem
    sub_dir = crate_root / stem
    if sub_dir.exists():
        shutil.rmtree(sub_dir)
    sub_dir.mkdir()

    for pi in pub_items:
        snake = to_snake(pi['name'])
        chunks = []
        if header_text:
            chunks.append(header_text)
            chunks.append('')
        # Compute referenced siblings
        all_text = ''.join(lines[pi['start']:pi['end']])
        for it in type_data[pi['name']]['impls']:
            all_text += ''.join(lines[it['start']:it['end']])
        for h in type_data[pi['name']]['helpers']:
            all_text += ''.join(lines[helper_by_name[h]['start']:helper_by_name[h]['end']])
        siblings = []
        for other in pub_items:
            if other['name'] == pi['name']:
                continue
            if re.search(rf'\b{other["name"]}\b', all_text):
                siblings.append(other['name'])
        if siblings:
            chunks.append('\n'.join(f'use super::{to_snake(s)}::{s};' for s in sorted(siblings)))
            chunks.append('')
        # Type def
        chunks.append('\n'.join(lines[pi['start']:pi['end']]).rstrip())
        # Impls
        for it in type_data[pi['name']]['impls']:
            chunks.append('')
            chunks.append('\n'.join(lines[it['start']:it['end']]).rstrip())
        # Macros that go with this type
        for m in type_data[pi['name']]['macros']:
            chunks.append('')
            chunks.append('\n'.join(lines[m['start']:m['end']]).rstrip())
        # Helpers: import ALL helpers from mod.rs
        all_helpers = sorted({h['name'] for h in helper_items})
        if all_helpers:
            chunks.append('')
            for i in range(0, len(all_helpers), 8):
                batch = all_helpers[i:i+8]
                chunks.append(f"use super::{{{', '.join(batch)}}};")
        new_path = sub_dir / f"{snake}.rs"
        new_path.write_text('\n'.join(chunks).rstrip() + '\n')

    # Pre-step: assign each pub helper to its first owning type.
    # If no type uses it, assign to the first pub type.
    for h in helper_items:
        if not h.get('is_pub'):
            continue
        owner = None
        for tname, tdata in type_data.items():
            if h['name'] in tdata['helpers']:
                owner = tname
                break
        if owner is None and pub_items:
            owner = pub_items[0]['name']
            type_data[owner]['helpers'].add(h['name'])

    # mod.rs
    mod_chunks = []
    if header_text:
        mod_chunks.append(header_text)
        mod_chunks.append('')
    for pi in pub_items:
        mod_chunks.append(f"mod {to_snake(pi['name'])};")
    mod_chunks.append('')
    for pi in pub_items:
        mod_chunks.append(f"pub use {to_snake(pi['name'])}::{pi['name']};")
    # Emit ALL helpers directly in mod.rs (both pub and non-pub)
    # This ensures submodules can use super::name for any helper
    for h in helper_items:
        mod_chunks.append('')
        mod_chunks.append('\n'.join(lines[h['start']:h['end']]).rstrip())
    mod_chunks.append('')
    (sub_dir / 'mod.rs').write_text('\n'.join(mod_chunks))

    file_path.unlink()
    print(f"Split {file_path} -> {sub_dir}/ ({len(pub_items)} types)")


def main():
    if len(sys.argv) < 2:
        print("Usage: split_rs.py <file.rs> [more files...]")
        sys.exit(1)
    for arg in sys.argv[1:]:
        try:
            split_file(Path(arg))
        except FileNotFoundError:
            print(f"Skip {arg}: not found (already split?)")
        except Exception as e:
            import traceback
            print(f"Error processing {arg}: {e}")
            traceback.print_exc()


if __name__ == '__main__':
    main()
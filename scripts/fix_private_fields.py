import re, sys
from pathlib import Path

def fix_private_fields(crate_dir, field_names):
    for rs_file in Path(crate_dir).rglob('*.rs'):
        if '/target/' in str(rs_file): continue
        content = rs_file.read_text()
        lines = content.split('\n')
        modified = False
        in_struct_def = False
        struct_depth = 0
        for i, line in enumerate(lines):
            stripped = line.strip()
            if re.match(r'^(pub\s+)?(pub\(crate\)\s+)?struct\s+\w+', stripped):
                in_struct_def = True
                struct_depth = 0
                for ch in line:
                    if ch == '{': struct_depth += 1
                    elif ch == '}': struct_depth -= 1
                continue
            if in_struct_def:
                for ch in line:
                    if ch == '{': struct_depth += 1
                    elif ch == '}': struct_depth -= 1
                if struct_depth <= 0:
                    in_struct_def = False
                    continue
                for field_name in field_names:
                    pattern = rf'^(\s+){field_name}:\s'
                    match = re.match(pattern, line)
                    if match and not line.strip().startswith('pub(crate)'):
                        indent = match.group(1)
                        lines[i] = f"{indent}pub(crate) {line.lstrip()}"
                        modified = True
        if modified:
            rs_file.write_text('\n'.join(lines))

if __name__ == '__main__':
    fix_private_fields(sys.argv[1], sys.argv[2:])

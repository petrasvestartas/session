import re
import sys
import json

def parse_step(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Join continuation lines (lines not starting with #)
    lines = content.split('\n')
    entities = {}
    current = ''
    for line in lines:
        line = line.strip()
        if not line or line.startswith('/*') or line in ('ISO-10303-21;', 'HEADER;', 'ENDSEC;', 'DATA;', 'END-ISO-10303-21;'):
            continue
        if re.match(r'#\d+=', line):
            if current:
                m = re.match(r'#(\d+)=(.*)', current)
                if m:
                    entities[int(m.group(1))] = m.group(2).rstrip(';')
            current = line
        else:
            current += line
    if current:
        m = re.match(r'#(\d+)=(.*)', current)
        if m:
            entities[int(m.group(1))] = m.group(2).rstrip(';')

    return entities

def parse_cartesian_point(s):
    m = re.match(r"CARTESIAN_POINT\('',\(([-\d.eE+]+),([-\d.eE+]+),([-\d.eE+]+)\)\)", s)
    if m:
        return (float(m.group(1)), float(m.group(2)), float(m.group(3)))
    return None

def extract_refs(text):
    """Extract #NNN references from text"""
    return [int(x) for x in re.findall(r'#(\d+)', text)]

def parse_bspline_surface(entity_str, entities):
    # B_SPLINE_SURFACE_WITH_KNOTS('',degree_u,degree_v,((cp_refs...),...),
    #   .UNSPECIFIED.,.F.,.F.,.F.,
    #   (u_mults),(v_mults),(u_knots),(v_knots),.UNSPECIFIED.)

    # Extract degrees
    m = re.match(r"B_SPLINE_SURFACE_WITH_KNOTS\('',(\d+),(\d+),\(\(", entity_str)
    if not m:
        return None
    degree_u = int(m.group(1))
    degree_v = int(m.group(2))

    # Find the control point grid - between (( and ))
    # The CP grid is enclosed in (( ... ))
    # Find the start after the second comma
    cp_start = entity_str.index('((')

    # Count balanced parens to find end of CP grid
    depth = 0
    cp_end = cp_start
    for i in range(cp_start, len(entity_str)):
        if entity_str[i] == '(':
            depth += 1
        elif entity_str[i] == ')':
            depth -= 1
            if depth == 0:
                cp_end = i + 1
                break

    cp_block = entity_str[cp_start:cp_end]

    # Parse rows of CPs
    rows = re.findall(r'\(([^()]+)\)', cp_block)
    cp_grid = []
    for row in rows:
        refs = [int(x) for x in re.findall(r'#(\d+)', row)]
        cp_grid.append(refs)

    n_rows = len(cp_grid)
    n_cols = len(cp_grid[0]) if cp_grid else 0

    # Get control point coordinates
    points = []
    for row in cp_grid:
        row_pts = []
        for ref in row:
            pt = parse_cartesian_point(entities[ref])
            row_pts.append(pt)
        points.append(row_pts)

    # Parse the rest after the CP grid
    rest = entity_str[cp_end:]

    # Find knot multiplicities and knot values
    # Pattern: .UNSPECIFIED.,.F.,.F.,.F.,(u_mults),(v_mults),(u_knots),(v_knots),.UNSPECIFIED.
    # Find all parenthesized number lists
    num_lists = re.findall(r'\(([-\d.,eE+ ]+)\)', rest)

    if len(num_lists) >= 4:
        u_mults = [int(x.strip()) for x in num_lists[0].split(',')]
        v_mults = [int(x.strip()) for x in num_lists[1].split(',')]
        u_knots = [float(x.strip()) for x in num_lists[2].split(',')]
        v_knots = [float(x.strip()) for x in num_lists[3].split(',')]
    else:
        u_mults = v_mults = u_knots = v_knots = []

    return {
        'degree_u': degree_u,
        'degree_v': degree_v,
        'n_rows': n_rows,
        'n_cols': n_cols,
        'points': points,
        'u_mults': u_mults,
        'v_mults': v_mults,
        'u_knots': u_knots,
        'v_knots': v_knots
    }

def main():
    entities = parse_step('session_data/annen.stp')

    # Find all B_SPLINE_SURFACE_WITH_KNOTS
    surfaces = []
    for eid, estr in sorted(entities.items()):
        if estr.startswith('B_SPLINE_SURFACE_WITH_KNOTS'):
            srf = parse_bspline_surface(estr, entities)
            if srf:
                surfaces.append((eid, srf))

    print(f"Found {len(surfaces)} surfaces")
    for i, (eid, srf) in enumerate(surfaces):
        print(f"\nSurface {i} (#{eid}): degree {srf['degree_u']}x{srf['degree_v']}, "
              f"CVs {srf['n_rows']}x{srf['n_cols']}")
        print(f"  U knots ({len(srf['u_knots'])}): [{srf['u_knots'][0]:.2f} ... {srf['u_knots'][-1]:.2f}]")
        print(f"  V knots ({len(srf['v_knots'])}): [{srf['v_knots'][0]:.2f} ... {srf['v_knots'][-1]:.2f}]")
        print(f"  U mults: {srf['u_mults']}")
        print(f"  V mults: {srf['v_mults']}")
        # Print first and last CP
        p0 = srf['points'][0][0]
        pn = srf['points'][-1][-1]
        print(f"  First CP: ({p0[0]:.4f}, {p0[1]:.4f}, {p0[2]:.4f})")
        print(f"  Last CP:  ({pn[0]:.4f}, {pn[1]:.4f}, {pn[2]:.4f})")

    # Output as JSON for easy consumption
    output = []
    for i, (eid, srf) in enumerate(surfaces):
        output.append({
            'index': i,
            'step_id': eid,
            'degree_u': srf['degree_u'],
            'degree_v': srf['degree_v'],
            'n_u': srf['n_rows'],
            'n_v': srf['n_cols'],
            'u_mults': srf['u_mults'],
            'v_mults': srf['v_mults'],
            'u_knots': srf['u_knots'],
            'v_knots': srf['v_knots'],
            'points': [[[p[0], p[1], p[2]] for p in row] for row in srf['points']]
        })

    with open('session_data/annen_surfaces.json', 'w') as f:
        json.dump(output, f, indent=1)
    print(f"\nWrote {len(output)} surfaces to session_data/annen_surfaces.json")

if __name__ == '__main__':
    main()

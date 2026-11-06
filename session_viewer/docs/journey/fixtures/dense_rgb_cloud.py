import argparse
import array
import bisect
import hashlib
import json
import sys
import math
import struct
from pathlib import Path

COUNT = 196608

def sha(data):
    return hashlib.sha256(data).hexdigest()

def varint(value):
    result = bytearray()
    while value >= 128:
        result.append((value & 127) | 128)
        value >>= 7
    result.append(value)
    return bytes(result)

def field(number, value):
    if isinstance(value, str):
        value = value.encode()
    return varint(number * 8 + 2) + varint(len(value)) + value

def radical_inverse(index, base):
    value, factor = 0.0, 1.0 / base
    while index:
        value += (index % base) * factor
        index //= base
        factor /= base
    return value

def mesh(source):
    vertices, triangles, cumulative = [], [], []
    for line in source.splitlines():
        words = line.split()
        if not words:
            continue
        if words[0] == 'v':
            vertices.append(tuple(map(float, words[1:4])))
        elif words[0] == 'f':
            ids = [int(word.split('/')[0]) for word in words[1:]]
            ids = [i - 1 if i > 0 else len(vertices) + i for i in ids]
            for i in range(1, len(ids) - 1):
                a, b, c = (vertices[j] for j in [ids[0], ids[i], ids[i + 1]])
                u, v = [b[k] - a[k] for k in range(3)], [c[k] - a[k] for k in range(3)]
                cross = [u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0]]
                area = math.sqrt(sum(x*x for x in cross)) * 0.5
                if area > 0:
                    triangles.append((a, b, c))
                    cumulative.append((cumulative[-1] if cumulative else 0.0) + area)
    assert len(vertices) == 2503 and len(triangles) == 4968
    return vertices, triangles, cumulative

def source(obj):
    count = COUNT
    assert sha(obj.read_bytes()) == 'f545d83b0d6845fff8b0cea5ed6c1fc049f5855eb1910cf628878d9655454c3e', 'Bundled bunny.obj differs'
    vertices, triangles, cumulative = mesh(obj.read_text(encoding='utf-8'))
    lo = [min(p[k] for p in vertices) for k in range(3)]
    hi = [max(p[k] for p in vertices) for k in range(3)]
    center = [(a+b)*0.5 for a,b in zip(lo,hi)]
    extent = max(b-a for a,b in zip(lo,hi))
    coords, colors = array.array('d'), bytearray()
    for i in range(count):
        triangle = min(bisect.bisect_left(cumulative, (i+0.5)*cumulative[-1]/count), len(triangles)-1)
        a,b,c = triangles[triangle]
        u, v = math.sqrt(radical_inverse(i+1, 2)), radical_inverse(i+1, 3)
        p = [(1-u)*a[k]+u*(1-v)*b[k]+u*v*c[k] for k in range(3)]
        p = [(p[k]-center[k])/extent for k in range(3)]
        world = [1600*p[0], -1600*p[2], 1600*p[1]]
        coords.extend(world)
        rgba = [round(55+180*(p[1]+0.5)), round(60+170*(p[0]+0.5)), round(65+160*(p[2]+0.5)), 255]
        for value in rgba:
            assert 0 <= value <= 255
            colors.extend(varint(value))
    assert coords.itemsize == 8
    if sys.byteorder != 'little':
        coords.byteswap()
    xyz = coords.tobytes()
    cloud_guid = '794ba6e8-4650-5dc1-8d35-203c3ce3f401'
    root_guid = '794ba6e8-4650-5dc1-8d35-203c3ce3f402'
    name = 'Synthetic dense RGB bunny — mesh samples, procedural colors'
    cloud = field(1, cloud_guid) + field(2, name) + field(3, xyz) + field(4, colors) + varint(6*8+1) + struct.pack('<d', 3.0)
    objects = field(1, 'Synthetic dense illustration') + field(2, '794ba6e8-4650-5dc1-8d35-203c3ce3f403') + field(8, cloud)
    child = field(1, '794ba6e8-4650-5dc1-8d35-203c3ce3f404') + field(2, cloud_guid) + field(3, root_guid)
    tree_root = field(1, root_guid) + field(2, name) + field(4, child)
    tree = field(1, '794ba6e8-4650-5dc1-8d35-203c3ce3f405') + field(2, name) + field(3, tree_root)
    document = field(1, name) + field(2, '794ba6e8-4650-5dc1-8d35-203c3ce3f406') + field(3, objects) + field(4, tree)
    assert len(document) < 8*1024*1024
    assert sha(document) == '9cd395b1b97754b991f7473a867087497d0bf8e13a5aef2cc31afdd786d97207', 'Generated fixture differs'
    return document

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Recreate the synthetic 196,608-point RGB illustration fixture using only Python stdlib.')
    parser.add_argument('--output', type=Path, required=True, help='New .pb file; existing files are never overwritten')
    parser.add_argument('--mesh', type=Path, default=Path(__file__).resolve().parents[4] / 'session_data/bunny.obj')
    args = parser.parse_args()
    if args.output.exists():
        parser.error('Output exists; choose a new file')
    data = source(args.mesh)
    with args.output.open('xb') as stream:
        stream.write(data)
    print(json.dumps({'path': str(args.output), 'points': COUNT, 'bytes': len(data), 'sha256': sha(data)}))

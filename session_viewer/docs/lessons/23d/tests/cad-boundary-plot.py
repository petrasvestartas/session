#!/usr/bin/env python3
"""Plot exact CAD audit triangles, original boundary polygons and their nodes.

Requires matplotlib. Input is examples/cad_boundary_audit.rs geometry.json.
This CPU diagnostic deliberately shows boundary polygons through surfaces; it does
not simulate the viewer depth test or introduce additional CAD topology.
"""
import argparse
import json
from pathlib import Path

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d.art3d import Poly3DCollection


def panel(ax, brep):
    """Draw the actual producer mesh with original boundary IDs and sampled nodes."""
    faces = brep['face_data']
    meshes = []
    all_positions = []
    for face in faces:
        vertices = {v['key']: v['position'] for v in face['vertices']}
        meshes.append(vertices)
        all_positions.extend(vertices.values())
        triangles = [[vertices[key] for key in ids] for ids in face['triangles'].values()]
        ax.add_collection3d(Poly3DCollection(triangles, facecolors='#e1e6ea',
            edgecolors='#86929d', linewidths=0.18, alpha=0.3, zsort='average'))
    colors = ['#d71e35', '#0067cc', '#18964c', '#a02dc2', '#d67500']
    total = 0
    for edge in brep['edges']:
        if not edge['available']:
            continue
        points = np.array([meshes[edge['owner']][key] for key in edge['keys']])
        color = colors[edge['edge'] % len(colors)]
        ax.plot(*points.T, color=color, linewidth=2.0, label=f"E{edge['edge']}: {edge['segments']} segments")
        ax.scatter(*points.T, color=color, s=6, depthshade=False)
        total += edge['segments']
    points = np.array(all_positions)
    center = (points.max(axis=0)+points.min(axis=0))*0.5
    radius = max(points.max(axis=0)-points.min(axis=0))*0.56
    ax.set_xlim(center[0]-radius, center[0]+radius)
    ax.set_ylim(center[1]-radius, center[1]+radius)
    ax.set_zlim(center[2]-radius, center[2]+radius)
    ax.set_box_aspect((1,1,1))
    ax.view_init(elev=24, azim=38)
    ax.set_axis_off()
    count = sum(len(face['triangles']) for face in faces)
    ax.set_title(f"{brep['name'].upper()}  ·  {count:,} triangles\n{total} CAD boundary segments, all exact triangle edges", fontsize=12)
    ax.legend(loc='lower left', fontsize=8, frameon=False)


def main():
    """Prefer later exact-file objects over same-name synthetic control fixtures."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('audit', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    by_name = {brep['name']: brep for brep in json.loads(args.audit.read_text())}
    figure = plt.figure(figsize=(16,13), facecolor='white')
    for slot, name in enumerate(('cylinder','cone','sphere','torus')):
        panel(figure.add_subplot(2,2,slot+1,projection='3d',computed_zorder=False), by_name[name])
    figure.suptitle('Actual mixed-scene tessellation and source CAD boundary nodes', fontsize=19)
    figure.text(.5,.018,'Grey = exact face triangles   ·   Coloured lines + dots = original CAD edge polygons + their mesh vertices\n'
        'Transparent inspection view: hidden boundaries remain visible here. No curve resampling or added subdivisions.',ha='center',fontsize=11)
    figure.subplots_adjust(top=.93,bottom=.07,left=.015,right=.985,hspace=.08,wspace=.01)
    args.output.parent.mkdir(parents=True,exist_ok=True)
    figure.savefig(args.output,dpi=130)
    figure.savefig(args.output.with_suffix('.svg'))


if __name__ == '__main__':
    main()

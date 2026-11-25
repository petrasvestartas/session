<template>
  <div class="course-index">
    <p class="eyebrow">ETH Zürich · Block Research Group · Session WebGPU</p>
    <div class="hero-top">
      <div>
        <h1>Viewer course</h1>
        <p class="subhead">17 visual engineering tutorials · 388 verified Rust &amp; WGSL files</p>
      </div>
      <a class="launch-viewer-btn" :href="viewerHref">Open 3D Viewer ↗</a>
    </div>
    <div class="spine" aria-hidden="true">
      <i v-for="c in CAS_RAMP" :key="c" :style="{ background: c }"></i>
    </div>
    <p class="lead">
      Build the Session Viewer from scratch in Rust and WebGPU across {{ checkpointCount.toLocaleString('en-US') }} cumulative checkpoints, from the first cleared surface to every WGSL shader, 3D presentation stage and scene tool. Install Rust <code>1.97.1</code>, the browser target and Trunk <code>0.21.14</code>, then follow the 17 tutorials step by step:
    </p>
    <div class="setup-box">
      <div class="setup-head">
        <strong>Toolchain setup</strong>
        <span>Rust 1.97.1 · wasm32-unknown-unknown · Trunk 0.21.14</span>
      </div>
      <pre class="setup-code"><code>rustup toolchain install 1.97.1 --component rustfmt clippy
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14 --locked</code></pre>
    </div>
    <ol class="tutorial-list">
      <li
        v-for="(t, idx) in tutorials"
        :key="t.id"
        class="tutorial-item"
        :style="{ '--c': rampColor(idx, tutorials.length) }">
        <div class="tutorial-card">
          <div class="tutorial-body">
            <router-link :to="'/course/' + t.slug" class="tutorial-link">
              <span class="tutorial-dot">{{ t.number }}</span>
              <span class="tutorial-title">{{ t.title }}</span>
            </router-link>
            <p v-if="t.summary" class="tutorial-summary">{{ t.summary }}</p>
            <div class="tutorial-meta">
              <span v-if="t.fileCount" class="meta-chip">{{ t.fileCount }} source files</span>
            </div>
            <ol v-if="t.chapters.length" class="chapter-list">
              <li v-for="c in t.chapters" :key="c.slug" class="chapter-item">
                <div class="chapter-row">
                  <router-link :to="'/course/' + c.slug" class="tutorial-link chapter-link">
                    <span class="chapter-pill">{{ c.number }}</span>
                    <span class="tutorial-title">{{ c.title }}</span>
                  </router-link>
                  <span v-if="c.fileCount" class="chapter-files">{{ c.fileCount }} files</span>
                </div>
              </li>
            </ol>
          </div>
          <router-link
            v-if="t.heroImg"
            :to="'/course/' + t.slug"
            class="tutorial-thumb-link"
            tabindex="-1"
            aria-hidden="true">
            <img :src="t.heroImg" alt="" loading="lazy" decoding="async" class="tutorial-thumb" />
          </router-link>
        </div>
      </li>
    </ol>
  </div>
</template>

<script setup lang="ts">
import { checkpointCount, tutorials } from 'virtual:course';

const base = import.meta.env.BASE_URL;
const viewerHref = new URL('../', window.location.origin + base).href;

const CAS_RAMP = ['#3057b1', '#5552bb', '#7750c1', '#934abd', '#ae4bb2', '#c54b9f', '#dc4b8d'];
const rampColor = (idx: number, total: number): string => {
  if (total <= 1) return CAS_RAMP[0];
  const pos = (idx / (total - 1)) * (CAS_RAMP.length - 1);
  return CAS_RAMP[Math.round(pos)] ?? CAS_RAMP[0];
};
</script>

<style scoped>
.course-index {
  max-width: 1020px;
  padding: 2rem 0.5rem 3.5rem;
  line-height: 1.55;
}

.eyebrow {
  margin: 0 0 0.3rem;
  font-size: 13px;
  font-weight: 700;
  letter-spacing: 0.02em;
  text-transform: uppercase;
  color: var(--pink);
}

.hero-top {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 0.75rem;
}

h1 {
  font-size: clamp(32px, 3.8vw, 44px);
  font-weight: 800;
  letter-spacing: -0.025em;
  line-height: 1.06;
  color: var(--fg);
  margin: 0;
}

.subhead {
  margin: 0.3rem 0 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--muted);
}

.launch-viewer-btn {
  display: inline-flex;
  align-items: center;
  padding: 0.5rem 0.95rem;
  border: 2px solid var(--accent);
  background: var(--accent);
  color: #ffffff;
  font-size: 13.5px;
  font-weight: 700;
  text-decoration: none;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.launch-viewer-btn:hover {
  background: var(--pink);
  border-color: var(--pink);
  color: #ffffff;
}

.spine {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 6px;
  height: 8px;
  margin: 0 0 1.25rem;
}

.lead {
  color: var(--fg);
  margin: 0 0 1.15rem;
  font-size: 15.5px;
  line-height: 1.62;
}

.lead code {
  font-size: 0.88em;
  background: var(--code-bg);
  padding: 0.1em 0.35em;
  border: 1px solid var(--rule);
}

.setup-box {
  margin: 0 0 2rem;
  border: 2px solid var(--accent);
  background: var(--paper);
}

.setup-head {
  padding: 0.48rem 0.9rem;
  background: var(--accent);
  color: #ffffff;
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  justify-content: space-between;
  gap: 0.5rem;
  font-size: 13.5px;
}

.setup-head span {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.86);
}

.setup-code {
  margin: 0;
  padding: 0.8rem 0.95rem;
  background: var(--code-bg);
  overflow-x: auto;
  font-size: 13px;
  line-height: 1.55;
  color: var(--fg);
}

.tutorial-list,
.chapter-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.tutorial-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.tutorial-item {
  border: 1.5px solid var(--rule);
  border-left: 4px solid var(--c, var(--accent));
  background: var(--paper);
  padding: 0.85rem 1rem;
  transition: border-color 0.15s ease, box-shadow 0.15s ease, background 0.15s ease;
}

.tutorial-item:hover {
  border-color: var(--c, var(--accent));
  background: color-mix(in srgb, var(--c, var(--accent)) 3.5%, #ffffff);
  box-shadow: 0 6px 18px rgba(20, 27, 61, 0.06);
}

.tutorial-card {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 216px;
  gap: 1.15rem;
  align-items: center;
}

.tutorial-body {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.tutorial-link {
  display: inline-flex;
  align-items: center;
  gap: 0.65rem;
  color: var(--fg);
  text-decoration: none;
  font-size: 17px;
  font-weight: 700;
  letter-spacing: -0.012em;
}

.tutorial-summary {
  margin: 0;
  padding-left: 2.3rem;
  font-size: 13.8px;
  line-height: 1.5;
  color: var(--muted);
}

.tutorial-meta {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5rem;
  padding-left: 2.3rem;
  margin-top: 0.1rem;
}

.meta-chip {
  font-size: 11.5px;
  font-weight: 600;
  color: var(--muted);
  background: var(--wash);
  padding: 0.12rem 0.5rem;
  border: 1px solid var(--rule);
}

.meta-stage {
  font-size: 11.5px;
  font-weight: 700;
  color: var(--c, var(--accent));
  background: color-mix(in srgb, var(--c, var(--accent)) 8%, #ffffff);
  padding: 0.12rem 0.55rem;
  border: 1px solid color-mix(in srgb, var(--c, var(--accent)) 45%, var(--rule));
  text-decoration: none;
  transition: background 0.15s ease, color 0.15s ease;
}

.meta-stage:hover {
  background: var(--c, var(--accent));
  color: #ffffff;
}

.tutorial-thumb-link {
  display: block;
  border: 1.5px solid var(--rule);
  background: #ffffff;
  overflow: hidden;
  aspect-ratio: 16 / 10;
}

.tutorial-thumb {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
  transition: transform 0.2s ease;
}

.tutorial-item:hover .tutorial-thumb {
  transform: scale(1.03);
}

.chapter-list {
  padding: 0.45rem 0 0.1rem 2.3rem;
  margin-top: 0.25rem;
  border-top: 1px dashed var(--rule);
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.chapter-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5rem;
}

.chapter-link {
  font-size: 14.5px;
  font-weight: 600;
  color: var(--fg);
}

.chapter-files {
  font-size: 11.5px;
  color: var(--muted);
}

.chapter-stage {
  padding: 0.06rem 0.45rem;
}

.tutorial-dot {
  width: 26px;
  height: 26px;
  flex: 0 0 26px;
  display: grid;
  place-items: center;
  font-variant-numeric: tabular-nums;
  font-size: 12.5px;
  font-weight: 700;
  border-radius: 50%;
  border: 2px solid var(--c, var(--accent));
  color: var(--c, var(--accent));
  background: #ffffff;
  transition: background 0.15s ease, color 0.15s ease;
}

.tutorial-link:hover .tutorial-dot {
  background: var(--c, var(--accent));
  color: #ffffff;
}

.chapter-pill {
  padding: 0.05rem 0.42rem;
  font-variant-numeric: tabular-nums;
  font-size: 12px;
  font-weight: 700;
  border: 1.5px solid var(--c, var(--accent));
  color: var(--c, var(--accent));
  background: #ffffff;
}

.tutorial-link:hover .tutorial-title {
  color: var(--c, var(--accent));
  text-decoration: underline;
  text-decoration-thickness: 2px;
  text-underline-offset: 3px;
}

@media (max-width: 720px) {
  .tutorial-card {
    grid-template-columns: 1fr;
  }

  .tutorial-summary,
  .tutorial-meta,
  .chapter-list {
    padding-left: 0;
  }

  .tutorial-thumb-link {
    max-width: 320px;
  }
}
</style>

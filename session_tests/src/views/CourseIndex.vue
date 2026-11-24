<template>
  <div class="course-index">
    <p class="eyebrow">ETH Zürich · Block Research Group · Session WebGPU</p>
    <h1>Viewer course</h1>
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
        <router-link :to="'/course/' + t.slug" class="tutorial-link">
          <span class="tutorial-dot">{{ t.number }}</span>
          <span class="tutorial-title">{{ t.title }}</span>
        </router-link>
        <ol v-if="t.chapters.length" class="chapter-list">
          <li v-for="c in t.chapters" :key="c.slug" class="chapter-item">
            <router-link :to="'/course/' + c.slug" class="tutorial-link chapter-link">
              <span class="chapter-pill">{{ c.number }}</span>
              <span class="tutorial-title">{{ c.title }}</span>
            </router-link>
          </li>
        </ol>
      </li>
    </ol>
  </div>
</template>

<script setup lang="ts">
import { checkpointCount, tutorials } from 'virtual:course';

const CAS_RAMP = ['#3057b1', '#5552bb', '#7750c1', '#934abd', '#ae4bb2', '#c54b9f', '#dc4b8d'];
const rampColor = (idx: number, total: number): string => {
  if (total <= 1) return CAS_RAMP[0];
  const pos = (idx / (total - 1)) * (CAS_RAMP.length - 1);
  return CAS_RAMP[Math.round(pos)] ?? CAS_RAMP[0];
};
</script>

<style scoped>
.course-index {
  max-width: 880px;
  padding: 2.2rem 0.5rem 3.5rem;
  line-height: 1.55;
}

.eyebrow {
  margin: 0 0 0.35rem;
  font-size: 13.5px;
  font-weight: 600;
  letter-spacing: 0.01em;
  color: var(--pink);
}

h1 {
  font-size: clamp(30px, 3.6vw, 42px);
  font-weight: 800;
  letter-spacing: -0.025em;
  line-height: 1.08;
  color: var(--fg);
  margin: 0 0 0.75rem;
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
  font-size: 16px;
  line-height: 1.6;
}

.lead code {
  font-size: 0.88em;
  background: var(--code-bg);
  padding: 0.1em 0.35em;
  border: 1px solid var(--rule);
}

.setup-box {
  margin: 0 0 1.85rem;
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

.tutorial-item {
  border-top: 1px solid var(--rule);
  padding: 0.52rem 0.35rem;
  transition: background 0.15s ease;
}

.tutorial-item:nth-child(even) {
  background: color-mix(in srgb, var(--wash) 45%, #ffffff);
}

.tutorial-item:last-child {
  border-bottom: 1px solid var(--rule);
}

.tutorial-item:hover {
  background: color-mix(in srgb, var(--c, var(--accent)) 7%, #ffffff);
}

.chapter-list {
  padding: 0.32rem 0 0.12rem 2.45rem;
  display: flex;
  flex-direction: column;
  gap: 0.22rem;
}

.chapter-item {
  padding: 0.1rem 0;
}

.tutorial-link {
  display: inline-flex;
  align-items: center;
  gap: 0.7rem;
  color: var(--fg);
  text-decoration: none;
  font-size: 16.5px;
  font-weight: 600;
  letter-spacing: -0.01em;
}

.chapter-link {
  font-size: 14.5px;
  font-weight: 600;
  color: var(--muted);
}

.chapter-link:hover {
  color: var(--fg);
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
</style>

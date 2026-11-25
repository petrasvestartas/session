<template>
  <div class="course-index">
    <div class="hero-top">
      <div>
        <h1>Viewer course</h1>
        <p class="subhead">17 engineering tutorials · 388 verified Rust &amp; WGSL files</p>
      </div>
      <a class="wood-btn main" :href="viewerHref">Viewer</a>
    </div>
    <p class="lead">
      Build the Session Viewer from scratch in Rust and WebGPU across {{ checkpointCount.toLocaleString('en-US') }} cumulative checkpoints, from the first cleared surface to every WGSL shader, 3D presentation stage and scene tool. Install Rust <code>1.97.1</code>, the browser target and Trunk <code>0.21.14</code>, then follow the 17 tutorials step by step:
    </p>
    <pre class="setup-code"><code>rustup toolchain install 1.97.1 --component rustfmt clippy
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14 --locked</code></pre>

    <ol class="tutorial-list">
      <li v-for="t in tutorials" :key="t.id" class="tutorial-item">
        <div class="tutorial-card">
          <div class="tutorial-body">
            <router-link :to="'/course/' + t.slug" class="tutorial-link">
              <span class="tutorial-num">{{ t.number }}.</span>
              <span class="tutorial-title">{{ t.title }}</span>
            </router-link>
            <p v-if="t.summary" class="tutorial-summary">{{ t.summary }}</p>
            <div v-if="t.fileCount" class="tutorial-meta">
              <span class="meta-files">{{ t.fileCount }} source files</span>
            </div>
            <ol v-if="t.chapters.length" class="chapter-list">
              <li v-for="c in t.chapters" :key="c.slug" class="chapter-item">
                <div class="chapter-row">
                  <router-link :to="'/course/' + c.slug" class="tutorial-link chapter-link">
                    <span class="chapter-num">{{ c.number }}</span>
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
</script>

<style scoped>
.course-index {
  max-width: 1080px;
  padding: 2rem 0.75rem 4rem;
  line-height: 1.62;
}

.hero-top {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 1.1rem;
}

h1 {
  font-size: clamp(30px, 3.2vw, 42px);
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1.12;
  color: var(--fg);
  margin: 0;
}

.subhead {
  margin: 0.35rem 0 0;
  font-size: 14px;
  font-weight: 400;
  color: var(--muted);
}

.wood-btn {
  display: inline-flex;
  align-items: center;
  padding: 10px 22px;
  border: 1px solid var(--rule);
  background: #ffffff;
  color: var(--fg);
  font-size: 15px;
  font-weight: 500;
  text-decoration: none;
  transition: border-color 0.14s ease, transform 0.14s ease, box-shadow 0.14s ease;
}

.wood-btn.main {
  border-color: var(--accent);
  background: var(--accent);
  color: #ffffff;
}

.wood-btn:hover {
  border-color: var(--ink);
  box-shadow: 4px 4px 0 var(--accent), 4px 4px 0 1.5px var(--ink);
  transform: translate(-3px, -3px);
}

.lead {
  color: var(--fg);
  margin: 0 0 1rem;
  max-width: 78ch;
  font-size: 15.5px;
}

.lead code {
  font-size: 0.9em;
  background: var(--code-bg);
  padding: 0.08em 0.3em;
}

.setup-code {
  margin: 0 0 2.2rem;
  padding: 16px 20px;
  border: 1px solid var(--rule);
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
  gap: 1px;
  background: var(--rule);
  border: 1px solid var(--rule);
}

.tutorial-item {
  background: #ffffff;
  padding: 1.1rem 1.25rem;
  transition: background 0.14s ease;
}

.tutorial-item:hover {
  background: #fafaf9;
}

.tutorial-card {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 224px;
  gap: 1.35rem;
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
  align-items: baseline;
  gap: 0.45rem;
  color: var(--fg);
  text-decoration: none;
  font-size: 17.5px;
  font-weight: 500;
  letter-spacing: -0.01em;
}

.tutorial-num {
  color: var(--muted);
  font-variant-numeric: tabular-nums;
  min-width: 1.5rem;
}

.tutorial-link:hover .tutorial-title {
  color: var(--accent);
  text-decoration: underline;
  text-decoration-color: var(--mark);
  text-decoration-thickness: 2px;
  text-underline-offset: 3px;
}

.tutorial-summary {
  margin: 0;
  padding-left: 1.95rem;
  font-size: 14px;
  line-height: 1.55;
  color: var(--muted);
}

.tutorial-meta {
  padding-left: 1.95rem;
  margin-top: 0.05rem;
}

.meta-files {
  font-size: 12.5px;
  color: var(--muted);
}

.tutorial-thumb-link {
  display: block;
  border: 1px solid var(--rule);
  background: #ffffff;
  overflow: hidden;
  aspect-ratio: 3 / 2;
  padding: 6px;
}

.tutorial-thumb {
  width: 100%;
  height: 100%;
  object-fit: contain;
  display: block;
}

.chapter-list {
  padding: 0.5rem 0 0.1rem 1.95rem;
  margin-top: 0.25rem;
  border-top: 1px solid var(--rule);
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.chapter-row {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 0.6rem;
}

.chapter-link {
  font-size: 14.5px;
  font-weight: 400;
}

.chapter-num {
  font-size: 13px;
  color: var(--muted);
  font-variant-numeric: tabular-nums;
  min-width: 1.65rem;
}

.chapter-files {
  font-size: 12px;
  color: var(--muted);
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

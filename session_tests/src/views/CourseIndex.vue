<template>
  <div class="course-index">
    <h1>Viewer course</h1>
    <p class="lead">
      Build the Session Viewer from scratch in Rust and WebGPU across {{ checkpointCount.toLocaleString('en-US') }} cumulative checkpoints, from the first cleared surface to every WGSL shader, presentation stage and scene tool. Install Rust <code>1.97.1</code>, the browser target and Trunk <code>0.21.14</code>, then follow the tutorials step by step:
    </p>
    <pre class="setup-code"><code>rustup toolchain install 1.97.1 --component rustfmt clippy
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14 --locked</code></pre>
    <ol class="tutorial-list">
      <li v-for="t in tutorials" :key="t.id" class="tutorial-item">
        <router-link :to="'/course/' + t.slug" class="tutorial-link">
          <span class="tutorial-num">{{ t.number }}.</span>
          <span class="tutorial-title">{{ t.title }}</span>
        </router-link>
        <ol v-if="t.chapters.length" class="chapter-list">
          <li v-for="c in t.chapters" :key="c.slug" class="chapter-item">
            <router-link :to="'/course/' + c.slug" class="tutorial-link chapter-link">
              <span class="tutorial-num">{{ c.number }}</span>
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
</script>

<style scoped>
.course-index {
  max-width: 720px;
  padding: 2rem 0 3rem;
  line-height: 1.55;
}

h1 {
  font-size: 26px;
  font-weight: 600;
  margin: 0 0 0.45rem;
}

.lead {
  color: var(--fg);
  margin: 0 0 0.75rem;
  font-size: 15px;
}

.lead code {
  font-size: 0.88em;
  background: var(--code-bg);
  padding: 0.08em 0.3em;
  border-radius: 3px;
}

.setup-code {
  margin: 0 0 1.5rem;
  padding: 0.7rem 0.95rem;
  background: var(--code-bg);
  border-radius: 4px;
  overflow-x: auto;
  font-size: 13px;
  line-height: 1.5;
  color: #1a1a1a;
}

.tutorial-list,
.chapter-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.tutorial-item {
  border-top: 1px solid var(--rule);
  padding: 0.42rem 0;
}

.tutorial-item:last-child {
  border-bottom: 1px solid var(--rule);
}

.chapter-list {
  padding: 0.15rem 0 0.15rem 1.85rem;
}

.chapter-item {
  padding: 0.16rem 0;
}

.tutorial-link {
  display: inline-flex;
  align-items: baseline;
  gap: 0.5rem;
  color: var(--fg);
  text-decoration: none;
  font-size: 15.5px;
  font-weight: 500;
}

.chapter-link {
  font-size: 14.5px;
  font-weight: 400;
}

.tutorial-num {
  font-variant-numeric: tabular-nums;
  color: #007bc4;
  font-weight: 600;
  min-width: 1.65rem;
}

.tutorial-link:hover .tutorial-title {
  text-decoration: underline;
}
</style>

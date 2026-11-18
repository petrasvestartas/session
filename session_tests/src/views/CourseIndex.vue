<template>
  <div class="course-index">
    <h1>Viewer course</h1>
    <p class="lead">
      Build the Session Viewer in Rust and wgpu across {{ checkpointCount.toLocaleString('en-US') }} executable subject checkpoints.
    </p>
    <p class="jump-row">
      <router-link class="start-link" to="/course/journey/completion-frame">Start Tutorial 1: Open a GPU frame →</router-link>
    </p>
    <p class="meta-row">
      <router-link to="/course/journey/active">Course map</router-link>
      · <router-link to="/course/journey/active#acceptance">Acceptance status</router-link>
      <template v-if="latestLesson"> · <router-link :to="'/course/' + latestLesson">Endpoint: {{ pages[latestLesson].title }}</router-link></template>
    </p>
    <details v-for="g in groups" :key="g.title" :open="g.title === 'Cumulative subjects'">
      <summary>{{ g.title }}</summary>
      <template v-if="g.sections">
        <details v-for="section in g.sections" :key="section.title" class="book-section">
          <summary>{{ section.title }}</summary>
          <ol><li v-for="s in section.slugs" :key="s"><router-link :to="'/course/' + s">{{ pages[s]?.title }}</router-link></li></ol>
        </details>
      </template>
      <ol v-else :class="{ numbered: g.title === 'Cumulative subjects' }">
        <li v-for="s in g.slugs" :key="s">
          <router-link :to="'/course/' + s">{{ pages[s]?.title }}</router-link>
        </li>
      </ol>
    </details>
  </div>
</template>

<script setup lang="ts">
import { checkpointCount, groups, pages } from 'virtual:course';
import { latestCodingLesson } from '../courseOrder';
const latestLesson = latestCodingLesson(pages);
</script>

<style scoped>
.course-index {
  max-width: 720px;
  padding: 2rem 0 3rem;
  line-height: 1.55;
}

.book-section { margin-left: 1rem; }
.book-section summary { margin-top: 0.7rem; border: 0; }

h1 {
  font-size: 26px;
  font-weight: 600;
  margin: 0 0 0.35rem;
}

summary {
  font-size: 15px;
  font-weight: 600;
  margin: 1.6rem 0 0.4rem;
  padding-bottom: 0.3rem;
  border-bottom: 1px solid var(--rule);
  cursor: pointer;
}

.lead {
  color: var(--muted);
  margin: 0 0 0.9rem;
}

.jump-row {
  margin: 0 0 0.55rem;
}

.start-link {
  display: inline-block;
  padding: 0.42rem 0.85rem;
  font-size: 14.5px;
  font-weight: 600;
  color: #ffffff;
  background: #111111;
  border-radius: 5px;
  text-decoration: none;
}

.start-link:hover {
  background: #2b2b33;
  text-decoration: none;
}

.meta-row {
  margin: 0;
  font-size: 13.5px;
  color: var(--muted);
}

.meta-row a {
  color: var(--muted);
}

ol {
  list-style: none;
  padding: 0;
  margin: 0;
}

ol.numbered {
  list-style: decimal;
  padding-left: 1.35rem;
}

ol.numbered li {
  padding: 0.2rem 0.15rem;
}

li {
  padding: 0.15rem 0;
}

a {
  color: var(--fg);
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}
</style>

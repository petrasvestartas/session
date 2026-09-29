<template>
  <div class="course-index">
    <h1>Viewer course</h1>
    <p class="lead">
      Build a small viewer you can explain, then grow toward the full viewer’s features and quality.
      Follow a working picture through geometry buffers, view state and drawing.
      The course is being extended; browser checks and the later feature lessons remain pending.
      The complete implementation reference stays available below.
    </p>
    <p><router-link to="/course/journey">Start with the course route</router-link></p>
    <details v-for="g in groups" :key="g.title" :open="g.slugs.includes('readme') || g.slugs.includes('journey/01-canvas')">
      <summary>{{ g.title }}</summary>
      <ol :class="{ plain: g.title !== 'Course' }">
        <li v-for="s in g.slugs" :key="s">
          <router-link :to="'/course/' + s">{{ pages[s]?.title }}</router-link>
        </li>
      </ol>
    </details>
  </div>
</template>

<script setup lang="ts">
import { groups, pages } from 'virtual:course';
</script>

<style scoped>
.course-index {
  max-width: 720px;
  padding: 2.5rem 0 3rem;
  line-height: 1.55;
}

h1 {
  font-size: 26px;
  font-weight: 600;
  margin: 0 0 0.4rem;
}

summary {
  font-size: 15px;
  font-weight: 600;
  margin: 2rem 0 0.4rem;
  padding-bottom: 0.3rem;
  border-bottom: 1px solid var(--rule);
  cursor: pointer;
}

.lead {
  color: var(--muted);
  margin: 0;
}

ol {
  list-style: none;
  padding: 0;
  margin: 0;
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

<template>
  <div class="course-index">
    <h1>Viewer course</h1>
    <p class="lead">
      Learn the existing Session Viewer's implementation, from GPU ownership to its developed rendering paths.
      {{ consolidatedChapterCount }} of {{ chapterGroups.length }} chapters are consolidated. The route currently has
      {{ readerLessonCount.toLocaleString('en-US') }} reader-facing lessons.
      <template v-if="consolidatedChapterCount < chapterGroups.length">Consolidation of the remaining chapters is pending.</template>
      Rendering reconciliation and runtime acceptance remain pending.
      The complete implementation reference stays available below.
    </p>
    <p><router-link to="/course/journey/contents">Book contents: chapters and sections</router-link></p>
    <p><router-link to="/course/journey/daily-plan">Daily coding plan</router-link></p>
    <p><router-link to="/course/journey/roadmap">To-do list and progress: finished and remaining lessons</router-link></p>
    <p v-if="latestLesson"><router-link :to="'/course/' + latestLesson">Latest coding checkpoint: {{ pages[latestLesson].title }}</router-link></p>
    <details v-for="g in groups" :key="g.title" :open="g.slugs.includes('readme') || g.slugs.includes('journey/consolidation/01-canvas')">
      <summary>{{ g.title }}</summary>
      <template v-if="g.sections">
        <details v-for="section in g.sections" :key="section.title" class="book-section">
          <summary>{{ section.title }}</summary>
          <ol><li v-for="s in section.slugs" :key="s"><router-link :to="'/course/' + s">{{ pages[s]?.title }}</router-link></li></ol>
        </details>
      </template>
      <ol v-else :class="{ plain: g.title !== 'Course' }">
        <li v-for="s in g.slugs" :key="s">
          <router-link :to="'/course/' + s">{{ pages[s]?.title }}</router-link>
        </li>
      </ol>
    </details>
  </div>
</template>

<script setup lang="ts">
import { groups, pages } from 'virtual:course';
import { latestCodingLesson } from '../courseOrder';
const latestLesson = latestCodingLesson(pages);
const chapterGroups = groups.filter(group => /^\d+ · /.test(group.title));
const consolidatedChapterCount = chapterGroups.filter(group => group.slugs.every(slug => /^\d+\.\d+ · /.test(pages[slug]?.title))).length;
const readerLessonCount = chapterGroups
  .reduce((count, group) => count + group.slugs.length, 0);
</script>

<style scoped>
.course-index {
  max-width: 720px;
  padding: 2.5rem 0 3rem;
  line-height: 1.55;
}

.book-section { margin-left: 1rem; }
.book-section summary { margin-top: 0.7rem; border: 0; }

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

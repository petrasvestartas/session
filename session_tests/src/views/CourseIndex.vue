<template>
  <div class="course-index">
    <h1>Viewer course</h1>
    <p class="lead">
      Each lesson adds one piece to a working viewer; its code lives in a lesson crate that builds on
      its own. Start at the top and keep the order.
    </p>
    <section v-for="g in groups" :key="g.title">
      <h2>{{ g.title }}</h2>
      <ol :class="{ plain: g.title !== 'Course' }">
        <li v-for="s in g.slugs" :key="s">
          <router-link :to="'/course/' + s">{{ pages[s]?.title }}</router-link>
        </li>
      </ol>
    </section>
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

h2 {
  font-size: 15px;
  font-weight: 600;
  margin: 2rem 0 0.4rem;
  padding-bottom: 0.3rem;
  border-bottom: 1px solid var(--rule);
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

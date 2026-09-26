<template>
  <div class="home">
    <h1>Session</h1>
    <p class="lead">A geometry kernel in C++, Python and Rust, and a WebGPU viewer built on it.</p>
    <ul class="entries">
      <li>
        <router-link to="/course">Viewer course</router-link>
        <span>Build the Rust, WebAssembly and WebGPU viewer from an empty project, one lesson at a time.</span>
      </li>
      <li>
        <router-link to="/tests">Kernel API</router-link>
        <span>Every kernel class with its tests side by side in C++, Python and Rust.</span>
      </li>
      <li>
        <router-link to="/install">Install</router-link>
        <span>Get the code and build each kernel and the viewer.</span>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.home {
  max-width: 640px;
  padding: 2.5rem 0 3rem;
  line-height: 1.55;
}

h1 {
  font-size: 26px;
  font-weight: 600;
  margin: 0 0 0.4rem;
}

.lead {
  color: var(--muted);
  margin: 0 0 2rem;
}

.entries {
  list-style: none;
  padding: 0;
  margin: 0;
}

.entries li {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
  padding: 0.9rem 0;
  border-top: 1px solid var(--rule);
}

.entries a {
  color: var(--fg);
  font-size: 17px;
  font-weight: 600;
  text-decoration: none;
}

.entries a:hover {
  text-decoration: underline;
}

.entries span {
  color: var(--muted);
}
</style>

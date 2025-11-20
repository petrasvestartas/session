import { createApp } from 'vue'
import router from './router.js'
import App from './App.vue'
import 'highlight.js/styles/atom-one-light.css'

const app = createApp(App)
app.use(router)
app.mount('#app')

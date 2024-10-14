import { createApp } from 'vue'
import { Quasar } from 'quasar'
import App from './App.vue'

import './style.css'
import '@quasar/extras/material-icons/material-icons.css' // Import icon libraries
import 'quasar/dist/quasar.css' // Import Quasar css

createApp(App)
  .use(Quasar, {})
  .mount('#app')

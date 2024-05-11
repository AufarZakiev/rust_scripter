import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { Quasar } from 'quasar'

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'

createApp(App).use(Quasar, {
    plugins: {}, // import Quasar plugins and add here
}).mount('#app')

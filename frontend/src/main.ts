import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import i18n from './i18n'
import './main.css'
import { useNetworkDetailStore } from './stores/networkDetail'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)
app.use(i18n)

app.mount('#app')

// Initialize network detail store event listener
const networkDetailStore = useNetworkDetailStore()
networkDetailStore.initEventListener()

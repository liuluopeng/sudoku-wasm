import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import {
    // create naive ui
    create,
    // component
    NButton,
    NCard,
    NMessageProvider,
    NModal,
    NPopover,
    useMessage
} from 'naive-ui'


// createApp(App).mount('#app')

const app = createApp(App)
app.mount('#app');

const naive = create({
    components: [NButton, NModal, NCard, useMessage, NMessageProvider, NPopover]
})

app.use(naive);



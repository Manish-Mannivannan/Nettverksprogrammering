import { createRouter, createWebHistory } from 'vue-router'
import CompilerView from '../views/CompilerView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Compiler',
      component: CompilerView
    }
  ]
})

export default router

import { createRouter, createWebHistory } from 'vue-router'
import MapView from '../views/MapView.vue'
import TestView from '../views/TestView.vue'
import AdminView from '../views/AdminView.vue'
import InfoView from '../views/InfoView.vue'
import LoginView from '../views/LoginView.vue'
import RegistrationView from '../views/RegistrationView.vue'
import VendorView from '../views/VendorView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'map',
      component: MapView
    },
    {
	path: '/test',
	name: 'test',
	component: TestView
    },
    {
	path: '/balderdash',
	name: '',
	component: AdminView
    },
    {
	path: '/vendors',
	name: '',
	component: InfoView
    },
    {
	path: '/login',
	name: '',
	component: LoginView
    },
    {
	path: '/register',
	name: '',
	component: RegistrationView
    },
    {
	path: '/manage',
	name: '',
	component: VendorView
    },
  ]
})

export default router

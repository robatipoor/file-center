import Vue from 'vue'
import App from './App.vue'
import Login from './components/Login.vue';
import Register from './components/Register.vue';
import ListUsers from './components/ListUsers.vue';
import ListFiles from './components/ListFiles.vue';

import VueAxios from 'vue-axios';
import axios from 'axios';
import VueRouter from 'vue-router'

Vue.config.productionTip = false
Vue.use(VueRouter)
Vue.use(VueAxios)
Vue.use(axios)

const routes = [
  { name: 'login', path: '/login', component: Login },
  { name: 'register', path: '/register', component: Register },
  { name: 'users', path: '/users', component: ListUsers },
  { name: 'files', path: '/files', component: ListFiles },
];

const router = new VueRouter({ mode: 'history', routes: routes });

new Vue({
  render: h => h(App),
  router,
}).$mount('#app')

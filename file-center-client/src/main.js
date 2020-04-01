import Vue from 'vue';
import App from './App.vue';
import VueRouter from 'vue-router';
import axios from 'axios';
import VueAxios from 'vue-axios';
import Login from './components/Login.vue';
import Register from './components/Register.vue';
import ListUsers from './components/ListUsers.vue';
import ListFiles from './components/ListFiles.vue';
import Ping from './components/Ping.vue';

Vue.config.productionTip = false
axios.defaults.baseURL = 'http://localhost:8081';
Vue.use(VueAxios, axios)
Vue.use(VueRouter)

const routes = [
  { name: 'login', path: '/login', component: Login },
  { name: 'register', path: '/register', component: Register },
  { name: 'users', path: '/users', component: ListUsers },
  { name: 'files', path: '/files', component: ListFiles },
  { name: 'ping', path: '/ping', component: Ping },
];

const router = new VueRouter({ mode: 'history', routes: routes });

new Vue({
  render: h => h(App),
  router,
}).$mount('#app')

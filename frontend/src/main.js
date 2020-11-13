import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import axios from "axios";

Vue.config.productionTip = false;

const context = require.context("./components", true, /\.vue$/);
const components = context.keys().map(context);

const Components = {};
Components.install = function install(Vue) {
  components.forEach(c => {
    Vue.component(c.default.name, c.default);
  });
};
Vue.use(Components);

{
  const context = require.context("../public/grills", true);
  const components = context.keys().map(context);

  const grills = [];
  components.forEach(c => {
    grills.push(c);
  });

  Vue.prototype.random_grill = () => {
    const index = Math.floor(Math.random() * (grills.length));
    return grills[index];
  };
}


const env = {
  API_URL: "",
  CDN_URL: "",
};

Vue.prototype.http = axios.create({
  baseURL: env.API_URL,
  timeout: 10000,
});

Vue.prototype.env = env;


new Vue({
  router,
  render: h => h(App),
}).$mount("#app");

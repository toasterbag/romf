import Vue from "vue";
import Router from "vue-router";

Vue.use(Router);

const router = new Router({
  mode: "history",
  base: process.env.BASE_URL,
  routes: [
    {
      path: "*",
      redirect: { name: "albums" }
    },
    {
      path: "/",
      name: "home",
      component: () => import("./views/home.vue"),
    },
  ]
});

export default router;

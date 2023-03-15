// Composables
import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Home",
        // route level code-splitting
        // this generates a separate chunk (about.[hash].js) for this route
        // which is lazy-loaded when the route is visited.
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/Home.vue"),
      },
      {
        path: "/class",
        name: "Class",
        component: () =>
          import(/* webpackChunkName: "class" */ "@/views/Class.vue"),
      },
      {
        path: "/classes",
        name: "My Classes",
        component: () =>
          import(/* webpackChunkName: "classes" */ "@/views/UserClasses.vue"),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;

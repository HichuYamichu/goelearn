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
        path: "/class/:classId",
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
      {
        path: "/explore",
        name: "Explore classes",
        component: () =>
          import(
            /* webpackChunkName: "explore" */ "@/views/ExploreClasses.vue"
          ),
      },
      {
        path: "/login",
        name: "Login",
        component: () =>
          import(/* webpackChunkName: "login" */ "@/views/Login.vue"),
      },
      {
        path: "/register",
        name: "Register",
        component: () =>
          import(/* webpackChunkName: "register" */ "@/views/Register.vue"),
      },
      {
        path: "/calendar",
        name: "Calendar",
        component: () =>
          import(/* webpackChunkName: "calendar" */ "@/views/Register.vue"),
      },
      {
        path: "/assignments",
        name: "Assignments",
        component: () =>
          import(/* webpackChunkName: "assignments" */ "@/views/Register.vue"),
      },
      {
        path: "/class-create",
        name: "Create Class",
        component: () =>
          import(
            /* webpackChunkName: "class-create" */ "@/views/ClassCreate.vue"
          ),
      },
      {
        path: "/settings",
        name: "Settings",
        component: () =>
          import(/* webpackChunkName: "settings" */ "@/views/UserSettings.vue"),
      },
      {
        path: "/test",
        name: "Test",
        component: () =>
          import(/* webpackChunkName: "test" */ "@/views/Test.vue"),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;

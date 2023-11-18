import { createRouter, createWebHistory, RouteLocation } from "vue-router";
import { graphql } from "@/gql";
import { useLazyQuery } from "@vue/apollo-composable";
import { provideApolloClient } from "@vue/apollo-composable";
import { client } from "../client";

const IsLoggedIn = graphql(/* GraphQL */ `
  query IsLoggedIn {
    isLoggedIn @client
  }
`);

const isLoggedIn = async (to: RouteLocation, from: RouteLocation) => {
  const res = client.readQuery({
    query: IsLoggedIn,
  });
  return res?.isLoggedIn;
};

const ClassOwner = graphql(/* GraphQL */ `
  query routerClassById($id: ID!) {
    classById(id: $id) {
      id
      owner {
        id
      }
    }
  }
`);

const Me = graphql(/* GraphQL */ `
  query routerMe {
    me {
      id
      userType
    }
  }
`);

const isAdmin = async (to: RouteLocation, from: RouteLocation) => {
  const meResult = await client.query({
    query: Me,
  });

  return meResult?.data.me.userType === "ADMIN";
};

const routes = [
  {
    path: "/",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Home",
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/Home.vue"),
      },
      {
        path: "/class/:classId",
        name: "Class",
        beforeEnter: isLoggedIn,
        component: () =>
          import(/* webpackChunkName: "class" */ "@/views/Class.vue"),
      },
      {
        path: "/invite/:inviteId",
        name: "Invite",
        beforeEnter: isLoggedIn,
        component: () =>
          import(/* webpackChunkName: "invite" */ "@/views/Invite.vue"),
      },
      {
        path: "/classes",
        name: "My Classes",
        beforeEnter: isLoggedIn,
        component: () =>
          import(/* webpackChunkName: "classes" */ "@/views/UserClasses.vue"),
      },
      {
        path: "/explore",
        name: "Explore classes",
        beforeEnter: isLoggedIn,
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
        path: "/assignments",
        name: "Assignments",
        beforeEnter: isLoggedIn,
        component: () =>
          import(
            /* webpackChunkName: "assignments" */ "@/views/UserAssignments.vue"
          ),
      },
      {
        path: "/class-create",
        name: "Create Class",
        beforeEnter: isLoggedIn,
        component: () =>
          import(
            /* webpackChunkName: "class-create" */ "@/views/ClassCreate.vue"
          ),
      },
      {
        path: "/settings",
        name: "Settings",
        beforeEnter: isLoggedIn,
        component: () =>
          import(/* webpackChunkName: "settings" */ "@/views/UserSettings.vue"),
      },
      {
        path: "/admin",
        name: "Admin",
        beforeEnter: [isLoggedIn, isAdmin],
        component: () =>
          import(/* webpackChunkName: "settings" */ "@/views/Admin.vue"),
      },
      {
        path: "/password-reset/:token",
        name: "Reset Password",
        component: () =>
          import(
            /* webpackChunkName: "change-password" */ "@/views/ChangePassword.vue"
          ),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;

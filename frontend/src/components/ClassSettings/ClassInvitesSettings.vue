<template></template>

<script lang="ts" setup>
import { computed, ref, toRef, watch } from "vue";
import { useMutation, useQuery, useSubscription } from "@vue/apollo-composable";
import { useRoute, useRouter } from "vue-router";
import { reactive } from "vue";
import { graphql } from "@/gql";

const router = useRouter();
const classId = router.currentRoute.value.params.classId as string;
</script>

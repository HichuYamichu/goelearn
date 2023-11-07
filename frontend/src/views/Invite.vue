<template>
    <div class="d-flex justify-center align-center flex-wrap w-100 pa-16 ">
            <h2 class="text-h2 text-center w-100 ">You are about to join: {{ _class.name }}</h2>
            <p class="text-subtitle-1 text-center w-50 mt-4">{{ _class.description }}</p>
            <div class="d-flex w-100 pa-4">
                <div class="center d-flex gap">
                    <v-btn class="bg-success center" @click="joinClass">Confirm</v-btn>
                    <v-btn class="bg-error" @click="cancel">Cancel</v-btn>
                </div>
            </div>
    </div>
</template>

<script lang="ts" setup>
import { useRoute, useRouter } from "vue-router";

const router = useRouter();
const route = useRoute();
const inviteId = route.params.inviteId as string;

const _class = {
    id: "dfad",
    name: "aa",
    description: 'lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum '
}

const joinClass = () => {

}

const cancel = () => {

}

</script>

<style scoped>
.main-wrapper {
  width: 60%;
}


.center {
    margin: auto;
}

.gap {
    gap: 15px;
}
</style>
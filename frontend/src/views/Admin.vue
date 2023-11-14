<template>
    <v-card>
        <v-tabs v-model="tab" bg-color="primary">
            <v-tab value="one">Item One</v-tab>
            <v-tab value="two">Item Two</v-tab>
        </v-tabs>

        <v-card-text>
            <v-window v-model="tab">
                <v-window-item value="one">
                    <div class="pa-2">
                        <v-text-field label="Search user" variant="outlined"></v-text-field>
                        <v-list>
                            <v-list-item v-for="user in users">
                                <v-list-item-title> {{ user.username }}</v-list-item-title>
                                <v-checkbox label="Allow creating classes"></v-checkbox>
                                <v-checkbox label="Disable account"></v-checkbox>
                                <v-btn>Save</v-btn>
                            </v-list-item>
                        </v-list>
                    </div>
                </v-window-item>
                <v-window-item value="two">
                    <div class="pa-2">
                        <v-list>
                            <v-text-field label="Search class" variant="outlined"></v-text-field>
                            <v-list-item v-for="class_ in classes">
                                <v-list-item-title> {{ class_.name }}</v-list-item-title>
                                <v-checkbox label="Disable class"></v-checkbox>
                                <v-btn>Save</v-btn>
                            </v-list-item>
                        </v-list>
                    </div>
                </v-window-item>
            </v-window>
        </v-card-text>
    </v-card>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

const tab = ref('');

const users = [
    { username: 'aaaa' }
]

const classes = [
    { name: "bbbb" }
]

</script>
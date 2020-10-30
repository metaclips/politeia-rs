<template>
  <v-row class="mx-10" no-gutters>
    <v-col class="mb-5" cols="12">
      <h1>
        Public Proposals
      </h1>
    </v-col>

    <v-col cols="12">
      <v-tabs grow mobile-breakpoint="1">
        <v-tab>Item One</v-tab>
        <v-tab>Item Two</v-tab>
        <v-tab>Item Three</v-tab>
        <v-tab>Item Three</v-tab>
        <v-tab>Item Three</v-tab>
      </v-tabs>
    </v-col>
  </v-row>
</template>
<script lang="ts">
import { Component, Vue } from "vue-property-decorator";

@Component
export default class HomeComponents extends Vue {}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss"></style>

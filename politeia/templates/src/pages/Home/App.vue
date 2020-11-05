<template>
  <v-app>
    <v-app-bar app flat>
      <v-spacer></v-spacer>
      <div class="d-flex align-center">
        <v-img
          alt="Politeia Logo"
          :src="require('../../assets/logo.svg')"
          transition="scale-transition"
          contain
        />
      </div>
      <v-spacer></v-spacer>
    </v-app-bar>

    <v-main>
      <v-container fluid>
        <HomeComponents
          id="scroll-target"
          style="max-height: 90vh"
          class="overflow-y-auto"
        />
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import HomeComponents from "./components/HomeComponents.vue";

@Component({
  components: {
    HomeComponents
  }
})
export default class App extends Vue {}
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  margin-top: 20px;
}
html {
  overflow-y: hidden;
}
</style>

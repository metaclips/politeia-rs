<template>
  <v-row class="mx-10" no-gutters>
    <v-col class="mb-10" cols="12">
      <h5 class="header">
        Public Proposals
      </h5>
    </v-col>

    <v-col v-if="Object.keys(proposals).length" cols="8">
      <v-tabs v-model="tab" align-with-title grow mobile-breakpoint="3">
        <v-tab>In Discussion [{{ proposals["pre"].length }}]</v-tab>
        <v-tab>Voting [{{ proposals["active"].length }}]</v-tab>
        <v-tab>Approved [{{ proposals["approved"].length }}]</v-tab>
        <v-tab>Rejected [{{ proposals["rejected"].length }}]</v-tab>
        <v-tab>Abandoned [{{ proposals["abandoned"].length }}]</v-tab>
      </v-tabs>

      <v-tabs-items v-model="tab">
        <v-tab-item v-for="(proposalType, index) in proposalTypes" :key="index">
          <div
            class="align-center my-10"
            align="center"
            justify="center"
            v-if="proposals[proposalType].length == 0"
          >
            <span>No proposal under discussion</span>
          </div>

          <div v-else></div>
        </v-tab-item>
      </v-tabs-items>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { Component, Vue, Watch } from "vue-property-decorator";
import Axios from "axios";

@Component
export default class HomeComponents extends Vue {
  private proposals: Record<string, []> = {};
  private proposalTypes: Record<number, string> = {
    0: "pre",
    1: "active",
    2: "approved",
    3: "rejected",
    4: "abandoned"
  };

  private tab = 0;

  @Watch("tab")
  fetchTokens() {
    console.log("Tab is at", this.tab);

    console.log(
      "Fetching tokens for ",
      this.proposalTypes[this.tab],
      "\n",
      this.proposals[this.proposalTypes[this.tab]]
    );
  }

  mounted() {
    Axios.get(process.env.VUE_APP_BACKEND_SERVER + "/api/v1/fetchtokens")
      .then(Response => {
        if (Response.status == 200) {
          this.proposals = Response.data;
        }
      })
      .catch(errror => {
        console.log("Unable to fetch API, error: ", errror);
        // ToDo: Create a dialog that shows error.
      });
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
.header {
  font-size: xx-large;
  font-weight: 500;
}
</style>

<template>
  <v-row no-gutters class="mx-4">
    <v-col class="mb-10" cols="12">
      <h5 class="header">
        Public Proposals
      </h5>
    </v-col>

    <v-col
      v-if="Object.keys(proposalTokens).length"
      :cols="$vuetify.breakpoint.smAndDown ? '12' : '12'"
    >
      <v-tabs v-model="tab" align-with-title grow show-arrows>
        <v-tab>In Discussion [{{ proposalTokens["pre"].length }}]</v-tab>
        <v-tab>Voting [{{ proposalTokens["active"].length }}]</v-tab>
        <v-tab>Approved [{{ proposalTokens["approved"].length }}]</v-tab>
        <v-tab>Rejected [{{ proposalTokens["rejected"].length }}]</v-tab>
        <v-tab>Abandoned [{{ proposalTokens["abandoned"].length }}]</v-tab>
      </v-tabs>

      <v-tabs-items v-model="tab">
        <v-tab-item v-for="(proposalType, index) in proposalTypes" :key="index">
          <v-row
            class="align-center my-10"
            align="center"
            justify="center"
            v-if="proposalTokens[proposalType].length == 0"
          >
            <span>No proposal under discussion</span>
          </v-row>

          <!-- Check if there are proposal to show for current tab. -->
          <v-row
            v-else-if="proposals[proposalType]"
            class="overflow-y-auto mx-8"
          >
            <v-col
              v-for="(proposal, propsalIndex) in proposals[proposalType]"
              :key="propsalIndex"
            >
              <v-card flat outlined tile>
                <v-card-title class="my-1" href="https://github.com/metaclips">
                  {{ proposal.name }}
                </v-card-title>

                <v-card-subtitle>
                  <a class="mr-4" href="https://github.com/metaclips">{{
                    proposal.username
                  }}</a>

                  <span v-if="!$vuetify.breakpoint.smAndDown" class="mr-4"
                    >published about {{ getDate(proposal.publishedat) }}</span
                  >

                  <span v-if="!$vuetify.breakpoint.smAndDown" class="mr-4"
                    >edited about {{ getDate(proposal.timestamp) }}</span
                  >
                </v-card-subtitle>
              </v-card>
            </v-col>
          </v-row>

          <v-row class="mx-8" v-else>
            <v-col
              v-for="index in proposalTokens[proposalType].length"
              :key="index"
              cols="12"
            >
              <v-sheet color="transparent" class="pa-3">
                <v-card tile flat outlined>
                  <v-skeleton-loader type="article"></v-skeleton-loader>
                </v-card>
              </v-sheet>
            </v-col>
          </v-row>
        </v-tab-item>
      </v-tabs-items>
    </v-col>

    <v-col v-else :cols="$vuetify.breakpoint.smAndDown ? '12' : '8'">
      <v-sheet color="transparent" class="pa-3">
        <v-skeleton-loader type="article"></v-skeleton-loader>
      </v-sheet>
    </v-col>
  </v-row>
</template>

<script lang="ts">
"use strict";

import { Component, Vue, Watch } from "vue-property-decorator";
import Axios from "axios";

@Component
export default class HomeComponents extends Vue {
  private tab = 0;
  private proposalTokens: Record<string, []> = {};
  private proposals: Record<string, []> = {};
  private proposalTypes: Record<number, string> = {
    0: "pre",
    1: "active",
    2: "approved",
    3: "rejected",
    4: "abandoned"
  };

  public getDate(timestamp: number): string {
    const date = new Date(timestamp * 1000);

    const daysDifference = Math.floor(
      (new Date().getTime() - timestamp) / 1000 / 60 / 60 / 24
    );

    console.log("Days diff", daysDifference, timestamp, new Date().getTime());

    if (daysDifference < 31) {
      return daysDifference > 1
        ? daysDifference + " days ago"
        : daysDifference + " day ago";
    }

    const years = date.getFullYear();
    const months = date.getUTCMonth() + 1;

    const yearElapsed = years - new Date().getFullYear();
    const monthElapsed = new Date().getUTCMonth() + 1 - months;

    if (yearElapsed == 0) {
      return monthElapsed > 1
        ? monthElapsed + " months ago"
        : monthElapsed + " month ago";
    }

    return yearElapsed > 1
      ? yearElapsed + " years ago"
      : yearElapsed + " year ago";
  }

  @Watch("tab")
  fetchProposalFromToken() {
    console.log("Tab is at", this.tab);
    console.log(
      "Fetching tokens for ",
      this.proposalTypes[this.tab],
      "\n",
      this.proposalTokens[this.proposalTypes[this.tab]]
    );

    const proposalType = this.proposalTypes[this.tab];

    if (
      this.proposals[proposalType] == undefined &&
      this.proposalTokens[proposalType].length > 0
    ) {
      const tokensValue = this.proposalTokens[proposalType];
      // Fetch tokens
      Axios.post(
        process.env.VUE_APP_BACKEND_SERVER + "/api/v1/fetchproposals",
        JSON.stringify({ tokens: tokensValue }),
        {
          headers: {
            // Overwrite Axios's automatically set Content-Type
            "Content-Type": "application/json"
          }
        }
      )
        .then(Response => {
          if (Response.status == 200) {
            this.proposals[proposalType] = Response.data.proposals;
            console.log("proposal", proposalType, this.proposals[proposalType]);
            this.$forceUpdate();
          }
        })
        .catch(error => {
          console.log("Error fetching proposals", error);
        });
    }
  }

  mounted() {
    console.log("Sending fetch tokens");
    Axios.get(process.env.VUE_APP_BACKEND_SERVER + "/api/v1/fetchtokens")
      .then(Response => {
        if (Response.status == 200) {
          this.proposalTokens = Response.data;
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

a {
  text-decoration: none;
}
</style>

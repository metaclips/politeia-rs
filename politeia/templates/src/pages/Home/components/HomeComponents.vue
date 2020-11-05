<template>
  <v-row class="mx-4" no-gutters>
    <v-col class="mb-10" cols="12">
      <h5 class="header">
        Public Proposals
      </h5>
    </v-col>

    <v-col
      v-if="Object.keys(proposalTokens).length"
      :cols="$vuetify.breakpoint.smAndDown ? '12' : '10'"
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
          <v-container v-else>
            <v-row v-scroll:#scroll-target="onScroll" class="mx-8">
              <v-col
                v-for="(proposal, proposalIndex) in proposals[proposalType]"
                :key="proposalIndex"
                cols="12"
              >
                <!-- Proposal card -->
                <v-card max-width="90vh" flat outlined tile>
                  <div class="mx-4 my-4">
                    <v-card-title href="https://github.com/metaclips">
                      {{ proposal.name }}
                    </v-card-title>

                    <v-card-subtitle>
                      <a class="mr-4" href="https://github.com/metaclips">{{
                        proposal.username
                      }}</a>

                      <v-tooltip bottom>
                        <template v-slot:activator="{ on, attrs }">
                          <span
                            v-if="!$vuetify.breakpoint.smAndDown"
                            v-bind="attrs"
                            v-on="on"
                            class="mr-4"
                            >published about
                            {{
                              moment.unix(proposal.publishedat).fromNow()
                            }}</span
                          >
                        </template>
                        <span>{{ new Date(proposal.publishedat * 1000) }}</span>
                      </v-tooltip>

                      <v-tooltip
                        v-if="proposal.timestamp != proposal.publishedat"
                        bottom
                      >
                        <template v-slot:activator="{ on, attrs }">
                          <span
                            v-if="!$vuetify.breakpoint.smAndDown"
                            v-bind="attrs"
                            v-on="on"
                            class="mr-4"
                            >edited about
                            {{
                              moment.unix(proposal.timestamp).fromNow()
                            }}</span
                          >
                        </template>
                        <span>{{ new Date(proposal.timestamp * 1000) }}</span>
                      </v-tooltip>

                      <span
                        v-if="
                          !$vuetify.breakpoint.smAndDown && proposal.version > 1
                        "
                        class="mr-4"
                        >version {{ proposal.version }}</span
                      >
                    </v-card-subtitle>

                    <v-card-actions>
                      <template v-if="proposal.numcomments > 1">
                        {{ proposal.numcomments }} comments
                      </template>

                      <template v-else
                        >{{ proposal.numcomments }} comment</template
                      >
                    </v-card-actions>
                  </div>
                </v-card>
              </v-col>

              <template v-if="proposalIsFetching">
                <v-col v-for="n in proposalToFetch" :key="n + 'aaa'" cols="12">
                  <!-- Skeleton card, more like a waiter/loader. n+aaa is an hack. -->
                  <v-sheet color="transparent" class="pa-3">
                    <v-card tile flat outlined>
                      <v-skeleton-loader type="article"></v-skeleton-loader>
                    </v-card>
                  </v-sheet>
                </v-col>
              </template>
            </v-row>
          </v-container>
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
import { Component, Vue, Watch } from "vue-property-decorator";
import { Proposal } from "../../../Types";
import Axios from "axios";

@Component
export default class HomeComponents extends Vue {
  private tab = 0;
  private maxFetchTokens = 4;
  private proposalToFetch = 0;
  private proposalIsFetching = false;
  private proposalTokens: Record<string, []> = {};
  private proposals: Record<string, Proposal[]> = {};
  private proposalTypes: Record<number, string> = {
    0: "pre",
    1: "active",
    2: "approved",
    3: "rejected",
    4: "abandoned"
  };

  public onScroll(scrollElement: { target: Element }) {
    if (
      scrollElement.target.scrollTop + scrollElement.target.clientHeight >
        scrollElement.target.scrollHeight - 30 &&
      !this.proposalIsFetching
    ) {
      console.log("Fetching");
      this.fetchProposals();
    }
  }

  @Watch("tab")
  public fetchProposals() {
    const proposalType = this.proposalTypes[this.tab];
    let fetchedProposals = 0;
    if (this.proposals[proposalType]) {
      fetchedProposals = this.proposals[proposalType].length;
    }

    const tokensValue = this.proposalTokens[proposalType].slice(
      fetchedProposals,
      fetchedProposals + this.maxFetchTokens
    );

    // return if all proposals are fetched
    if (tokensValue.length == 0) {
      console.log("Nothing to fetch here");
      return;
    }

    this.proposalToFetch = tokensValue.length;

    this.proposalIsFetching = true;

    Axios.post(
      process.env.VUE_APP_BACKEND_SERVER + "/api/v1/fetchproposals",
      JSON.stringify({ tokens: tokensValue }),
      {
        headers: {
          "Content-Type": "application/json"
        }
      }
    )
      .then(Response => {
        if (Response.status == 200) {
          if (this.proposals[proposalType] == undefined) {
            this.proposals[proposalType] = [];
          }

          const proposals: [Proposal] = Response.data.proposals;
          for (const proposal of proposals) {
            this.proposals[proposalType].push(proposal);
          }

          this.proposalIsFetching = false;
        }
      })
      .catch(error => {
        console.log("Error fetching proposals", error);
      });
  }

  mounted() {
    console.log("Sending fetch tokens");
    Axios.get(process.env.VUE_APP_BACKEND_SERVER + "/api/v1/fetchtokens")
      .then(Response => {
        if (Response.status == 200) {
          this.proposalTokens = Response.data;
          this.fetchProposals();
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

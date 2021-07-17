<script lang="ts">
  import ValidatorModal from "./ValidatorModal.svelte";
  export let data;

  const modal_id = "vals-tab-val-modal";

  interface ValInfo {
    account_address: string;
    pub_key: string;
    voting_power: Number;
    full_node_ip: string;
    validator_ip: string;
    tower_height: Number;
    tower_epoch: Number;
    count_proofs_in_epoch: Number;
    epochs_validating_and_mining: Number;
    contiguous_epochs_validating_and_mining: Number;
    epochs_since_last_account_creation: Number;
    vote_count_in_epoch: Number;
    prop_count_in_epoch: Number;
  }
  
  let set: ValInfo[] = [];
  let selectedVal: ValInfo = null;
  let sortableColumns = [
    { label: "voting power", sortKey: "voting_power" },
    { label: "proofs in epoch", sortKey: "count_proofs_in_epoch" },
    { label: "tower height", sortKey: "tower_height" },
    { label: "votes in epoch", sortKey: "vote_count_in_epoch" },
    { label: "props in epoch", sortKey: "prop_count_in_epoch" },
  ];
  let sortOption: string = "voting_power";
  let sortOrder = 1;

  $: if (data.chain_view && data.chain_view.validator_view) {
    set = data.chain_view.validator_view;
    selectedVal = set[0];
  }
  $: set = set.sort((a, b) => (a[sortOption] > b[sortOption]) ? sortOrder : -sortOrder);
  
  function thOnClick(key: string) {
    if (sortOption == key) {
      sortOrder = -sortOrder;
    }
    sortOption = key;
  }
</script>

<style>
  .owner {
    background: #E6E6E6;
  }
</style>

<main uk-height-viewport="expand: true">
  <h2 class="uk-text-center uk-text-uppercase uk-text-muted uk-text-light uk-margin-medium-bottom">
    <span>{set.length} Validators</span>
  </h2>
 
  <div class="uk-overflow-auto">
    <table class="uk-table uk-table-hover uk-text-muted">
      <thead>
        <tr>
            <th class="uk-text-center">account</th>
            {#each sortableColumns as col}
              <th class="uk-text-right" on:click={() => thOnClick(col.sortKey)}>
                <span class="disable-select">{col.label}</span>
                {#if sortOption == col.sortKey}
                  {#if sortOrder == 1}
                    <span uk-icon="icon: triangle-up"></span>
                  {:else}
                    <span uk-icon="icon: triangle-down"></span>
                  {/if}
                {/if}
              </th>
            {/each}
            <th></th>
        </tr>
      </thead>
      <tbody>
        {#each set as val, i}
        <tr class="{val.account_address === data.account_view.address ? 'owner' : ''}" on:click={() => selectedVal = val}>        
            <td class="uk-visible@s uk-text-center">{val.account_address}</td>
            <td class="uk-hidden@s uk-text-truncate">{val.account_address}</td>
            <td class="uk-text-right">{val.voting_power}</td>
            <td class="uk-text-right">{val.count_proofs_in_epoch}</td>
            <td class="uk-text-right">{val.tower_height}</td>
            <td class="uk-text-right">{val.vote_count_in_epoch}</td>
            <td class="uk-text-right">{val.prop_count_in_epoch}</td>
            <td>
              <span uk-icon="icon: info" uk-toggle="target: #{modal_id}"></span>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <ValidatorModal validator={selectedVal} id={modal_id}></ValidatorModal>
</main>

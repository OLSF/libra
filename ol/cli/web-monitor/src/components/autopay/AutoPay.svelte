<script lang="ts">
  import AutoPaySummary from "./AutoPaySummary.svelte";
  import "/layout/Style.svelte";
  export let account;

  let total;
  $: if (account && account.autopay) {
    total = account.autopay.payments.reduce((a, b) => a + (b.amt || 0), 0);
  }
</script>

<div>
  <h2 class="uk-text-center uk-text-uppercase uk-text-muted uk-text-light uk-margin-medium-bottom">
    Autopay Instructions
  </h2>
  
  {#if account}
    {#if account.autopay && account.autopay.payments.length > 0}
      <AutoPaySummary payments={account.autopay.payments}/>
      <div class="uk-overflow-auto">
        <table class="uk-table uk-table-hover">
          <thead>
            <tr>
              <th class="uk-text-center">uid</th>
              <th class="uk-text-center">note</th>
              <th class="uk-text-center">payee</th>
              <th class="uk-text-center">type</th>
              <th class="uk-text-center">end epoch</th>
              <th class="uk-text-center">amount</th>
            </tr>
          </thead>
          <tbody>
            {#each account.autopay.payments as {uid, note, type_desc, payee, end_epoch, amount}}
              <tr>
                <td class="uk-text-center">{uid}</td>
                <td class="uk-text-center">{note || ""}</td>
                <td class="uk-visible@s uk-text-center">{payee}</td>
                <td class="uk-hidden@s uk-text-truncate">{payee}</td>
                <td class="uk-text-center">{type_desc}</td>
                <td class="uk-text-right">{end_epoch}</td>
                <td class="uk-text-right">{amount}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <p class="uk-text-center uk-text-warning">Your validator does not have autopay instructions.</p>
    {/if}
  {:else}
    <p>loading...</p>
  {/if}
</div>
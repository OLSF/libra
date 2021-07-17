<script lang="ts">
  import "../../layout/Style.svelte";
  
  export let account;

  function formatBalance(balance) {
    return balance.toLocaleString('en-ES', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    });
  }
</script>

<div class="uk-card uk-card-default uk-card-body uk-margin-bottom">
  <h3 class="uk-card-title uk-text-center uk-text-uppercase uk-text-muted uk-text-large">
    Accounts
  </h3>

  {#if account}
    <table class="uk-table">
      <thead>
        <tr>
          <th>Type</th>
          <th>Address</th>
          <th>Balance</th>
        </tr>
      <tbody>
        <tr>
          <td class="uk-text-uppercase">validator</td>
          <td class="uk-text-truncate">{account.address}</td>
          <td class="uk-text-right">{formatBalance(account.balance)}</td>
        </tr>
        {#if account.operator_account != null}
          <tr>
            <td class="uk-text-uppercase">operator</td>
            <td class="uk-text-truncate">{account.operator_account}</td>
            <td class="uk-text-right">{formatBalance(account.operator_balance)}</td>
          </tr>
        {/if}
      </tbody>
    </table>
  {:else}
    <p>loading...</p>
  {/if}
</div>

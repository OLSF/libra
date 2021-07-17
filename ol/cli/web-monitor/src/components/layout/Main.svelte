<script>
  import Nav from "./Nav.svelte";
  import Dash from "../monitor/Dash.svelte";
  import Vals from "../validators/Vals.svelte";
  import Upgrade from "../upgrade/Upgrade.svelte";
  import AutoPay from "../autopay/AutoPay.svelte";
  import WatchList from "../watch-list/WatchList.svelte";
  import AuditVals from "../audit/AuditVals.svelte";
  import { onDestroy } from 'svelte';
  import { chainInfo } from "../../store.ts";

  let data;
  
  const unsubscribe = chainInfo.subscribe((info_str) => {
    data = JSON.parse(info_str);
  });
  
  onDestroy(unsubscribe);
</script>

<main uk-height-viewport="expand: true" class="uk-background-muted uk-overflow-auto">
  <Nav />
  <div class="uk-container uk-margin-top">
    <ul class="uk-switcher uk-margin switcher-container uk-height-large">
      <Dash data={data}/>
      <Vals data={data}/>
      <AutoPay account={data.account_view}/>
      <WatchList data={data}/>
      <AuditVals data={data}/>      
      <Upgrade data={data}/>
    </ul>
  </div>
</main>

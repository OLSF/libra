<script>
    export let data;
   
    let sortableColumns = [
        { label: "note", sortKey: "note" },
        { label: "address", sortKey: "address" },
        { label: "#payers", sortKey: "payers" },
        { label: "average %", sortKey: "average_percent" },
        { label: "balance", sortKey: "balance" },
        { label: "sum %", sortKey: "sum_percentage" },
        { label: "% of all", sortKey: "all_percentage" },
    ];
    let sortOption = "payers";
    let sortOrder = -1;
    
    let total_balance, total_sum_percentage, total_percentage;

    let watch_list = null;
    $: if (data && data.chain_view && data.chain_view.autopay_watch_list) {
        watch_list = data.chain_view.autopay_watch_list.sort((a, b) => (a[sortOption] > b[sortOption]) ? sortOrder : -sortOrder);        
        
        // update totals
        total_balance = 0;
        total_percentage = 0;
        total_sum_percentage = 0;
        watch_list.forEach(stat => {
            total_balance += stat.balance;
            total_percentage += stat.all_percentage;
            total_sum_percentage += stat.sum_percentage;
        });
    }
   
    function thOnClick(key) {
        if (sortOption == key) {
            sortOrder = -sortOrder;
        }
        sortOption = key;
    }

    function formatBalance(balance) {
        return balance.toLocaleString('en-ES', {
            minimumFractionDigits: 0,
            maximumFractionDigits: 0
        });
    }

    function formatPercent(num) {
        return (num / 100).toFixed(2) + "%";
    }
</script>

<div>
    <h2 class="uk-text-center uk-text-uppercase uk-text-muted uk-text-light uk-margin-medium-bottom">
        Autopay Watch List
    </h2>
    {#if watch_list}
        <div class="uk-overflow-auto">
            <table class="uk-table uk-table-hover">
                <thead>
                    <tr>
                        {#each sortableColumns as col}
                            <th class="uk-text-center" on:click={() => thOnClick(col.sortKey)}>
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
                    </tr>
                </thead>
                <tbody>
                    {#each watch_list as {note, address, balance, payers, average_percent, sum_percentage, all_percentage}}
                        <tr>
                            <td class="uk-visible@s uk-text-center">{note}</td>
                            <td class="uk-hidden@s uk-text-truncate">{note}</td>
                            <td class="uk-visible@s uk-text-center">{address}</td>
                            <td class="uk-hidden@s uk-text-truncate">{address}</td>
                            <td class="uk-text-right">{payers}</td>
                            <td class="uk-text-right">{formatPercent(average_percent)}</td>
                            <td class="uk-text-right">{formatBalance(balance)}</td>
                            <td class="uk-text-right">{formatPercent(sum_percentage)}</td>
                            <td class="uk-text-right">{formatPercent(all_percentage)}</td>
                        </tr>
                    {/each}
                    <tr>
                        <td class="uk-text-center uk-text-bold">TOTAL</td>
                        <td></td>
                        <td></td>
                        <td></td>
                        <td class="uk-text-right uk-text-bold">{formatBalance(total_balance)}</td>
                        <td class="uk-text-right uk-text-bold">{formatPercent(total_sum_percentage)}</td>
                        <td class="uk-text-right uk-text-bold">{formatPercent(total_percentage)}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    {/if}
</div>
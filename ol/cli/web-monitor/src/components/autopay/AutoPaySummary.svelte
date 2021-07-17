<script>
    export let payments;

    // TODO: move to the server side
    let summary;
    $: if (payments) {
        // calc totals by type
        let totals = {};
        payments.forEach(each => {
            let previous = totals[each.type_desc];
            totals[each.type_desc] = (previous || 0) + each.amt;
        });

        // format summary
        summary = [];
        Object.keys(totals).forEach(type => {
            let total = (type == "percent of balance" || type == "percent of change") 
                ? (totals[type] / 100).toFixed(2) + "%"
                : totals[type].toLocaleString('en-ES');
            summary.push({
                type: type,
                total: total 
            })
        });
    } 
</script>

<style>
    .autopay-summary-container {
        margin: auto;
        max-width: 400px;
    }
</style>

{#if summary}
    <div class="uk-card uk-card-default uk-card-body uk-margin-bottom autopay-summary-container">
        <table class="uk-table">
            <thead>
                <tr>
                    <th class="uk-text-left">type</th>
                    <th class="uk-text-center">total</th>
                </tr>
            </thead>
            <tbody>
                {#each summary as {type, total}}
                <tr>
                    <td class="uk-text-left">{type}</td>
                    <td class="uk-text-right">{total}</td>
                {/each}
            </tbody>            
        </table>        
    </div>
{/if}

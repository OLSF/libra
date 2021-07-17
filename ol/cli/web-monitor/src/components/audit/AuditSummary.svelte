<script>
    export let stats;
    function print_percent(num) {
        return (num * 100).toFixed(0) + "%";
    }
</script>

<style>
    .audit-container {
        margin: auto;
        max-width: 500px;
    }
</style>

<div class="uk-card uk-card-default uk-card-body uk-margin-bottom audit-container">
    <table class="uk-table">
        <thead>
            <tr>
                <th class="uk-text-left">check</th>
                <th class="uk-text-center">#vals</th>
                <th class="uk-text-center">%vals</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>Has Recurring Autopay</td>
                <td class="uk-text-right">{stats.count_vals_with_autopay}</td>
                <td class="uk-text-right">{print_percent(stats.percent_vals_with_autopay)}</td>
            </tr>
            <tr>
                <td>Has Operator Account</td>
                <td class="uk-text-right">{stats.count_vals_with_operator}</td>
                <td class="uk-text-right">{print_percent(stats.percent_vals_with_operator)}</td>
            </tr>
            <tr>
                <td>Has Operator Positive Balance</td>
                <td class="uk-text-right">{stats.count_positive_balance_operators}</td>
                <td class="uk-text-right">{print_percent(stats.percent_positive_balance_operators)}</td>
            </tr>
        </tbody>            
    </table>        
</div>

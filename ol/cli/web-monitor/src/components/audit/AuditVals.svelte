<script>
    import ValidatorModal from "../validators/ValidatorModal.svelte";
    import AuditSummary from "./AuditSummary.svelte";
    export let data;

    const modal_id = "audit-tab-val-modal";

    let audit_set = [];
    let selected_val = null;
    $: if (data.chain_view) {
        audit_set = data.chain_view.validator_view.map(each => {
            let sum_formated = each.autopay && each.autopay.recurring_sum 
                ? (each.autopay.recurring_sum / 100).toFixed(2) + "%"
                : "";
            return {
                view: each,
                address: each.account_address,
                has_autopay: each.autopay && each.autopay.payments.length > 0,
                has_op_account: each.validator_config.operator_account != null,
                has_op_balance: each.validator_config.operator_has_balance,
                recurring_sum: sum_formated
            }
        })
        if (selected_val == null) {
            selected_val = audit_set[0];
        }
    }
</script>

<div>
    <h2 class="uk-text-center uk-text-uppercase uk-text-muted uk-text-light uk-margin-medium-bottom">
      Audit Validators Configs
    </h2>
    {#if data.chain_view}
        <AuditSummary stats={data.chain_view.vals_config_stats} />
        <div class="uk-overflow-auto">
            <table class="uk-table uk-table-hover">
                <thead>
                    <tr>
                        <th class="uk-text-center">Validator</th>
                        <th class="uk-text-center">Recurring Autopay</th>
                        <th class="uk-text-center">Operator Account</th>
                        <th class="uk-text-center">Operator Has<br>Positive Balance</th>
                        <th class="uk-text-center"></th>
                    </tr>
                </thead>
                <tbody>
                    {#each audit_set as val}
                        <tr on:click={() => selected_val = val.view}>
                            <td class="uk-visible@s uk-text-center">{val.address}</td>
                            <td class="uk-hidden@s uk-text-truncate">{val.address}</td>
                            <td class="uk-text-center">
                                {#if val.has_autopay}
                                    <span class="uk-text-success" uk-icon="icon: check"></span> {val.recurring_sum}
                                {:else}
                                    <span class="uk-text-danger" uk-icon="icon: close"></span>
                                {/if}
                            </td>
                            <td class="uk-text-center">
                                {#if val.has_op_account}
                                    <span class="uk-text-success" uk-icon="icon: check"></span>    
                                {:else}
                                    <span class="uk-text-danger" uk-icon="icon: close"></span>
                                {/if}
                            </td>
                            <td class="uk-text-center">
                                {#if val.has_op_balance}
                                    <span class="uk-text-success" uk-icon="icon: check"></span>
                                {:else}
                                    <span class="uk-text-danger" uk-icon="icon: close"></span>
                                {/if}
                            </td>
                            <td class="uk-text-center">
                                <span uk-icon="icon: info" uk-toggle="target: #{modal_id}"></span>
                            </td>
                        </tr>
                    {/each}                
                </tbody>
            </table>
        </div>
    {:else}
        loading...
    {/if}
    <ValidatorModal validator={selected_val} id={modal_id}></ValidatorModal>
</div>
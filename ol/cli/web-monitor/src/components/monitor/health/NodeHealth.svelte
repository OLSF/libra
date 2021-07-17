<script lang="ts">
  import Check from "./Check.svelte";
  import { map } from "lodash";

  export let health_data;

  interface CheckObj {
    id: string;
    title: string;
    description: string;
    is_true: Boolean;
  }

  let allChecks: CheckObj[] = [
    {
      id: "config",
      title: "Node configured",
      description: "operator files created",
      is_true: false,
    },
    {
      id: "restore",
      title: "DB boostrapped",
      description: "db successfully initialized",
      is_true: false,
    },
    {
      id: "account",
      title: "Account exists",
      description: "owner account created on chain",
      is_true: false,
    },
    {
      id: "miner",
      title: "Miner is running",
      description: "process `miner` has started",
      is_true: false,
    },
    {
      id: "node",
      title: "Node is running",
      description: "process `libra-node` has started",
      is_true: false,
    },
    {
      id: "sync",
      title: "Node is synced",
      description: "node is up to date with upstream",
      is_true: false,
    },
    {
      id: "set",
      title: "In validator set",
      description: "owner account is in the validator set",
      is_true: false,
    },
    {
      id: "correct_mode",
      title: "Mode",
      description: "node running in mode: ",
      is_true: false,
    },
    {
      id: "has_autopay",
      title: "Autopay",
      description: "autopay instructions are set up",
      is_true: false,
    },
    {
      id: "has_operator_set",
      title: "Operator Account",
      description: "operator account is not set",
      is_true: false,
    },
    {
      id: "has_operator_positive_balance",
      title: "Operator Balance",
      description: "operator balance is not greater than zero",
      is_true: false,
    },
  ];

  $: if (health_data) {
    allChecks = map(allChecks, (i: CheckObj) => {
      if (i.id === "config") {
        i.is_true = health_data.configs_exist;
      }
      if (i.id === "account") {
        i.is_true = health_data.account_created;
      }
      if (i.id === "restore") {
        i.is_true = health_data.db_restored;
      }
      if (i.id === "node") {
        i.is_true = health_data.node_running;
      }
      if (i.id === "miner") {
        i.is_true = health_data.miner_running;
      }
      if (i.id === "sync") {
        i.is_true = health_data.is_synced;
      }
      if (i.id === "set") {
        i.is_true = health_data.validator_set;
      }
      if (i.id === "correct_mode") {
        i.is_true = false; 
        if (health_data.validator_set) {
          i.is_true = health_data.node_mode == "Validator";
        } else {
          i.is_true = health_data.node_mode != "Validator";
        }
        i.description = "node running in mode: ".concat(health_data.node_mode);
      }
      if (i.id === "has_operator_set") {
        i.is_true = health_data.has_operator_set;
        i.description = i.is_true 
          ? "operator account is set"
          : "operator account is not set";
      }
      if (i.id === "has_operator_positive_balance") {
        i.is_true = health_data.has_operator_positive_balance;
        i.description = i.is_true 
          ? "operator balance is positive"
          : "operator balance is not positive";
      }
      if (i.id === "has_autopay") {
        i.is_true = health_data.has_autopay;
        i.description = i.is_true 
          ? "autopay instructions are set up"
          : "autopay instructions not found";
      }
      return i;
    });
  }
</script>

<div class="uk-card uk-card-default uk-card-body uk-margin-bottom">
  <h3 class="uk-card-title uk-text-center uk-text-uppercase uk-text-muted">
    Node Health
  </h3>
  {#if health_data}
    <dl class="uk-description-list">
      {#each allChecks as c}
        <Check
          title={c.title}
          description={c.description}
          isTrue={c.is_true}
        />
      {/each}
    </dl>
  {:else}
    <p>loading...</p>
  {/if}
</div>


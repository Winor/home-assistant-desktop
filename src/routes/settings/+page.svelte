<script lang="ts">
    import { state } from "../../store/state";
    import { goto } from "$app/navigation";
</script>

{#await state.update()}
...
{:then}
{#if $state === "Disconnected"}
    {goto("/settings/list")}
{:else}
    <span class="flex-auto">
        <h1 class="h1">You are connected.</h1>
        <h2>server ID: {$state.Connected}</h2>
    </span>
{/if}
{/await}
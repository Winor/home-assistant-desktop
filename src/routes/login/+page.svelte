<script lang="ts" context="module">
    export let server: string;
</script>

<script lang="ts">
    import { MinusOutline, PlusOutline } from "flowbite-svelte-icons";
    import { ProgressBar } from "@skeletonlabs/skeleton";
    import { list } from "../../store/server";
    import { get } from "svelte/store";
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api";
    import { listen } from "@tauri-apps/api/event";
    import { error } from "console";
    import { onMount } from "svelte";

    onMount(async () => {
        await list.updateList();
    });

    // list.updateList();

    // function callback() : Promise<string> {
    //     return new Promise(async (resolve) => {

    //         const unlisten = await listen<string>("connection", (event) => {
    //             unlisten();
    //             resolve(event.payload);
    //         });

    //     });
    // }

    let isLoading = false;

    async function connect() {
        try {
            if (server) {
                isLoading = true;
                const cnt = await invoke("hass_connect", { serverId: server });
                console.log(cnt)
            } else throw new Error("Server not selected");
        } catch (err) {
            isLoading = false;
            alert(err);
        }
    }

    function add() {
        goto("/login/add");
    }

    function remove() {
        list.remove(server);
    }
</script>

<h1 class="h1">Select a server</h1>
<button type="button" class="btn-icon variant-filled" on:click={add}
    ><PlusOutline /></button
>
<button type="button" class="btn-icon variant-filled" on:click={remove}
    ><MinusOutline /></button
>
<select class="select" size="4" bind:value={server}>
    {#each $list as [key, value]}
        <option value={key}>{value}</option>
    {/each}
</select>

{#if isLoading}
    <ProgressBar />
{:else}
    <button type="button" class="btn variant-filled" on:click={connect}
        >Connect</button
    >
{/if}

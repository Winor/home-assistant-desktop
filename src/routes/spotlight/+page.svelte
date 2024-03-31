<script lang="ts">
    import type { AutocompleteOption } from "@skeletonlabs/skeleton";
    import { entities } from "../../store/entities";
    let input = "";

    function onFlavorSelection(
        event: CustomEvent<AutocompleteOption<string>>,
    ): void {
        input = event.detail.label;
    }
</script>

{#await entities.update()}
    ...
{:then}
    <input
        class="input"
        type="search"
        name="spotlight"
        bind:value={input}
        placeholder="Search..."
    />
    {#if input != ""}
        <div class="card p-4">
            <nav class="list-nav">
                {#each $entities as entity}
                    <ul>
                        <li>
                            <a href="/elements/lists">
                                <span class="badge bg-primary-500">(icon)</span>
                                <span class="flex-auto">{entity.attributes.friendly_name}</span>
                            </a>
                        </li>
                        <!-- ... -->
                    </ul>
                {/each}
            </nav>
        </div>
    {/if}
{/await}

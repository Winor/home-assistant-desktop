<script lang="ts">
  import { AppShell, AppBar } from "@skeletonlabs/skeleton";
  import "../app.pcss";
</script>

<AppShell>
  <svelte:fragment slot="header">

  </svelte:fragment>
  <div class="container mx-auto p-8 space-y-8">
    <slot />
  </div>
</AppShell>

<style>
</style>

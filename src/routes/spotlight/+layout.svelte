<script lang="ts">
    import { AppShell } from "@skeletonlabs/skeleton";
    import "../../app.pcss";
  </script>
  
  <AppShell>
    <svelte:fragment slot="header">
  
    </svelte:fragment>
    <div data-tauri-drag-region class="container mx-auto p-8 space-y-8">
      <slot />
    </div>
  </AppShell>
  
<style>
    :global(body)  {
      background-color: rgba(0, 0, 0, 0) !important;
    }
</style>
  
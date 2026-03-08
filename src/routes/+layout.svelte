<script lang="ts">
  import "../app.css";

  import { Tooltip } from "bits-ui";

  let { children } = $props();
</script>

<svelte:body
  oncontextmenu={(event) => {
    // hide tauri context menu in release builds
    if (window.location.hostname === "tauri.localhost") {
      event.preventDefault();
    }
  }}
/>

<Tooltip.Provider skipDelayDuration={1} disableCloseOnTriggerClick>
  <div class="fixed inset-0 flex flex-col overflow-hidden">
    <main class="flex min-h-0 flex-1 flex-col overflow-hidden bg-primary-900">
      <div class="flex min-h-0 flex-1 overflow-hidden">{@render children?.()}</div>
    </main>
  </div>
</Tooltip.Provider>

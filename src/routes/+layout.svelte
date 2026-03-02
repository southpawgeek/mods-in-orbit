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
  <main class="relative flex flex-col overflow-hidden bg-primary-900">
    <div class="relative flex overflow-hidden grow">
      {@render children?.()}
    </div>
  </main>
</Tooltip.Provider>

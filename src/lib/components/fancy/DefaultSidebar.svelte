<script lang="ts">
  import Header from "./Header.svelte";
</script>

<div class="relative flex min-h-0 min-w-72 w-[40%] flex-col overflow-hidden">
  <div class="fancy-headered-content">
    <Header
      text="Welcome!"
      position="left"
    />

    <p class="mt-2 text-xl text-primary-300 grow">
      Available mods are on the left. Click one to view details.
    </p>
  </div>
</div>

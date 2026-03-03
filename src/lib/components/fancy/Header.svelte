<script lang="ts">
  import tabTrinkets from "$lib/assets/tab_trinkets.png";

  let { text = "" } = $props<{ text?: string }>();
</script>

<div class="fancy-header">
  <div class="fancy-left">&nbsp;</div>
  <img src={tabTrinkets} alt="" class="fancy-header-icon" />
  <h3>{text}</h3>
  <div class="fancy-right">&nbsp;</div>
</div>

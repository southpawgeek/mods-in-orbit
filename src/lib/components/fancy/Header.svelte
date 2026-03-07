<script lang="ts">
  import tabTrinkets from "$lib/assets/tab_trinkets.png";

  let { text = "", position = "left" } = $props<{
    text?: string;
    position?: "left" | "right";
  }>();
</script>

<div class="fancy-header {position === "left" ? "header-left" : "header-right"}">
  <div class="fancy-left">&nbsp;</div>
  <img src={tabTrinkets} alt="" class="fancy-header-icon" />
  <h3>{text}</h3>
  <div class="fancy-right">&nbsp;</div>
</div>

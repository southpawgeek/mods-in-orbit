<script lang="ts">
  import { Dialog } from "bits-ui";
  import type { Snippet } from "svelte";
  import { expoOut, linear } from "svelte/easing";
  import { fade, fly } from "svelte/transition";

  type Props = {
    open: boolean;
    title?: string | null;
    canClose?: boolean;
    large?: boolean;
    onclose?: () => void;
    children?: Snippet;
  };

  let {
    open = $bindable(),
    title = null,
    canClose = true,
    large = false,
    onclose,
    children,
  }: Props = $props();

  async function close(evt: UIEvent) {
    if (!canClose) {
      evt.preventDefault();
      return;
    }

    open = false;
    onclose?.();
  }
</script>

<Dialog.Root
  bind:open
  onOpenChange={(open) => {
    if (!open) onclose?.();
  }}
>
  <Dialog.Portal>
    <Dialog.Overlay
      forceMount
      class="pointer-events-none"
      data-tauri-drag-region={!canClose}
    >
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            transition:fade={{ duration: 80 }}
            class="fixed inset-0 z-0 rounded-lg bg-black/60"
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>
    <Dialog.Content
      interactOutsideBehavior={canClose ? "close" : "ignore"}
      class="pointer-events-none"
    >
      {#if open}
        <div
          class="fixed inset-0 flex items-center justify-center pointer-events-none"
          in:fly={{ duration: 200, easing: expoOut, y: 8 }}
          out:fly={{ duration: 50, easing: linear, y: 5 }}
        >
          <div
            class={[
              large ? "max-w-240" : "max-w-140",
              "border-primary-600 bg-primary-800 pointer-events-auto relative z-30 max-h-[85%] w-[85%] overflow-x-hidden overflow-y-auto rounded-xl border p-6 shadow-xl",
            ]}
          >
            {#if title}
              <Dialog.Title
                class="w-full pr-10 text-2xl font-bold text-white wrap-break-word"
                >Dependencies of {title}</Dialog.Title
              >
            {/if}

            {@render children?.()}

            {#if canClose}
              <button
                class="text-primary-400 hover:bg-primary-700 hover:text-primary-300 absolute top-5 right-5 rounded-md p-0.5 text-3xl"
                onclick={close}
              >
                test
              </button>
            {/if}
          </div>
        </div>
      {/if}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

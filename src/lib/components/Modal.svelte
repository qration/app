<script lang="ts">
	import type { Snippet } from "svelte";
	import IconButton from "./IconButton.svelte";
	import { IconX } from "@tabler/icons-svelte-runes";

  let { title, body, open = $bindable() }: {
    title: string,
    body: Snippet,
    open: boolean,
  } = $props();

  let dialog: HTMLDialogElement | undefined = $state();

  $effect(() => { if (open) dialog?.showModal() });
</script>

<dialog
  bind:this={dialog}
  onclose={() => { open = false }}
  class="p-10 rounded-2xl w-full max-w-2xl m-auto">
  <div class="flex flex-col gap-2">
    <div class="flex flex-row items-center justify-between">
      <span class="text-4xl font-bold">{title}</span>
      <IconButton Icon={IconX} onclick={() => { dialog?.close() }}/>
    </div>
    {@render body?.()}
  </div>
</dialog>
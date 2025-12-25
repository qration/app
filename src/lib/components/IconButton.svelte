<script lang="ts">
	import type { Component } from "svelte";
	import type { MouseEventHandler } from "svelte/elements";

  let { text = '', Icon, onclick }: {
    text?: string,
    Icon: Component,
    onclick: MouseEventHandler<EventTarget>
  } = $props();
</script>

<button class="bg-gray-100
             px-2 py-1 my-1
             rounded-xl border-2
             w-min cursor-pointer hover:brightness-90"
        {onclick} >
  <Icon />
  {text}
</button>
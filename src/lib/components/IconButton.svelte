<script lang="ts">
	import type { Component } from "svelte";
	import type { MouseEventHandler } from "svelte/elements";

  let { text = '', Icon, onclick }: {
    text?: string,
    Icon: Component,
    onclick: MouseEventHandler<EventTarget>
  } = $props();
</script>

<button class="bg-white px-2 py-2
               rounded-xl border-2 border-gray-400
               w-min cursor-pointer hover:bg-gray-200"
        {onclick} >
  <Icon />
  {text}
</button>
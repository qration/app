<script lang="ts">
	import Tag from './Tag.svelte';
  import { getRelativeTime } from '$lib/util/date';
	import type { Item } from '$lib/util/types';
  const rtf = new Intl.RelativeTimeFormat('en-CA');

  let { item }: { item: Item } = $props();
</script>

<div class="w-full border-2 border-gray-400 p-4 rounded-xl flex flex-col">
  <span class="text-3xl font-bold">{item.name}</span>
  <div class="flex flex-row gap-1">
    {#each item.tags as tag}
      <Tag name={tag}/>
    {/each}
  </div>
  <span>{item.body}</span>
  <span class="text-gray-500 italic mt-auto self-end">{getRelativeTime(item.date, rtf)}</span>
</div>
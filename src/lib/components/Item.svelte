<script lang="ts">
	import Tag from './Tag.svelte';
  import { getRelativeTime } from '$lib/util/date';
	import type { Item } from '$lib/util/types';
	import IconButton from './IconButton.svelte';
  import {
    IconExternalLink,
    IconStar,
    IconStarFilled
  } from '@tabler/icons-svelte-runes';
	import type { MouseEventHandler } from 'svelte/elements';
  const rtf = new Intl.RelativeTimeFormat('en-CA');

  let { item, onstar }: {
    item: Item,
    onstar: MouseEventHandler<EventTarget>
  } = $props();
</script>

<div class="w-full border-2 border-gray-400 p-4 rounded-xl flex flex-col bg-white">
  <div class="flex flex-row justify-between">
    <a href={item.link} target="_blank" class="hover:underline">
      <span class="text-3xl font-bold flex flex-row gap-2 items-center">
        {item.name}
        <IconExternalLink/>
      </span>
    </a>
    <IconButton Icon={item.starred ? IconStarFilled : IconStar} onclick={onstar} />
  </div>
  <div class="flex flex-row gap-1">
    {#each item.tags as tag}
      <Tag name={tag}/>
    {/each}
  </div>
  <span>{item.body}</span>
  <span class="text-gray-500 italic mt-auto self-end">{getRelativeTime(item.date, rtf)}</span>
</div>
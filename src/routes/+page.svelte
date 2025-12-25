<script lang="ts">
	import Item from '$lib/components/Item.svelte';
	import data from '../test-data.json';

	let starToggle = (index: number) => {
		return data[index].starred = !data[index].starred;
  }
</script>

<div class="flex flex-col gap-2">
	{#each data as item, index}
		<Item item={item} onstar={() => starToggle(index)} />
	{/each}
</div>
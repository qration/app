<script>
	import Item from '$lib/components/Item.svelte';
	import data from '../test-data.json';
</script>

<div class="flex flex-col gap-2">
	{#each data as item}
		<Item
			name={item.name}
			tags={item.tags}
			body={item.body}
			date={item.date}/>
	{/each}
</div>
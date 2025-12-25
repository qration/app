<script>
	import Item from '$lib/components/Item.svelte';
	import data from '../test-data.json';
</script>

<div class="flex flex-col gap-2">
	{#each data as item}
		<Item item={item}/>
	{/each}
</div>
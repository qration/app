<script lang="ts">
	import Item from '$lib/components/Item.svelte';
	import data from '../test-data.json';
	let filtered = $state(data);

	let search = $state('');

	let starToggle = (index: number) => {
		return data[index].starred = !data[index].starred;
  }

	let searchFilter = () => {
		console.log(search);
		filtered = data.filter(x =>
		  x.body.toLowerCase().startsWith(search.toLowerCase()) ||
			x.name.toLowerCase().startsWith(search.toLowerCase()))
	}
</script>
<div class="flex flex-col gap-2">
	<input class="w-full rounded-xl border-2 border-gray-400"
				 type="text" placeholder="Search..."
				 bind:value={search} oninput={searchFilter} />
	{#each filtered as item, index}
		<Item item={item} onstar={() => starToggle(index)} />
	{/each}
</div>
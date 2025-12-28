<script lang="ts">
	import IconButton from '$lib/components/IconButton.svelte';
	import Item from '$lib/components/Item.svelte';
	import { IconLayoutGrid, IconLayoutList, IconSearch } from '@tabler/icons-svelte-runes';
	import data from '../test-data.json';

	let filtered = $state(data);
	let search = $state('');
	let layoutList = $state(true);

	// surely there's a better way to do this?
	let starToggle = (id: number): void => {
		const index = data.findIndex(x => x.id == id); 
		const fIndex = filtered.findIndex(x => x.id == id); 
		data[index].starred = !data[index].starred;
		filtered[fIndex].starred = !filtered[fIndex].starred;
	}

	let searchFilter = (): void => {
		filtered = data.filter(x =>
			x.body.toLowerCase().includes(search.toLowerCase()) ||
			x.name.toLowerCase().includes(search.toLowerCase()));
	}

	let layoutToggle = (): void => {
		layoutList = !layoutList;
	}
</script>

<div class="flex flex-col gap-2">
	<div class="flex flex-row gap-1.5">
		<div class="w-full rounded-xl border-2 border-gray-400
		            flex flex-row items-center">
			<IconSearch class="absolute left-4"/>
			<input class="border-none bg-none rounded-xl w-full pl-12"
						 type="search" placeholder="Search..."
						 bind:value={search} oninput={searchFilter} />
		</div>
		<IconButton Icon={layoutList ? IconLayoutList : IconLayoutGrid}
		            onclick={layoutToggle}/>
	</div>
	<div class={`${layoutList ? 'flex flex-col' : 'grid grid-cols-4'} gap-2`}>
		{#each filtered as item}
			<Item item={item} onstar={() => starToggle(item.id)} />
		{/each}
	</div>
</div>
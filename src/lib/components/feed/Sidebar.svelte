<script lang="ts">
	import wordmark from '$lib/assets/qration-wordmark.svg';
	import { setContext } from 'svelte';
	import IconButton from '../ui/IconButton.svelte';
	import SidebarButton from '../ui/SidebarButton.svelte';
	import AddNew from './AddNew.svelte';
	import { resolve } from '$app/paths';
	import { getFeedIcon } from '$lib/util/util';
	import type { Feed } from '$lib/util/interfaces';

	let collapsed = $state(false);
	let selectedFilter = $state('feed-all');
	let addNewOpen = $state(false);

	let {
		onfilterchange,
		data,
	}: { onfilterchange: (filter: string) => void; data: { feeds: Feed[] } } =
		$props();

	setContext('sidebar', {
		isCollapsed: () => collapsed,
	});

	function toggleSidebarCollapse() {
		collapsed = !collapsed;
	}

	function setSelectedFilter(filter: string) {
		selectedFilter = filter;
		onfilterchange(filter);
	}
</script>

<div
	class="border-r-2 font-medium border-r-border bg-bg pl-4 pr-3.5 py-4
		overflow-x-hidden {collapsed ? 'w-16' : 'w-70'} flex flex-col shrink-0
		justify-between transition-all duration-100">
	<div class="flex flex-col gap-4 shrink-0">
		<div class="flex flex-col">
			<div
				class="flex flex-row
					{collapsed ? ' justify-center' : ' justify-between'} items-center">
				<div
					class="overflow-hidden transition-all duration-75
						{collapsed ? ' max-w-0 opacity-0' : ' max-w-xs opacity-100'}
						shrink-0">
					<a href={resolve('/')}>
						<img
							src={wordmark}
							alt="Qration logo"
							class="max-w-none shrink-0 h-10 cursor-pointer transition-opacity" />
					</a>
				</div>
				<div class="w-6 flex justify-center shrink-0">
					<IconButton
						icon="tabler:layout-sidebar-left-{collapsed
							? 'expand'
							: 'collapse'}"
						label="Collapse"
						onclick={() => toggleSidebarCollapse()} />
				</div>
			</div>
		</div>
		<div class="items-center">
			<SidebarButton
				text="Add New"
				icon="tabler:plus"
				onclick={() => (addNewOpen = true)} />
		</div>
		<div class="flex flex-col gap-0.5">
			<SidebarButton
				text="All Feeds"
				icon="tabler:news"
				selected={selectedFilter == 'feed-all'}
				onclick={() => setSelectedFilter('feed-all')} />
			<SidebarButton
				text="Unread"
				icon="tabler:notification"
				selected={selectedFilter == 'article-unread'}
				onclick={() => setSelectedFilter('article-unread')} />
			<SidebarButton
				text="Today"
				icon="tabler:calendar"
				selected={selectedFilter == 'article-today'}
				onclick={() => setSelectedFilter('article-today')} />
			<SidebarButton
				text="Favourites"
				icon="tabler:star"
				selected={selectedFilter == 'feed-favourites'}
				onclick={() => setSelectedFilter('feed-favourites')} />
			<SidebarButton
				text="Saved"
				icon="tabler:bookmark"
				selected={selectedFilter == 'article-saved'}
				onclick={() => setSelectedFilter('article-saved')} />
		</div>

		<div class="flex flex-col gap-0.5">
			{#if !collapsed}
				<div class="text-xl">Subscriptions</div>
			{/if}
			{#each data.feeds as feed (feed.id)}
				<SidebarButton
					text={feed.name}
					icon={getFeedIcon(feed.type)}
					selected={selectedFilter == feed.id}
					onclick={() => setSelectedFilter(feed.id)}
					starrable={true}
					starred={feed.favourited}
					onstar={() => (feed.favourited = !feed.favourited)} />
			{/each}
		</div>
	</div>

	<div class="flex flex-col">
		<SidebarButton text="Settings" icon="tabler:settings" />
	</div>
</div>

<AddNew bind:open={addNewOpen} />

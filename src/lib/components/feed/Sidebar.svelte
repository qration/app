<script lang="ts">
	import Wordmark from '$lib/assets/qration-wordmark.svg?component';
	import { setContext } from 'svelte';
	import IconButton from '../ui/IconButton.svelte';
	import SidebarButton from '../ui/SidebarButton.svelte';
	import AddNew from './AddNew.svelte';
	import { resolve } from '$app/paths';
	import { getFeedIcon } from '$lib/util/util';
	import { feedStore } from '$lib/stores/feeds.svelte';
	import Settings from './Settings.svelte';
	import ConfirmDelete from './ConfirmDelete.svelte';
	import type { Feed } from '$lib/util/bindings';

	let collapsed = $state(false);
	let selectedFilter = $state('feed-all');
	let delFeed: Feed | undefined = $state();
	let addNewOpen = $state(false);
	let settingsOpen = $state(false);
	let deleteOpen = $state(true);

	let { onfilterchange }: { onfilterchange: (filter: string) => void } =
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
	class="overflow-x-hidden border-r-2 border-r-border bg-bg px-3.75 py-4
		font-medium {collapsed ? 'w-16' : 'w-70'} flex shrink-0 flex-col
		justify-between transition-all duration-500">
	<div class="flex shrink-0 flex-col gap-4">
		<div class="flex flex-col">
			<div class="relative flex h-10 items-center">
				<div
					class="overflow-hidden transition-all duration-500
						{collapsed ? 'max-w-0 opacity-0' : 'max-w-xs opacity-100'} shrink-0">
					<a href={resolve('/')}>
						<Wordmark
							class="h-10 max-w-none shrink-0 cursor-pointer fill-text" />
					</a>
				</div>
				<div class="absolute right-1 flex w-6 justify-center">
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
				<div class="text-xl text-text">Subscriptions</div>
			{/if}
			{#each feedStore.data.feeds as feed (feed.id)}
				<SidebarButton
					text={feed.name}
					icon={getFeedIcon(feed.feed_type)}
					selected={selectedFilter == feed.id}
					onclick={() => setSelectedFilter(feed.id)}
					starrable={true}
					starred={feed.favourited}
					onstar={() => (feed.favourited = !feed.favourited)}
					ondelete={() => {
						delFeed = feed;
						deleteOpen = true;
					}} />
			{/each}
		</div>
	</div>

	<div class="flex flex-col">
		<SidebarButton
			text="Settings"
			icon="tabler:settings"
			onclick={() => (settingsOpen = true)} />
	</div>
</div>

<AddNew bind:open={addNewOpen} />
<Settings bind:open={settingsOpen} />
<ConfirmDelete feed={delFeed} bind:open={deleteOpen} />

<script lang="ts">
	import Wordmark from '$lib/assets/qration-wordmark.svg?component';
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import SidebarButton from '$lib/components/ui/SidebarButton.svelte';

	import AddNew from './dialogs/AddNew.svelte';
	import Settings from './dialogs/Settings.svelte';
	import ConfirmDelete from './dialogs/ConfirmDelete.svelte';

	import { onMount, setContext } from 'svelte';
	import { resolve } from '$app/paths';
	import { getFeedIcon } from '$lib/util/util';
	import { getFeedStore, getMQ } from '$lib/context/context.svelte';

	import { commands, type Feed } from '$lib/util/bindings';

	let selectedFilter = $state('feed-all');
	let delFeed: Feed | undefined = $state();
	let addNewOpen = $state(false);
	let settingsOpen = $state(false);
	let deleteOpen = $state(true);

	let sidebarRef: HTMLElement;

	let {
		onfilterchange,
		collapsed,
		oncollapse,
	}: {
		onfilterchange: (filter: string) => void;
		collapsed: boolean;
		oncollapse: () => void;
	} = $props();

	setContext('sidebar', {
		isCollapsed: () => collapsed,
	});

	const feedStore = getFeedStore();
	const MQ = getMQ();

	function setSelectedFilter(filter: string) {
		selectedFilter = filter;
		onfilterchange(filter);
	}

	onMount(() => {
		const observer = new ResizeObserver(() => {
			if (!sidebarRef) return;
			sidebarRef.style.transition = 'none';
			void sidebarRef.offsetHeight;
			sidebarRef.style.transition = '';
		});

		observer.observe(document.body);

		return () => observer.disconnect();
	});
</script>

<div
	bind:this={sidebarRef}
	class="overflow-x-hidden border-r border-r-border bg-bg px-3.75 py-4
		font-medium {collapsed
		? '-translate-x-full lg:w-16'
		: 'translate-x-0 lg:w-70'} safe fixed inset-y-0 z-50 flex h-full w-70
		shrink-0 flex-col justify-between transition-all duration-500 lg:static lg:translate-x-0"
	inert={collapsed && !MQ.current}>
	<div class="flex shrink-0 flex-col gap-4 pt-safe-top pb-safe-bottom">
		<div class="flex flex-col">
			<div class="relative flex h-10 items-center">
				<div
					class="overflow-hidden duration-500 lg:transition-opacity
						{collapsed ? 'max-w-0 opacity-0' : 'max-w-xs opacity-100'} shrink-0">
					<a href={resolve('/')} tabindex={collapsed ? -1 : 0}>
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
						onclick={() => oncollapse()} />
				</div>
			</div>
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

		<div
			class="flex flex-col gap-0.5 transition-opacity duration-500 {collapsed
				? 'opacity-0'
				: 'opacity-100'}">
			<div class="flex flex-row items-center justify-between text-xl text-text">
				<div>Subscriptions</div>
				<IconButton
					icon="tabler:plus"
					label="Add Feed"
					onclick={() => (addNewOpen = true)} />
			</div>
			{#each feedStore.feeds as feed (feed.id)}
				<SidebarButton
					text={feed.feed_name}
					icon={getFeedIcon(feed.feed_type)}
					selected={selectedFilter == feed.id}
					onclick={() => setSelectedFilter(feed.id)}
					starrable={true}
					starred={feed.favourited}
					tabindex={collapsed ? -1 : 0}
					onstar={async () => {
						await commands.setStarFeed(feed.id, !feed.favourited);
						feed.favourited = !feed.favourited;
					}}
					ondelete={() => {
						delFeed = feed;
						deleteOpen = true;
					}}
					onrefresh={async () => {
						await commands.refreshFeed(feed.id);
						await feedStore.load();
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

<script>
	import wordmark from '$lib/assets/qration-wordmark.svg';
	import { setContext } from 'svelte';

	import IconButton from '../ui/IconButton.svelte';
	import SidebarButton from '../ui/SidebarButton.svelte';

	let collapsed = $state(false);

	setContext('sidebar', {
		isCollapsed: () => collapsed,
	});

	function toggleSidebarCollapse() {
		collapsed = !collapsed;
	}
</script>

<div
	class="border-r-2 font-medium border-r-gray-300 p-4 overflow-x-hidden
		{collapsed ? 'w-16' : 'w-70'} flex flex-col justify-between transition-all
		duration-100">
	<div class="flex flex-col gap-4 shrink-0">
		<div class="flex flex-col">
			<div
				class="flex flex-row items-center{collapsed
					? ' justify-center'
					: ' justify-between'}">
				<div
					class="overflow-hidden transition-all duration-75 shrink-0{collapsed
						? ' max-w-0 opacity-0'
						: ' max-w-xs opacity-100'}">
					<img
						src={wordmark}
						alt="Qration logo"
						class="max-w-none shrink-0 h-10 cursor-pointer transition-opacity" />
				</div>
				<div class="w-6 flex justify-center shrink-0">
					<IconButton
						iconClass="ti ti-layout-sidebar-left-{collapsed
							? 'expand'
							: 'collapse'}"
						label="Collapse"
						onclick={() => toggleSidebarCollapse()} />
				</div>
			</div>
		</div>
		<div class="items-center">
			<SidebarButton text="Add New" iconClass="ti ti-plus " />
		</div>
		<div class="flex flex-col">
			<SidebarButton text="All Feeds" iconClass="ti ti-news " />
			<SidebarButton text="Unread" iconClass="ti ti-notification " />
			<SidebarButton text="Favourites" iconClass="ti ti-star " />
			<SidebarButton text="Saved" iconClass="ti ti-bookmark " />
		</div>

		{#if !collapsed}
			<div class="text-xl">Subscriptions</div>
		{/if}
	</div>

	<div class="flex flex-col">
		<SidebarButton text="Settings" iconClass="ti ti-settings " />
	</div>
</div>

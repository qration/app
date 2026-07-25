<script module lang="ts">
	let uid = 0;
</script>

<script lang="ts">
	import Icon from '@iconify/svelte';
	import IconButton from './IconButton.svelte';
	import DetailsList from './DetailsList.svelte';

	import { getContext } from 'svelte';

	let {
		text,
		icon,
		selected = false,
		starrable = false,
		starred = false,
		tabindex = 0,
		onclick,
		onstar,
		ondelete,
		onrefresh,
	}: {
		text: string;
		icon: string;
		selected?: boolean;
		starrable?: boolean;
		starred?: boolean;
		tabindex?: number;
		onclick?: (e?: MouseEvent) => void;
		onstar?: () => void;
		ondelete?: () => void;
		onrefresh?: () => void;
	} = $props();

	// eslint-disable-next-line no-useless-assignment
	const menuId = `sidebar-details-${uid++}`;
	const sidebar: { isCollapsed: () => boolean } = getContext('sidebar');
	const collapsed = $derived(sidebar.isCollapsed());

	function handleKeyDown(e: KeyboardEvent) {
		if (e.target !== e.currentTarget) return;
		if (e.key === ' ' || e.key === 'Enter') {
			e.preventDefault();
			onclick?.();
		}
	}
</script>

<div
	role="button"
	{tabindex}
	class="group flex w-full cursor-pointer flex-row items-center gap-2 p-1
    text-text-muted {selected ? 'bg-bg-hover' : 'bg-bg'} rounded hover:text-text
    [&:hover:not(:has(button:hover)):not(:has(:popover-open))]:bg-bg-hover"
	{onclick}
	onkeydown={handleKeyDown}>
	<div class="flex shrink-0 items-center justify-center text-2xl leading-none">
		<Icon {icon} />
	</div>
	<div
		class="flex min-w-0 flex-1 flex-row justify-between gap-5 whitespace-nowrap transition-all duration-500 {collapsed
			? 'w-0 overflow-hidden opacity-0'
			: 'w-full opacity-100'}">
		<div class="min-w-0 truncate">
			{text}
		</div>
		{#if starrable}
			<div class="m-0 shrink-0">
				<div
					class="opacity-100 group-focus-within:opacity-100 group-hover:opacity-100 group-has-[:popover-open]:opacity-100
						lg:opacity-0"
					class:opacity-100={selected}>
					<IconButton
						icon="tabler:dots-vertical"
						{tabindex}
						label="Details for {text}"
						popovertarget={menuId}
						class="-m-1 {selected
							? 'bg-bg-hover!'
							: ''} group-hover:bg-bg-hover"
						onclick={(e) => e.stopPropagation()} />
				</div>
				<DetailsList {menuId} {starred} {onstar} {ondelete} {onrefresh} />
			</div>
		{/if}
	</div>
</div>

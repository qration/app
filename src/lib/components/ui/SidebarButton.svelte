<script module lang="ts">
	let uid = 0;
</script>

<script lang="ts">
	import Icon from '@iconify/svelte';
	import { getContext } from 'svelte';
	import { fade } from 'svelte/transition';
	import IconButton from './IconButton.svelte';
	import DetailsList from './DetailsList.svelte';

	let {
		text,
		icon,
		selected = false,
		starrable = false,
		starred = false,
		onclick,
		onstar,
		ondelete,
	}: {
		text: string;
		icon: string;
		selected?: boolean;
		starrable?: boolean;
		starred?: boolean;
		onclick?: (e?: MouseEvent) => void;
		onstar?: () => void;
		ondelete?: () => void;
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
	tabindex="0"
	class="group flex w-full cursor-pointer flex-row items-center gap-2 p-1
    text-text-muted {selected ? 'bg-bg-hover' : 'bg-bg'} rounded hover:text-text
    [&:hover:not(:has(button:hover)):not(:has(:popover-open))]:bg-bg-hover"
	{onclick}
	onkeydown={handleKeyDown}>
	<div class="flex shrink-0 items-center justify-center text-2xl leading-none">
		<Icon {icon} />
	</div>
	{#if !collapsed}
		<div
			class="flex w-full flex-row justify-between whitespace-nowrap"
			transition:fade={{ duration: 500 }}>
			{text}
			{#if starrable}
				<div class="m-0">
					<div
						class="opacity-0 group-focus-within:opacity-100 group-hover:opacity-100
              group-has-[:popover-open]:opacity-100"
						class:opacity-100={selected}>
						<IconButton
							icon="tabler:dots-vertical"
							label="Details for {text}"
							popovertarget={menuId}
							class="-m-1 {selected
								? 'bg-bg-hover!'
								: ''} group-hover:bg-bg-hover"
							onclick={(e) => e.stopPropagation()} />
					</div>
					<DetailsList {menuId} {starred} {onstar} {ondelete} />
				</div>
			{/if}
		</div>
	{/if}
</div>

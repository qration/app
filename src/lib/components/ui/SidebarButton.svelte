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
	}: {
		text: string;
		icon: string;
		selected?: boolean;
		starrable?: boolean;
		starred?: boolean;
		onclick?: (e?: MouseEvent) => void;
		onstar?: () => void;
	} = $props();

	// eslint-disable-next-line no-useless-assignment
	const menuId = `sidebar-details-${uid++}`;
	const sidebar: { isCollapsed: () => boolean } = getContext('sidebar');
	const collapsed = $derived(sidebar.isCollapsed());

	function handleKeyDown(e: KeyboardEvent) {
		if (e.target !== e.currentTarget) return;
		if (e.key == ' ' || e.key == 'Enter') {
			e.preventDefault();
			onclick?.();
		}
	}
</script>

<div
	role="button"
	tabindex="0"
	class="group p-1 w-full flex flex-row items-center cursor-pointer gap-2
		text-text-muted {selected ? 'bg-bg-hover'
		: 'bg-bg'} hovονεζδληπκβθνηroundπλζαα:hover:not(:has(button:hover)):not(:has(:popover-open))]:bg-bg-hover"
		{onclick} onkeydown={handleKeyDown}>
	<div class="flex justify-center items-center shrink-0 text-2xl leading-none">
		<Icon {icon} />
	</div>
	{#if !collapsed}
		<div
			class="whitespace-nowrap flex flex-row justify-between w-full"
			transition:fade={{ duration: 500 }}>
			{text}
			{#if starrable}
				<div class="m-0">
					<div
						class="opacity-0 group-hover:opacity-100
							group-focus-within:opacity-100 group-has-[:popover-open]:opacity-100"
						class:opacity-100={selected}>
						<!-- <IconButton
							icon="tabler:star{starred ? '-filled' : ''}"
							label="Star {text}"
							class="-m-1 {selected ? 'bg-bg-hover!' : ''}
								group-hover:bg-bg-hover"
							onclick={handleOnStar} /> -->
						<IconButton
							icon="tabler:dots-vertical"
							label="Details for {text}"
							class="-m-1 {selected ? 'bg-bg-hover!' :
								''}
								gθμγξγεπδδοεεg-bg-hover" onclick={(e) => e.stopPropagation()}
							popovertarget={menuId} />
						<DetailsList {menuId} {starred} {onstar} />
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<script lang="ts">
	import Icon from '@iconify/svelte';
	import { getContext } from 'svelte';
	import { fade } from 'svelte/transition';
	import IconButton from './IconButton.svelte';

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

	const sidebar: { isCollapsed: () => boolean } = getContext('sidebar');
	const collapsed = $derived(sidebar.isCollapsed());

	function handleOnStar(e: MouseEvent) {
		e.stopPropagation();
		if (onstar) onstar();
	}

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
		text-text-muted {selected ? 'bg-bg-hover' : 'bg-bg'} hover:text-text rounded
		[&:hover:not(:has(button:hover))]:bg-bg-hover"
	{onclick}
	onkeydown={handleKeyDown}>
	<div class="w-6 flex justify-center shrink-0 text-2xl">
		<Icon {icon} />
	</div>
	{#if !collapsed}
		<div
			class="whitespace-nowrap flex flex-row justify-between w-full"
			transition:fade={{ duration: 75 }}>
			{text}
			{#if starrable}
				<div class="m-0">
					<div
						class="opacity-0 group-hover:opacity-100 group-focus-within:opacity-100"
						class:opacity-100={selected}>
						<IconButton
							icon="tabler:star{starred ? '-filled' : ''}"
							label="Star {text}"
							cls="-m-1 group-hover:bg-bg-hover {selected
								? '!bg-bg-hover'
								: ''}"
							onclick={handleOnStar} />
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<script lang="ts">
	import { getContext } from 'svelte';
	import { fade } from 'svelte/transition';

	let { text = '', iconClass = '' } = $props();

	const sidebar: { isCollapsed: () => boolean } = getContext('sidebar');
	const collapsed = $derived(sidebar.isCollapsed());
</script>

<button
	class="p-1 w-full flex flex-row items-center cursor-pointer gap-2
		text-text-muted bg-bg hover:bg-bg-hover hover:text-text rounded">
	<div class="w-6 flex justify-center shrink-0">
		<i class="{iconClass}text-2xl"></i>
	</div>
	{#if !collapsed}
		<div class="whitespace-nowrap" transition:fade={{ duration: 75 }}>
			{text}
		</div>
	{/if}
</button>

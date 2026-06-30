<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	let {
		icon,
		label,
		onclick,
		class: cls = '',
		...rest
	}: {
		icon: string;
		label: string;
		onclick?: (e: MouseEvent) => void;
		class?: string;
	} & HTMLButtonAttributes = $props();
</script>

<button
	{...rest}
	class="p-1 flex justify-center rounded text-2xl h-min cursor-pointer
		text-text-muted bg-bg hover:text-text {cls} hover:bg-bg-hover"
	aria-label={label}
	{onclick}
	type="button">
	<Icon {icon} />
</button>

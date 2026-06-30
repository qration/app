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
	class="flex h-min cursor-pointer justify-center rounded bg-bg p-1
		text-2xl text-text-muted hover:text-text {cls} hover:bg-bg-hover"
	aria-label={label}
	{onclick}
	type="button">
	<Icon {icon} />
</button>

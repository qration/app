<script lang="ts">
	import Icon from '@iconify/svelte';

	import type { HTMLButtonAttributes } from 'svelte/elements';

	let {
		icon,
		label,
		onclick,
		class: cls = '',
		iconClass,
		...rest
	}: {
		icon: string;
		label: string;
		onclick?: (e: MouseEvent) => void;
		class?: string;
		iconClass?: string;
	} & HTMLButtonAttributes = $props();
</script>

<button
	{...rest}
	class="flex h-min cursor-pointer justify-center rounded bg-bg p-1
		text-2xl text-text-muted hover:text-text {cls} select-none hover:bg-bg-hover"
	aria-label={label}
	{onclick}
	type="button">
	<Icon {icon} class={iconClass} />
</button>

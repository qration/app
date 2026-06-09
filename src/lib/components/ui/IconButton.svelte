<script lang="ts">
	import Icon from '@iconify/svelte';

	let {
		icon,
		label,
		onclick,
		class: cls = '',
	}: {
		icon: string;
		label: string;
		onclick?: (e: MouseEvent) => void;
		class?: string;
	} = $props();
</script>

<button
	class="p-1 flex justify-center rounded text-2xl h-min cursor-pointer
		text-text-muted bg-bg hover:text-text {cls} hover:bg-bg-hover"
	aria-label={label}
	{onclick}>
	<Icon {icon} />
</button>

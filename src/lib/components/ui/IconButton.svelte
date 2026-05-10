<script lang="ts">
	let {
		iconClass,
		label,
		onclick,
    cls = '',
	}: {
		iconClass: string;
		label: string;
		onclick?: (e: MouseEvent) => void;
    cls?: string;
	} = $props();
</script>

<button
	class="{cls}p-1 flex justify-center rounded text-2xl h-min cursor-pointer
		text-text-muted bg-bg hover:text-text hover:bg-bg-hover"
	aria-label={label}
	{onclick}>
	<i class={iconClass}></i>
</button>

<script lang="ts">
	let {
		iconClass,
		label,
		onclick,
	}: {
		iconClass: string;
		label: string;
		onclick?: (e: MouseEvent) => void;
	} = $props();
</script>

<button
	class="py-1 px-2 text-2xl h-min cursor-pointer"
	aria-label={label}
	{onclick}>
	<i class={iconClass}></i>
</button>

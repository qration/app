<script lang="ts">
	import Icon from '@iconify/svelte';

	let {
		text,
		icon,
		onclick,
	}: {
		text: string;
		icon: string;
		onclick?: (e?: MouseEvent) => void;
	} = $props();

	function handleOnClick(e: MouseEvent) {
		e.stopPropagation();
		onclick?.(e);
	}
</script>

<button
	class="flex flex-row items-center gap-2 text-lg w-30 pl-2 pr-4 py-1
		bg-bg-secondary rounded hover:bg-bg-hover cursor-pointer"
	onclick={handleOnClick}>
	<Icon {icon} />
	<div>{text}</div>
</button>

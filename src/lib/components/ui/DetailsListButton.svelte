<script lang="ts">
	import Icon from '@iconify/svelte';

	let {
		text,
		icon,
		iconClass,
		onclick,
	}: {
		text: string;
		icon: string;
		iconClass?: string;
		onclick?: (e?: MouseEvent) => void;
	} = $props();

	function handleOnClick(e: MouseEvent) {
		e.stopPropagation();
		onclick?.(e);
	}
</script>

<button
	class="flex w-30 cursor-pointer flex-row items-center gap-2 rounded bg-bg-secondary py-1
		pr-4 pl-2 text-lg hover:bg-bg-hover"
	onclick={handleOnClick}>
	<Icon {icon} class={iconClass} />
	<div>{text}</div>
</button>

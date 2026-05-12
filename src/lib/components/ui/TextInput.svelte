<script lang="ts">
	import Icon from '@iconify/svelte';
	import IconButton from './IconButton.svelte';

	let {
		placeholder,
		icon = '',
		clear = true,
		cls = '',
		input = $bindable(),
	}: {
		placeholder: string;
		icon?: string;
		clear?: boolean;
		cls?: string;
		input: string;
	} = $props();

	function clearText() {
		input = '';
	}
</script>

<div
	class="{cls}flex w-full flex-row items-center rounded border-2 border-border
		text-xl bg-bg">
	<div class="text-xl absolute pl-3 text-text-secondary">
		<Icon {icon} />
	</div>
	<input
		type="text"
		{placeholder}
		class="w-full rounded border-none bg-transparent px-10 py-2 text-xl
			placeholder:text-text-secondary active:border-none text-text
			focus:outline-none"
		bind:value={input} />
	{#if clear && input != ''}
		<div class="flex items-center justify-end">
			<IconButton
				icon="tabler:x"
				onclick={() => clearText()}
				label="Clear"
				cls="absolute mr-1.5 " />
		</div>
	{/if}
</div>

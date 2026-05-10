<script lang="ts">
	import IconButton from "./IconButton.svelte";

	let {
		placeholder,
		iconClass = '',
		clear = true,
		cls = '',
	}: {
		placeholder: string;
		iconClass?: string;
		clear?: boolean;
		cls?: string;
	} = $props();

	let input = $state('');
	function clearText() {
		input = '';
	}
</script>

<div
	class="{cls}flex w-full flex-row items-center rounded border-2
		border-border text-xl bg-bg">
	<i class="{iconClass}absolute pl-3 text-text-secondary"></i>
	<input
		type="text"
		{placeholder}
		class="w-full rounded border-none bg-transparent px-10 py-2 text-xl
			placeholder:text-text-secondary active:border-none text-text"
		bind:value={input} />
	{#if clear && input != ''}
	<div class="flex items-center justify-end">
		<IconButton iconClass="ti ti-x " onclick={() => clearText()} label="Clear" cls="absolute mr-1.5 "/>
	</div>
		<!-- <button
			class="flex cursor-pointer items-center justify-end"
			aria-label="Clear"
			onclick={() => clearText()}>
			<i class=" absolute px-3 py-3 text-text-secondary hover:text-text"
			></i> -->
		<!-- </button> -->
	{/if}
</div>

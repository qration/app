<script lang="ts">
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
	class="{cls}flex w-full flex-row items-center rounded-xl border-2
		border-border text-xl">
	<i class="{iconClass}absolute pl-3 text-text-secondary"></i>
	<input
		type="text"
		{placeholder}
		class="w-full rounded-xl border-none bg-transparent px-10 py-2 text-xl
			placeholder:text-text-secondary active:border-none text-text"
		bind:value={input} />
	{#if clear && input != ''}
		<button
			class="flex cursor-pointer items-center justify-end"
			aria-label="Clear"
			onclick={() => clearText()}>
			<i class="ti ti-x absolute px-3 py-3 text-text-secondary hover:text-text"
			></i>
		</button>
	{/if}
</div>

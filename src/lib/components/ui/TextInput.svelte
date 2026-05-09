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
		border-gray-300 text-xl">
	<i class="{iconClass}absolute pl-3"></i>
	<input
		type="text"
		{placeholder}
		class="w-full rounded-xl border-none bg-transparent px-10 py-2 text-xl
			placeholder:text-gray-400 active:border-none"
		bind:value={input} />
	{#if clear && input != ''}
		<button
			class="flex cursor-pointer items-center justify-end"
			aria-label="Clear"
			onclick={() => clearText()}>
			<i class="ti ti-x absolute px-3 py-3"></i>
		</button>
	{/if}
</div>

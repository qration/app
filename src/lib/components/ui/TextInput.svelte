<script lang="ts">
	import Icon from '@iconify/svelte';
	import IconButton from './IconButton.svelte';

	let {
		placeholder,
		icon = '',
		clear = true,
		class: cls = '',
		input = $bindable(),
		error = false,
		oninput,
	}: {
		placeholder: string;
		icon?: string;
		clear?: boolean;
		class?: string;
		input: string;
		error?: boolean;
		oninput?: () => void;
	} = $props();

	function clearText() {
		input = '';
	}
</script>

<div
	class="flex {cls} w-full flex-row items-center rounded-lg
		{error
		? 'border-error'
		: 'border-transparent'} border bg-bg-secondary text-xl select-none">
	{#if icon}
		<div class="absolute pl-3 text-xl text-text-secondary">
			<Icon {icon} />
		</div>
	{/if}
	<input
		type="text"
		{placeholder}
		class="w-full rounded-lg border-none bg-bg-secondary py-2
			text-lg text-text placeholder:text-text-secondary focus:outline-none
			active:border-none"
		class:pl-10={icon}
		class:pr-10={clear}
		bind:value={input}
		{oninput} />
	{#if clear && input != ''}
		<div class="flex items-center justify-end">
			<IconButton
				icon="tabler:x"
				onclick={() => clearText()}
				label="Clear"
				class="absolute mr-1.5" />
		</div>
	{/if}
</div>

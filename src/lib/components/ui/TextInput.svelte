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
	class="flex {cls} w-full flex-row items-center rounded border-2
		{error ? 'border-error' : 'border-border'} text-xl bg-bg">
	{#if icon}
		<div class="text-xl absolute pl-3 text-text-secondary">
			<Icon {icon} />
		</div>
	{/if}
	<input
		type="text"
		{placeholder}
		class="w-full rounded border-none bg-transparent py-2 text-lg
			placeholder:text-text-secondary active:border-none text-text
			focus:outline-none"
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

<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		type = 'button',
		display = 'primary',
		disabled = false,
		class: cls = '',
		onclick,
		children,
	}: {
		type?: 'button' | 'submit' | 'reset';
		display?: 'primary' | 'secondary';
		disabled?: boolean;
		class?: string;
		onclick?: (e: MouseEvent) => void;
		children: Snippet;
	} = $props();
</script>

<button
	{type}
	{disabled}
	{onclick}
	class="rounded {cls}
		{display == 'primary'
		? 'bg-button-primary text-bg hover:bg-button-primary-hover'
		: 'bg-button-secondary text-text hover:bg-button-secondary-hover'}
		hover:bg-text-800 cursor-pointer px-4 py-2 disabled:cursor-not-allowed
		disabled:opacity-50">
	{@render children()}
</button>

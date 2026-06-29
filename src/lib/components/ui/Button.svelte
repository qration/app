<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		onclick,
		type = 'button',
		display = 'primary',
		disabled = false,
		class: cls = '',
		children,
	}: {
		onclick?: (e: MouseEvent) => void;
		type?: 'button' | 'submit' | 'reset';
		display?: 'primary' | 'secondary';
		disabled?: boolean;
		class?: string;
		children: Snippet;
	} = $props();
</script>

<button
	{type}
	{disabled}
	{onclick}
	class="rounded {cls}
		{display == 'primary'
		? 'bg-button-primary text-bg'
		: 'bg-button-secondary text-text'}
		px-4 py-2 cursor-pointer hover:bg-text-800 disabled:opacity-50
		disabled:cursor-not-allowed">
	{@render children()}
</button>

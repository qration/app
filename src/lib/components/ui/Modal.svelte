<script lang="ts">
	import type { Snippet } from 'svelte';
	import { fade } from 'svelte/transition';
	import IconButton from './IconButton.svelte';

	let {
		open = $bindable(false),
		title = '',
		onclose,
		children,
	}: {
		open: boolean;
		title?: string;
		onclose?: () => void;
		children: Snippet;
	} = $props();

	function close() {
		open = false;
		onclose?.();
	}

	function onBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) close();
	}

	$effect(() => {
		if (!open) return;
		function onKey(e: KeyboardEvent) {
			if (e.key === 'Escape') close();
		}
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	});
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
		role="presentation"
		onclick={onBackdropClick}
		transition:fade={{ duration: 75 }}>
		<div
			class="relative bg-bg text-text border-2 border-border rounded-xl p-6
				min-w-[24rem] max-w-[90vw] shadow-lg"
			role="dialog"
			aria-modal="true"
			aria-label={title || 'Dialog'}>
			<div class="flex justify-between items-center mb-4">
				<div class="text-xl font-medium">{title}</div>
				<IconButton icon="tabler:x" label="Close" onclick={close} />
			</div>
			{@render children()}
		</div>
	</div>
{/if}

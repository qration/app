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

	let dialog: HTMLDialogElement | null = $state(null);

	function close() {
		open = false;
		dialog?.close();
		onclose?.();
	}

	function onBackdropClick(e: MouseEvent) {
		e.stopPropagation();
		if (e.target === e.currentTarget) close();
	}

	$effect(() => {
		console.log('what');
		if (!open) dialog?.close();
		else dialog?.showModal();
	});
</script>

{#if open}
	<div
		class="w-full h-full fixed inset-0 z-50 flex items-center justify-center bg-black/50 transition backdrop-blur-xs"
		role="presentation"
		onclick={onBackdropClick}
		transition:fade={{ duration: 50 }}>
		<dialog
			bind:this={dialog}
			class="static mx-auto my-auto bg-bg text-text border-2 border-border rounded-xl p-6
				min-w-96 max-w-[90vw] shadow-lg"
			aria-modal="true"
			aria-label={title || 'Dialog'}
			onclose={close}>
			<div class="flex justify-between items-center mb-4">
				<div class="text-xl font-medium">{title}</div>
				<IconButton icon="tabler:x" label="Close" onclick={close} />
			</div>
			{@render children()}
		</dialog>
	</div>
{/if}

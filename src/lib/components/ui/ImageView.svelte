<script lang="ts">
	import IconButton from './IconButton.svelte';

	let {
		open = $bindable(false),
		alt = '',
		src = '',
		onclose,
	}: {
		open: boolean;
		alt?: string;
		src?: string;
		onclose?: () => void;
	} = $props();

	let zoomed = $state(false);
	let dialog: HTMLDialogElement | null = $state(null);

	function close() {
		open = false;
		dialog?.close();
		onclose?.();
	}

	function handleClick(e: MouseEvent) {
		if (!(e.target as HTMLElement).closest('img')) close();
	}

	$effect(() => {
		console.log('what');
		if (!open) dialog?.close();
		else dialog?.showModal();
	});

	function zoom() {
		zoomed = !zoomed;
	}
</script>

<dialog
	bind:this={dialog}
	class="m-0 h-dvh max-h-none w-screen max-w-none overflow-hidden bg-transparent
    pt-safe-top pl-safe-left text-text backdrop:bg-black/50 backdrop:backdrop-blur-xs"
	aria-modal="true"
	aria-label={alt || 'Image'}
	onclose={close}
	onclick={handleClick}
	closedby="any">
	<IconButton
		class="fixed top-4 left-4 z-50 bg-bg-secondary"
		icon="tabler:x"
		label="Close"
		onclick={close} />
	<div class="flex h-full w-full items-center justify-center p-4">
		<img
			{src}
			{alt}
			class="max-h-full {zoomed
				? 'scale-200 cursor-zoom-out'
				: 'scale-100 cursor-zoom-in'} transition-scale duration-200"
			onclick={zoom}
			role="presentation" />
	</div>
</dialog>

<style>
	dialog {
		opacity: 0;
		transform: scale(0.97);
		transition: all 0.1s ease-out allow-discrete;
	}

	dialog::backdrop {
		opacity: 0;
		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		transition: all 0.1s ease-out allow-discrete;
	}

	dialog[open] {
		opacity: 1;
		transform: scale(1);
	}

	dialog[open]::backdrop {
		opacity: 1;
	}

	@starting-style {
		dialog[open] {
			opacity: 0;
			transform: scale(0.97);
			&::backdrop {
				opacity: 0;
			}
		}
	}
</style>

<script lang="ts">
	import IconButton from './IconButton.svelte';
	import type { Snippet } from 'svelte';
	import { untrack } from 'svelte';
	import { pushState } from '$app/navigation';
	import { page } from '$app/state';

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
	let pushedEntry = false;

	function close() {
		open = false;
	}

	function handleClose() {
		open = false;
		onclose?.();
	}

	function reposition() {
		const vv = window.visualViewport;
		if (!dialog || !vv) return;
		const doc = document.documentElement;
		dialog.style.top = `${vv.offsetTop}px`;
		dialog.style.left = `${vv.offsetLeft}px`;
		dialog.style.right = `${doc.clientWidth - vv.offsetLeft - vv.width}px`;
		dialog.style.bottom = `${doc.clientHeight - vv.offsetTop - vv.height}px`;
		dialog.style.maxHeight = `${vv.height}px`;
	}

	$effect(() => {
		if (!dialog) return;

		if (open) {
			if (!dialog.open) dialog.showModal();
			reposition();

			const vv = window.visualViewport;
			vv?.addEventListener('resize', reposition);
			vv?.addEventListener('scroll', reposition);

			return () => {
				vv?.removeEventListener('resize', reposition);
				vv?.removeEventListener('scroll', reposition);
			};
		} else {
			if (dialog.open) dialog.close();
		}
	});

	$effect(() => {
		if (!open) return;

		untrack(() => pushState('', { ...page.state, dialogOpen: true }));
		pushedEntry = true;

		const onPop = () => {
			pushedEntry = false;
			open = false;
		};
		window.addEventListener('popstate', onPop);

		return () => {
			window.removeEventListener('popstate', onPop);
			if (pushedEntry) {
				pushedEntry = false;
				history.back();
			}
		};
	});
</script>

<dialog
	bind:this={dialog}
	class="mx-auto my-auto flex w-120 max-w-[calc(100vw-2rem)] flex-col
		overflow-y-auto rounded-xl border border-border bg-bg p-6 text-text
		shadow-lg select-none
		backdrop:bg-black/50 backdrop:backdrop-blur-xs"
	aria-modal="true"
	aria-label={title || 'Dialog'}
	onclose={handleClose}
	closedby="any">
	<div class="mb-4 flex items-center justify-between">
		<div class="text-xl font-medium">{title}</div>
		<IconButton icon="tabler:x" label="Close" onclick={close} />
	</div>
	{@render children()}
</dialog>

<style>
	dialog {
		opacity: 0;
		transform: scale(0.97);
		transition:
			opacity 0.1s ease-out,
			transform 0.1s ease-out;
	}

	dialog::backdrop {
		opacity: 0;
		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		transition: opacity 0.1s ease-out;
	}

	dialog[open] {
		opacity: 1;
		transform: scale(1);
	}

	dialog[open]::backdrop {
		opacity: 1;
	}

	dialog:not([open]),
	dialog:not([open])::backdrop {
		pointer-events: none;
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

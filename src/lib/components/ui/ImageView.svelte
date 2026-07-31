<script lang="ts">
	import IconButton from './IconButton.svelte';
	import Panzoom, { type PanzoomObject } from '@panzoom/panzoom';

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

	const DELTA = 5;

	let zoomed: boolean = $state(false);
	let mouseStartX: number = $state(0);
	let mouseStartY: number = $state(0);
	let dialog: HTMLDialogElement | null = $state(null);
	let img: HTMLImageElement | null = $state(null);
	let panzoom: PanzoomObject | null = $state(null);

	function close() {
		open = false;
		dialog?.close();
		zoomed = false;
		panzoom?.reset();
		onclose?.();
	}

	function handleClick(e: MouseEvent) {
		if (!(e.target as HTMLElement).closest('img')) close();
	}

	function onWheel(e: WheelEvent) {
		if (!panzoom) return;
		const s = panzoom.getScale();
		if (s <= 1) return;
		e.preventDefault();
		panzoom.pan(-e.deltaX / s, -e.deltaY / s, {
			relative: true,
			animate: false,
		});
	}

	function onPointerUp(e: MouseEvent) {
		let diffX = Math.abs(e.pageX - mouseStartX);
		let diffY = Math.abs(e.pageY - mouseStartY);
		if (diffX < DELTA && diffY < DELTA) {
			if (!zoomed) panzoom?.zoom(2);
			else panzoom?.reset();
			zoomed = !zoomed;
		}
		mouseStartX = 0;
		mouseStartY = 0;
	}

	$effect(() => {
		if (!img) return;
		if (!open) return dialog?.close();
		else dialog?.showModal();
		let pz = Panzoom(img, {
			animate: true,
			maxScale: 2,
			minScale: 1,
			overflow: 'visible',
			contain: 'outside',
			easing: 'ease-in-out',
			cursor: 'zoom-in',
			handleStartEvent: (e) => e.preventDefault(),
		});
		panzoom = pz;

		return () => {
			pz.destroy();
			panzoom = null;
			zoomed = false;
		};
	});
</script>

<dialog
	bind:this={dialog}
	class="m-0 h-dvh max-h-none w-screen max-w-none overflow-hidden bg-transparent
    pt-safe-top pl-safe-left text-text backdrop:bg-black/50 backdrop:backdrop-blur-xs"
	aria-modal="true"
	aria-label={alt || 'Image'}
	onclose={close}
	onpointerup={handleClick}
	closedby="any">
	<IconButton
		class="z-50 mt-4 ml-4 bg-bg-secondary"
		icon="tabler:x"
		label="Close"
		onclick={close} />
	<div
		class="grid h-full w-full place-items-center overflow-hidden"
		onwheel={onWheel}>
		<div class="h-fit w-fit">
			<img
				{src}
				{alt}
				bind:this={img}
				class="block max-h-dvh max-w-full select-none"
				onpointerdown={(e) => {
					mouseStartX = e.pageX;
					mouseStartY = e.pageY;
				}}
				onpointerup={onPointerUp}
				role="presentation" />
		</div>
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

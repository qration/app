<script lang="ts">
	import DetailsListButton from './DetailsListButton.svelte';

	let {
		menuId,
		starred = false,
		onstar,
		ondelete,
		onrefresh,
	}: {
		menuId: string;
		starred?: boolean;
		onstar?: () => void;
		ondelete?: () => void;
		onrefresh?: () => void;
	} = $props();

	function handleOnStar(e: MouseEvent | undefined) {
		e?.stopPropagation();
		onstar?.();
	}

	function handleOnDelete(e: MouseEvent | undefined) {
		e?.stopPropagation();
		ondelete?.();
	}

	function handleOnRefresh(e: MouseEvent | undefined) {
		e?.stopPropagation();
		onrefresh?.();
	}
</script>

<div
	id={menuId}
	popover
	class="m-0 w-max rounded-lg border-2 border-border bg-bg-secondary p-2"
	role="none"
	onclick={(e) => e.stopPropagation()}
	style="position-anchor: auto; position-area: bottom span-right; position-visibility: anchors-visible
    position-try-fallbacks: flip-block;">
	<div class="flex flex-col">
		<DetailsListButton
			icon={starred ? 'tabler:star-filled' : 'tabler:star'}
			text={starred ? 'Unstar' : 'Star'}
			onclick={handleOnStar} />
		<DetailsListButton
			icon="tabler:reload"
			text="Refresh"
			onclick={handleOnRefresh} />
		<DetailsListButton
			icon="tabler:trash"
			text="Delete"
			onclick={handleOnDelete} />
	</div>
</div>

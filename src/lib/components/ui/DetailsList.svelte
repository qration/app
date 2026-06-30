<script lang="ts">
	import DetailsListButton from './DetailsListButton.svelte';

	let {
		menuId,
		starred = false,
		onstar,
	}: {
		menuId: string;
		starred?: boolean;
		onstar?: () => void;
	} = $props();

	function handleOnStar(e: MouseEvent | undefined) {
		e?.stopPropagation();
		onstar?.();
	}
</script>

<div
	id={menuId}
	popover
	class="m-0 w-max bg-bg-secondary rounded-lg p-2 border-2 border-border"
	role="none"
	onclick={(e) => e.stopPropagation()}
	style="position-anchor: auto; position-area: bottom span-right; position-visibility: anchors-visible
    position-try-fallbacks: flip-block;">
	<div class="flex flex-col">
		<DetailsListButton
			icon={starred ? 'tabler:star-filled' : 'tabler:star'}
			text={starred ? 'Unstar' : 'Star'}
			onclick={handleOnStar} />
		<DetailsListButton icon="tabler:trash" text="Delete" />
	</div>
</div>

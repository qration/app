<script lang="ts">
	import DetailsListButton from './DetailsListButton.svelte';
	import { commands, type Feed } from '$lib/util/bindings';

	let {
		menuId,
		feed,
		starred = false,
		onrefresh,
		ondelete,
	}: {
		menuId: string;
		feed?: Feed;
		starred?: boolean;
		onrefresh?: () => Promise<void>;
		ondelete?: () => void;
	} = $props();

	let feedRefreshing = $state(false);

	async function handleOnStar(e: MouseEvent | undefined) {
		e?.stopPropagation();
		if (feed) {
			await commands.setStarFeed(feed.id, !feed.favourited);
			feed.favourited = !feed.favourited;
		}
	}

	function handleOnDelete(e: MouseEvent | undefined) {
		e?.stopPropagation();
		ondelete?.();
	}

	async function handleOnRefresh(e: MouseEvent | undefined) {
		e?.stopPropagation();
		console.log('what');
		if (feed && !feedRefreshing) {
			console.log(feed.id);
			feedRefreshing = true;
			await commands.refreshFeed(feed.id);
			await onrefresh?.();
			feedRefreshing = false;
			console.log(feed.id, 'done');
		}
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
			onclick={handleOnRefresh}
			iconClass={feedRefreshing ? 'animate-spin' : ''} />
		<DetailsListButton
			icon="tabler:trash"
			text="Delete"
			onclick={handleOnDelete} />
	</div>
</div>

<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';

	import { feedStore } from '$lib/stores/feeds.svelte';

	import type { Feed } from '$lib/util/bindings';

	let {
		open = $bindable(false),
		feed,
	}: {
		open: boolean;
		feed: Feed | undefined;
	} = $props();

	async function deleteFeed() {
		if (feed) {
			feedStore.data.feeds = feedStore.data.feeds.filter(
				(f) => f.id != feed.id,
			);
			feedStore.data.articles = feedStore.data.articles.filter(
				(a) => a.feed_id != feed.id,
			);
		}
		open = false;
	}

	function cancelDelete() {
		open = false;
	}
</script>

{#if feed}
	<Modal bind:open title="Delete {feed.name}">
		<div class="flex flex-col gap-2">
			<div>
				Are you sure you want to delete <span class="font-bold"
					>{feed.name}</span> from your feeds? This will remove all its articles as
				well!
			</div>
			<div class="flex flex-row gap-1 self-end">
				<Button onclick={cancelDelete} display="secondary">Cancel</Button>
				<Button onclick={deleteFeed}>Delete</Button>
			</div>
		</div>
	</Modal>
{/if}

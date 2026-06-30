<script lang="ts">
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import ExternalLink from './dialogs/ExternalLink.svelte';

	import { openUrl } from '@tauri-apps/plugin-opener';
	import { feedStore } from '$lib/stores/feeds.svelte';

	import type { Article, Feed } from '$lib/util/bindings';

	let { articleId }: { articleId: string } = $props();
	let article: Article | undefined = $derived(
		feedStore.data.articles.find((a) => a.id == articleId),
	);
	let feed: Feed | undefined = $derived(
		feedStore.data.feeds.find((f) => f.id == article?.feed_id),
	);

	let externalLinkOpen = $state(false);
	let href = $state('');

	function handleLinkClick(e: MouseEvent) {
		const a = (e.target as HTMLElement).closest('a');
		if (!a) return;

		href = a.getAttribute('href') || '';
		href = href.replace(/\s+/g, '');
		if (!href) return;
		externalLinkOpen = true;
		e.preventDefault();
	}
</script>

{#if article && feed}
	<div class="flex h-full w-full min-w-0 flex-col">
		<div
			class="flex max-w-full min-w-0 shrink-0 flex-row items-center
				justify-between border-b-2 border-border p-2">
			<div
				class="flex max-w-full min-w-0 flex-row items-center px-2 text-2xl
					font-medium text-text">
				<span class="min-w-0">{article.name}</span>
				<span
					class="shrink-0 px-2 font-light whitespace-nowrap text-text-secondary"
					>&#8729;</span>
				<span class="shrink-0 font-light whitespace-nowrap text-text-secondary"
					>{feed.name}</span>
			</div>
			<div class="flex flex-row justify-end gap-1">
				<IconButton
					icon="tabler:bookmark{article.saved ? '-filled' : ''}"
					label="Save {article.name}"
					onclick={() => (article.saved = !article.saved)} />
				{#if article.url}
					<IconButton
						icon="tabler:external-link"
						label="Open {article.name}"
						onclick={async () => await openUrl(article.url!)} />
				{/if}
			</div>
		</div>
		<div class="flex h-full w-full flex-col gap-2 overflow-y-scroll px-10 py-5">
			<!-- {#if article.media_type == 'video'}
				<iframe
					title={article.name}
					src={article.enclosure!.url}
					width="100%"
					height="allowfullscreen"
					frameborder="0"
					class="aspect-video"></iframe>
			{/if} -->
			<div
				onclick={handleLinkClick}
				role="presentation"
				class="feed-content h-min">
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				{@html article.content}
			</div>
		</div>
	</div>
{/if}

<ExternalLink bind:open={externalLinkOpen} {href} />

<style>
	.feed-content :global(img) {
		display: block;
		margin: 0 auto;
		max-width: 80%;
		padding: 0.5rem 0rem;
	}

	.feed-content :global(p) {
		padding: 0.5rem 0rem;
	}

	.feed-content :global(a) {
		text-decoration: underline;
	}

	.feed-content :global(a:hover) {
		color: var(--color-text-secondary);
	}
</style>

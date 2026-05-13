<script lang="ts">
	import type { Article, Feed } from '$lib/util/interfaces';
	import IconButton from '../ui/IconButton.svelte';

	let {
		articleId,
		data,
	}: { articleId: string; data: { feeds: Feed[]; articles: Article[] } } =
		$props();
	let article: Article | undefined = $derived(
		data.articles.find((a) => a.id == articleId),
	);
	let feed: Feed | undefined = $derived(
		data.feeds.find((f) => f.id == article?.feed_id),
	);
</script>

{#if article && feed}
	<div class="flex flex-col w-full h-full">
		<div
			class="flex flex-row justify-between items-center border-b-2 border-border p-2 shrink-0">
			<div class="text-text font-medium pl-2 text-2xl">
				{article.name}
				<span class="font-light text-text-secondary">&#8729; {feed.name}</span>
			</div>
			<div class="flex flex-row justify-end gap-1">
				<IconButton
					icon="tabler:bookmark{article.saved ? '-filled' : ''}"
					label="Save {article.name}"
					onclick={() => (article.saved = !article.saved)} />
				<IconButton
					icon="tabler:external-link"
					label="Open {article.name}"
					onclick={() => window.open(article.url, '_blank')} />
			</div>
		</div>
		<div class="p-4 w-full h-full flex flex-col gap-2 overflow-y-scroll">
			{#if article.media_type == 'video'}
				<iframe
					title={article.name}
					src={article.media_url}
					width="100%"
					height="allowfullscreen"
					frameborder="0"
					class="aspect-video"></iframe>
			{/if}
			{article.content}
		</div>
	</div>
{/if}

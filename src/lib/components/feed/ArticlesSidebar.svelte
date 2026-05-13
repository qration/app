<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import ArticleButton from '$lib/components/ui/ArticleButton.svelte';
	import { filterArticles, getFeedIcon } from '$lib/util/util';
	import type { Article, Feed } from '$lib/util/interfaces';

	let {
		filter,
		onarticleselect,
		data,
	}: {
		filter: string;
		onarticleselect: (article: string) => void;
		data: { feeds: Feed[]; articles: Article[] };
	} = $props();
	let selectedArticle = $state('');
	let search = $state('');

	let filteredArticles = $derived(
		filterArticles(data.feeds, data.articles, filter).filter((a) =>
			a.name.toLowerCase().includes(search.toLowerCase()),
		),
	);
</script>

<div
	class="flex flex-col gap bg-bg text-text text-4xl overflow-y-scroll
		justify-start h-full border-border border-r-2 shrink-0 w-100">
	<div
		class="sticky top-0 bg-bg py-4 w-full px-4 border-2 border-x-transparent
			border-t-transparent border-b-border">
		<TextInput
			placeholder="Search..."
			icon="tabler:search"
			bind:input={search} />
	</div>
	<div
		class="flex flex-col gap-4 bg-bg text-text text-4xl overflow-y-scroll
			justify-start h-full p-4">
		{#each filteredArticles as article (article.id)}
			{@const articleFeed = data.feeds.find((f) => f.id == article.feed_id)}
			{#if articleFeed}
				<ArticleButton
					title={article.name}
					author={articleFeed.name}
					icon={getFeedIcon(
						data.feeds.find((f) => f.id == article.feed_id)!.type,
					)}
					timestamp={article.timestamp}
					selected={selectedArticle == article.id}
					onclick={() => {
						selectedArticle = article.id;
						onarticleselect(selectedArticle);
					}} />
			{/if}
		{/each}
	</div>
</div>

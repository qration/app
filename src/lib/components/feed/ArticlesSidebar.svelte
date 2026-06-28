<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import ArticleButton from '$lib/components/ui/ArticleButton.svelte';
	import { dateStrParse, filterArticles, getFeedIcon } from '$lib/util/util';
	import { feedStore } from '$lib/stores/feeds.svelte';

	let {
		filter,
		onarticleselect,
	}: {
		filter: string;
		onarticleselect: (article: string) => void;
	} = $props();
	let selectedArticle = $state('');
	let search = $state('');

	let filteredArticles = $derived(
		filterArticles(feedStore.data.feeds, feedStore.data.articles, filter)
			.filter((a) => a.name.toLowerCase().includes(search.toLowerCase()))
			.sort((a, b) => dateStrParse(b.date) - dateStrParse(a.date)),
	);
</script>

<div
	class="flex flex-col gap bg-bg text-text text-4xl justify-start h-full
		border-border border-r-2 shrink-0 w-100">
	<div
		class="sticky top-0 py-4 w-full px-4 border-2 border-x-transparent
			border-t-transparent border-b-border">
		<TextInput
			placeholder="Search..."
			icon="tabler:search"
			bind:input={search} />
	</div>
	<div
		class="flex flex-col gap-4 py-4 px-2 text-text text-4xl overflow-y-scroll
			justify-start h-full scrollbar-gutter-both">
		{#each filteredArticles as article (article.id)}
			{@const articleFeed = feedStore.data.feeds.find(
				(f) => f.id == article.feed_id,
			)}
			{#if articleFeed}
				<ArticleButton
					{article}
					author={articleFeed.name}
					icon={getFeedIcon(
						feedStore.data.feeds.find((f) => f.id == article.feed_id)!
							.feed_type,
					)}
					selected={selectedArticle == article.id}
					onclick={() => {
						article.read = true;
						selectedArticle = article.id;
						onarticleselect(selectedArticle);
					}} />
			{/if}
		{/each}
	</div>
</div>

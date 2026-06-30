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
			.filter((a) =>
				a.name ? a.name.toLowerCase().includes(search.toLowerCase()) : false,
			)
			.sort((a, b) =>
				a.date && b.date ? dateStrParse(b.date) - dateStrParse(a.date) : 0,
			),
	);
</script>

<div
	class="gap flex h-full w-100 shrink-0 flex-col justify-start border-r-2
		border-border bg-bg text-4xl text-text">
	<div
		class="sticky top-0 w-full border-2 border-x-transparent border-t-transparent border-b-border
			px-4 py-4">
		<TextInput
			placeholder="Search..."
			icon="tabler:search"
			bind:input={search} />
	</div>
	<div
		class="flex h-full flex-col justify-start gap-4 overflow-y-scroll p-4
			text-4xl text-text">
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

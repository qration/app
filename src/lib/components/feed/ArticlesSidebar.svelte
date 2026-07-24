<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import ArticleButton from '$lib/components/ui/ArticleButton.svelte';

	import { filterArticles, getFeedIcon } from '$lib/util/util';
	import IconButton from '../ui/IconButton.svelte';
	import { getFeedStore, getMQ } from '$lib/context/context.svelte';

	let {
		filter,
		inert,
		onarticleselect,
		onsidebarcollapse,
	}: {
		filter: string;
		inert: boolean;
		onarticleselect: (article: string) => void;
		onsidebarcollapse: () => void;
	} = $props();

	const feedStore = getFeedStore();
	const MQ = getMQ();

	let selectedArticle = $state('');
	let search = $state('');

	let filteredArticles = $derived(
		filterArticles(feedStore.feeds, feedStore.articles_light, filter)
			.filter((a) =>
				a.article_name
					? a.article_name.toLowerCase().includes(search.toLowerCase())
					: false,
			)
			.sort((a, b) =>
				a.article_date && b.article_date ? b.article_date - a.article_date : 0,
			),
	);
</script>

<div
	class="gap flex h-full min-h-0 {MQ.current
		? 'w-100'
		: 'w-full'} shrink-0 touch-none flex-col justify-start
		border-r-2 border-border bg-bg text-4xl text-text"
	{inert}>
	<div
		class="sticky top-0 flex w-full flex-row
			items-center gap-4 border-b-2 border-border px-4 py-4">
		<IconButton
			icon="tabler:layout-sidebar-left-expand"
			label="Uncollapse"
			onclick={() => onsidebarcollapse()}
			class={MQ.current ? 'hidden' : ''} />
		<TextInput
			placeholder="Search..."
			icon="tabler:search"
			bind:input={search} />
	</div>
	<div
		class="flex h-full flex-col justify-start gap-4 overflow-y-scroll
			p-4 text-4xl text-text">
		{#each filteredArticles as article (article.id)}
			{@const articleFeed = feedStore.feeds.find(
				(f) => f.id == article.feed_id,
			)}
			{#if articleFeed}
				<ArticleButton
					{article}
					author={articleFeed.feed_name}
					icon={getFeedIcon(
						feedStore.feeds.find((f) => f.id == article.feed_id)!.feed_type,
					)}
					selected={selectedArticle == article.id}
					onclick={() => {
						article.article_read = true;
						selectedArticle = article.id;
						onarticleselect(selectedArticle);
					}} />
			{/if}
		{/each}
	</div>
</div>

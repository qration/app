<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Article from '../ui/Article.svelte';
	import data from '$lib/assets/test-data.json';
	import { filteredArticles, getFeedIcon } from '$lib/util/util';

	let {
		filter,
		onarticleselect,
	}: { filter: string; onarticleselect: (article: string) => void } = $props();
	let selectedArticle = $state('');
	let search = $state('');
</script>

<div
	class="flex flex-col gap bg-bg text-text text-4xl overflow-y-scroll
		justify-start h-full border-border border-r-2">
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
		{#each filteredArticles(data.feeds, data.articles, filter).filter( (a) => a.name.includes(search), ) as article (article.id)}
			<Article
				title={article.name}
				author={data.feeds.find((f) => f.id == article.feed_id)!.name}
				icon={getFeedIcon(
					data.feeds.find((f) => f.id == article.feed_id)!.type,
				)}
				timestamp={article.timestamp}
				selected={selectedArticle == article.id}
				onclick={() => {
					selectedArticle = article.id;
					onarticleselect(selectedArticle);
				}} />
		{/each}
	</div>
</div>

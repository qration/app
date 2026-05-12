<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Article from '../ui/Article.svelte';

	let selectedArticle = $state('');
</script>

<div
	class="flex flex-col gap bg-bg text-text text-4xl overflow-y-scroll
		justify-start h-full border-border border-r-2">
	<div
		class="sticky top-0 bg-bg py-4 w-full px-4 border-2 border-x-transparent
			border-t-transparent border-b-border">
		<TextInput placeholder="Search..." icon="tabler:search" />
	</div>
	<div
		class="flex flex-col gap-4 bg-bg text-text text-4xl overflow-y-scroll
			justify-start h-full p-4">
		<Article
			title="Article 1"
			author="Feed 1"
			icon="tabler:rss"
			time={new Date()}
			selected={selectedArticle == 'article-1'}
			onclick={() => (selectedArticle = 'article-1')} />
		<Article
			title="Article 2"
			author="Feed 2"
			icon="tabler:brand-youtube"
			time={new Date()}
			selected={selectedArticle == 'article-2'}
			onclick={() => (selectedArticle = 'article-2')} />
	</div>
</div>

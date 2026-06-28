<script lang="ts">
	import ArticlesSidebar from '$lib/components/feed/ArticlesSidebar.svelte';
	import Sidebar from '$lib/components/feed/Sidebar.svelte';
	import ArticleView from '$lib/components/feed/ArticleView.svelte';

	let articleFilter = $state('feed-all');
	let selectedArticleId = $state('');

	function filterChange(filter: string) {
		articleFilter = filter;
	}

	function articleSelect(articleId: string) {
		selectedArticleId = articleId;
	}
</script>

<div class="flex h-screen w-screen flex-row bg-bg">
	<Sidebar onfilterchange={filterChange} />
	<ArticlesSidebar filter={articleFilter} onarticleselect={articleSelect} />
	<ArticleView articleId={selectedArticleId} />
	<!-- <h1>Welcome to SvelteKit</h1>
	<p>
		Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read
		the documentation
	</p> -->
</div>

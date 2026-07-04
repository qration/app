<script lang="ts">
	import ArticlesSidebar from '$lib/components/feed/ArticlesSidebar.svelte';
	import Sidebar from '$lib/components/feed/Sidebar.svelte';
	import ArticleView from '$lib/components/feed/ArticleView.svelte';

	let articleFilter = $state('feed-all');
	let selectedArticleId = $state('');
	let sidebarCollapsed = $state(true);
	let articleOpen = $state(false);

	function filterChange(filter: string) {
		articleFilter = filter;
	}

	function articleSelect(articleId: string) {
		selectedArticleId = articleId;
		articleOpen = true;
	}

	function closeArticle() {
		articleOpen = false;
	}

	function closeArticleEnd() {
		selectedArticleId = '';
	}

	function collapseSidebar() {
		sidebarCollapsed = !sidebarCollapsed;
	}
</script>

<div class="relative flex h-screen w-full flex-row overflow-hidden bg-bg">
	{#if !sidebarCollapsed}
		<div
			class="fixed inset-0 bg-black/50 lg:hidden"
			onclick={collapseSidebar}
			role="presentation"
			aria-hidden="true">
		</div>
	{/if}
	<Sidebar
		onfilterchange={filterChange}
		collapsed={sidebarCollapsed}
		oncollapse={collapseSidebar} />
	<ArticlesSidebar
		filter={articleFilter}
		onarticleselect={articleSelect}
		onsidebarcollapse={collapseSidebar} />
	<ArticleView
		articleId={selectedArticleId}
		isOpen={articleOpen}
		onclose={closeArticle}
		ontransitionend={closeArticleEnd} />
	<!-- <h1>Welcome to SvelteKit</h1>
	<p>
		Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read
		the documentation
	</p> -->
</div>

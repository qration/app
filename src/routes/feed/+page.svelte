<script lang="ts">
	import ArticlesSidebar from '$lib/components/feed/ArticlesSidebar.svelte';
	import Sidebar from '$lib/components/feed/Sidebar.svelte';
	import ArticleView from '$lib/components/feed/ArticleView.svelte';
	import { onMount, tick } from 'svelte';
	import { setMQ, setFeedStore, FeedStore } from '$lib/context/context.svelte';
	import { MediaQuery } from 'svelte/reactivity';

	let articleFilter = $state('feed-all');
	let selectedArticleId = $state('');
	let sidebarCollapsed = $state(true);
	let articleOpen = $state(false);

	const MQ = new MediaQuery('width >= 64rem', true);
	const feedStore = setFeedStore(new FeedStore());
	setFeedStore(feedStore);
	setMQ(MQ);

	let ac: HTMLDivElement | null = $state(null);

	function filterChange(filter: string) {
		articleFilter = filter;
	}

	async function articleSelect(articleId: string) {
		selectedArticleId = articleId;
		articleOpen = true;
		await tick();
		ac?.focus({ preventScroll: true });
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

	onMount(() => {
		feedStore.load();
		feedStore.refreshAll();
		const interval = setInterval(
			async () => await feedStore.refreshAll(),
			15 * 60000,
		);
		return () => clearInterval(interval);
	});

	const onFocus = () => void feedStore.refreshAll();
	window.addEventListener('focus', onFocus);
</script>

<div class="relative flex h-full w-full flex-row overflow-hidden bg-bg p-0">
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
		onsidebarcollapse={collapseSidebar}
		inert={!MQ.current && (!sidebarCollapsed || articleOpen)} />
	<ArticleView
		bind:articleContainer={ac}
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

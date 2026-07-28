<script lang="ts">
	import ArticlesSidebar from '$lib/components/feed/ArticlesSidebar.svelte';
	import Sidebar from '$lib/components/feed/Sidebar.svelte';
	import ArticleView from '$lib/components/feed/ArticleView.svelte';
	import { onMount, tick } from 'svelte';
	import { setMQ, setFeedStore, FeedStore } from '$lib/context/context.svelte';
	import { MediaQuery } from 'svelte/reactivity';
	import {
		onNotificationClicked,
		requestPermission,
	} from '@choochmeque/tauri-plugin-notifications-api';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import 'overlayscrollbars/overlayscrollbars.css';
	import { pushState, replaceState } from '$app/navigation';
	import { page } from '$app/state';

	let articleFilter = $state('feed-all');
	let selectedArticleId = $derived(page.state.selectedArticleId || '');
	let sidebarCollapsed = $derived(!page.state.showSidebar);
	let articleOpen = $derived(page.state.showArticle || false);

	const MQ = new MediaQuery('width >= 64rem', true);
	const feedStore = setFeedStore(new FeedStore());
	setFeedStore(feedStore);
	setMQ(MQ);

	let ac: HTMLDivElement | null = $state(null);

	function filterChange(filter: string) {
		articleFilter = filter;
	}

	async function articleSelect(articleId: string) {
		articleOpen = true;
		if (selectedArticleId != articleId) {
			pushState('', {
				...page.state,
				selectedArticleId: articleId,
				showArticle: true,
			});
		}
		await tick();
		ac?.focus({ preventScroll: true });
	}

	function closeArticle() {
		replaceState('', { ...page.state, selectedArticleId, showArticle: false });
	}

	function closeArticleEnd() {
		selectedArticleId = '';
		history.back();
	}

	function collapseSidebar() {
		if (sidebarCollapsed) {
			pushState('', { ...page.state, showSidebar: true });
		} else {
			history.back();
		}
	}

	onMount(() => {
		feedStore.load();
		feedStore.refreshAll();
		const interval = setInterval(
			async () => await feedStore.refreshAll(),
			15 * 60000,
		);

		let unlisten: () => Promise<void> | null;
		onNotificationClicked(async () => {
			await getCurrentWindow().setFocus();
		}).then((u) => (unlisten = u.unregister));

		requestPermission().then((p) => {
			feedStore.notificationPermsGranted = p == 'granted';
		});

		return async () => {
			clearInterval(interval);
			await unlisten?.();
		};
	});

	const onFocus = () => void feedStore.refreshAll();
	window.addEventListener('focus', onFocus);
</script>

<div class="relative flex h-full w-full flex-row overflow-hidden bg-bg p-0">
	{#if !sidebarCollapsed}
		<div
			class="fixed inset-0 z-49 bg-black/50 lg:hidden"
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
		{selectedArticleId}
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

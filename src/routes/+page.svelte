<script>
	import ArticlesSidebar from '$lib/components/feed/ArticlesSidebar.svelte';
	import Sidebar from '$lib/components/feed/Sidebar.svelte';
</script>

<div class="flex h-screen flex-row bg-bg">
	<Sidebar />
	<ArticlesSidebar />
	<div class="p-4">
		<h1>Welcome to SvelteKit</h1>
		<p>
			Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read
			the documentation
		</p>
	</div>
</div>

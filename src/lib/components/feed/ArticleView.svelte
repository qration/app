<script lang="ts">
	import type { Article } from '$lib/util/interfaces';
	// import IconButton from '../ui/IconButton.svelte';

	let {
		articleId,
		data,
	}: { articleId: string; data: { articles: Article[] } } = $props();
	let article: Article | undefined = $derived(
		data.articles.find((a) => a.id == articleId),
	);
</script>

{#if article}
	<div class="flex flex-col">
		<div class="flex flex-row">
			<!-- <IconButton -->
		</div>
		{article.content}
	</div>
{/if}

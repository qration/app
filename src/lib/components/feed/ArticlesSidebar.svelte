<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
	import Article from '../ui/Article.svelte';
</script>

<div
	class="flex flex-col justify-start h-full border-border border-r-2 py-3
		px-4">
	<div
		class="flex flex-col gap-4 border-border bg-bg text-4xl text-text">

		<TextInput placeholder="Search..." iconClass="ti ti-search " />
		<Article title="Article 1" author="Feed 1" iconClass="ti ti-rss " time={new Date()}/>
		<Article title="Article 2" author="Feed 2" iconClass="ti ti-brand-youtube " time={new Date()}/>
	</div>
</div>

<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();

	import { addCollection } from '@iconify/svelte';
	import { icons } from '@iconify-json/tabler';

	// TODO: add only the necessary icons here
	addCollection(icons);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}

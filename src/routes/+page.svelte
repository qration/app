<script lang="ts">
	import Logo from '$lib/assets/favicon.svg?component';
	import { resolve } from '$app/paths';
</script>

<div class="align-center p-safe flex h-full w-full justify-center">
	<Logo />
	<a href={resolve('/feed')}>Go to your feed</a>
</div>

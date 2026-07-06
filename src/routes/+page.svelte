<script lang="ts">
	import logo from '$lib/assets/qration.svg';
	import { resolve } from '$app/paths';
</script>

<div class="align-center p-safe flex h-full w-full justify-center">
	<img src={logo} alt="Logo" />
	<a href={resolve('/feed')}>Go to your feed</a>
</div>

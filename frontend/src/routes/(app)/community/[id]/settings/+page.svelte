<script lang="ts">
	import type { PageProps } from './$types';
	import DangerZone from './DangerZone.svelte';
	import InfoZone from './InfoZone.svelte';
	import ManageAccessZone from './ManageAccessZone.svelte';

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div class="grid gap-4">
		<InfoZone community={data.community} />
		<ManageAccessZone participants={data.participants} />
		<DangerZone community={data.community} />
	</div>
</div>

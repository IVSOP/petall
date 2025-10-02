<script lang="ts">
	import DangerZone from '$lib/components/DangerZone.svelte';
	import InfoZone from '$lib/components/InfoZone.svelte';
	import ManageAccessZone from '$lib/components/ManageAccessZone.svelte';
	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div class="grid gap-4">
		<InfoZone community={data.community} />
		<ManageAccessZone participants={data.participants} />
		<DangerZone community={data.community} />
	</div>
</div>

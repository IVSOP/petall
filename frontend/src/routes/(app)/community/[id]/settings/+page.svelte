<script lang="ts">
	import type { PageProps } from './$types';
	import InfoZone from '../../../../../components/InfoZone.svelte';
	import DangerZone from '../../../../../components/DangerZone.svelte';
	import ManageAccessZone from '../../../../../components/ManageAccessZone.svelte';

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div class="grid gap-4">
		<InfoZone community={data.community} />
		<ManageAccessZone participants={data.participants} />
		<DangerZone community={data.community} />
	</div>
</div>

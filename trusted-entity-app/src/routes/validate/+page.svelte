<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<div>
    <p>{data.proof}</p>

    <p>{data.energyRecordCost}</p>
</div>


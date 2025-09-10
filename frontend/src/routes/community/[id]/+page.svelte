<script>
	import EnergyChart from '../../../components/EnergyChart.svelte';

	const { data } = $props();
</script>

<p>
	<span class="font-medium">Comunidade:</span>
	{JSON.stringify(data.community)}
</p>
<p>
	<span class="font-medium">Energia:</span>
	{JSON.stringify(data.energyTransfers)}
</p>

<div class="container">
	<EnergyChart></EnergyChart>

	<div class="font-medium">Informações Diárias</div>
</div>

<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';

	const { community } = $props();

    //TODO: remove this when real images are added
	function generateColorsFromUUID(uuid: string) {
		const cleanUuid = uuid.replace(/-/g, '');
		
		function simpleHash(str: string, seed = 0): number {
			let hash = seed;
			for (let i = 0; i < str.length; i++) {
				hash = Math.imul(31, hash) + str.charCodeAt(i) | 0;
			}
			return Math.abs(hash);
		}

		const hash1 = simpleHash(cleanUuid, 1);
		const hash2 = simpleHash(cleanUuid, 2);
		const hash3 = simpleHash(cleanUuid, 3);

		const hue1 = hash1 % 360;
		const hue2 = (hash2 % 120) + hue1;
		
		const saturation = 60 + (hash3 % 30);
		const lightness = 50 + (hash1 % 20); 

		return {
			color1: `hsl(${hue1}, ${saturation}%, ${lightness}%)`,
			color2: `hsl(${hue2 % 360}, ${saturation}%, ${lightness + 10}%)`
		};
	}

	const colors = generateColorsFromUUID(community.image);
	const gradientStyle = `background: linear-gradient(135deg, ${colors.color1}, ${colors.color2});`;
	
	function getInitials(entity: string | undefined): string {
		if (!entity || typeof entity !== 'string') return '??';
		return entity.substring(0, 2).toUpperCase();
	}
</script>

<a
	href={`/community/${community.id}`}
	class="block transform rounded-md transition-transform duration-300 hover:scale-102 hover:shadow-lg"
>
	<Card.Root>
		<Card.Header>
			<Card.Title>{community.entity || 'Unnamed Community'}</Card.Title>
			<Card.Description>Energy Community</Card.Description>
		</Card.Header>
		<Card.Content>
			<div 
				class="w-full h-32 rounded-lg mb-4 flex items-center justify-center text-white font-semibold text-lg shadow-inner"
				style={gradientStyle}
			>
				{getInitials(community.entity)}
			</div>
			<p>ID: {community.id}</p>
		</Card.Content>
		<Card.Footer>
			<p class="text-sm text-muted-foreground">
				Last Update: {new Date().toLocaleDateString()}
			</p>
		</Card.Footer>
	</Card.Root>
</a>
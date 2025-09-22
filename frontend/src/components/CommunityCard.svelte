<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import BadgeCheckIcon from '@lucide/svelte/icons/badge-check';
	import * as Card from '$lib/components/ui/card/index.js';

	const { community } = $props();

	//TODO: remove this when real images are added
	function generateColorsFromUUID(uuid: string) {
		const cleanUuid = uuid.replace(/-/g, '');

		function simpleHash(str: string, seed = 0): number {
			let hash = seed;
			for (let i = 0; i < str.length; i++) {
				hash = (Math.imul(31, hash) + str.charCodeAt(i)) | 0;
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
	class="block transform rounded-b-xl transition-transform duration-300 hover:scale-102 hover:shadow-xl"
>
	<Card.Root class="gap-3 overflow-hidden rounded-xl pt-0">
		<Card.Header class="px-0 py-0">
			<div
				class="flex h-48 w-full items-center justify-center overflow-hidden rounded-t-xl shadow-inner"
			>
				<img
					src="https://media.istockphoto.com/id/1284536377/photo/solar-panels-on-field-in-summer-aerial-view-of-poland.jpg?s=612x612&w=0&k=20&c=CaH_vkg5gwZl8QzQTQYHhAqm0G5jEqhuuGR6V0l3wWc="
					alt={community.name}
					class="h-full w-full object-cover"
				/>
			</div>
		</Card.Header>
		<Card.Content class="px-4 ">
			<div class="flex justify-between">
				<div class="truncate text-lg font-semibold text-gray-900">
					{community.name}
				</div>
				<Badge>User</Badge>
			</div>
		</Card.Content><Card.Content class="px-4">
			<div class="grid grid-cols-3 gap-4 text-center">
				<div class="flex flex-col items-center">
					<div
						class="mb-2 flex h-16 w-16 cursor-pointer items-center justify-center rounded-full bg-gray-100"
						title="Members"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 640 512"
							class="h-10 w-10 text-gray-600"
						>
							<path
								d="M320 16a104 104 0 1 1 0 208 104 104 0 1 1 0-208zM96 88a72 72 0 1 1 0 144 72 72 0 1 1 0-144zM0 416c0-70.7 57.3-128 128-128 12.8 0 25.2 1.9 36.9 5.4-32.9 36.8-52.9 85.4-52.9 138.6l0 16c0 11.4 2.4 22.2 6.7 32L32 480c-17.7 0-32-14.3-32-32l0-32zm521.3 64c4.3-9.8 6.7-20.6 6.7-32l0-16c0-53.2-20-101.8-52.9-138.6 11.7-3.5 24.1-5.4 36.9-5.4 70.7 0 128 57.3 128 128l0 32c0 17.7-14.3 32-32 32l-86.7 0zM472 160a72 72 0 1 1 144 0 72 72 0 1 1 -144 0zM160 432c0-88.4 71.6-160 160-160s160 71.6 160 160l0 16c0 17.7-14.3 32-32 32l-256 0c-17.7 0-32-14.3-32-32l0-16z"
							/>
						</svg>
					</div>
					<p class="text-sm font-medium text-gray-700">13</p>
				</div>
				<div class="flex flex-col items-center">
					<div
						class="mb-2 flex h-16 w-16 cursor-pointer items-center justify-center rounded-full bg-gray-100"
						title="Managers"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 576 512"
							class="h-10 w-10 text-gray-600"
						>
							<path
								d="M302.3-12.6c-9-4.5-19.6-4.5-28.6 0l-256 128C1.9 123.3-4.5 142.5 3.4 158.3s27.1 22.2 42.9 14.3L288 51.8 529.7 172.6c15.8 7.9 35 1.5 42.9-14.3s1.5-35-14.3-42.9l-256-128zM288 272a56 56 0 1 0 0-112 56 56 0 1 0 0 112zm0 48c-53 0-96 43-96 96l0 32c0 17.7 14.3 32 32 32l128 0c17.7 0 32-14.3 32-32l0-32c0-53-43-96-96-96zM160 256a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zm352 0a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM112 336c-44.2 0-80 35.8-80 80l0 33.1c0 17 13.8 30.9 30.9 30.9l87.8 0c-4.3-9.8-6.7-20.6-6.7-32l0-48c0-18.4 3.5-36 9.8-52.2-12.2-7.5-26.5-11.8-41.8-11.8zM425.4 480l87.8 0c17 0 30.9-13.8 30.9-30.9l0-33.1c0-44.2-35.8-80-80-80-15.3 0-29.6 4.3-41.8 11.8 6.3 16.2 9.8 33.8 9.8 52.2l0 48c0 11.4-2.4 22.2-6.7 32z"
							/>
						</svg>
					</div>
					<p class="text-sm font-medium text-gray-700">5</p>
				</div>
				<div class="flex flex-col items-center">
					<div
						class="mb-2 flex h-16 w-16 cursor-pointer items-center justify-center rounded-full bg-gray-100"
						title="Profit"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 640 640"
							class="h-10 w-10 text-gray-600"
						>
							<path
								d="M544 72C544 58.7 533.3 48 520 48L418.2 48C404.9 48 394.2 58.7 394.2 72C394.2 85.3 404.9 96 418.2 96L462.1 96L350.8 207.3L255.7 125.8C246.7 118.1 233.5 118.1 224.5 125.8L112.5 221.8C102.4 230.4 101.3 245.6 109.9 255.6C118.5 265.6 133.7 266.8 143.7 258.2L240.1 175.6L336.5 258.2C346 266.4 360.2 265.8 369.1 256.9L496.1 129.9L496.1 173.8C496.1 187.1 506.8 197.8 520.1 197.8C533.4 197.8 544.1 187.1 544.1 173.8L544 72zM112 320C85.5 320 64 341.5 64 368L64 528C64 554.5 85.5 576 112 576L528 576C554.5 576 576 554.5 576 528L576 368C576 341.5 554.5 320 528 320L112 320zM159.3 376C155.9 396.1 140.1 412 119.9 415.4C115.5 416.1 111.9 412.5 111.9 408.1L111.9 376.1C111.9 371.7 115.5 368.1 119.9 368.1L151.9 368.1C156.3 368.1 160 371.7 159.2 376.1zM159.3 520.1C160 524.5 156.4 528.1 152 528.1L120 528.1C115.6 528.1 112 524.5 112 520.1L112 488.1C112 483.7 115.6 480 120 480.8C140.1 484.2 156 500 159.4 520.2zM520 480.7C524.4 480 528 483.6 528 488L528 520C528 524.4 524.4 528 520 528L488 528C483.6 528 479.9 524.4 480.7 520C484.1 499.9 499.9 484 520.1 480.6zM480.7 376C480 371.6 483.6 368 488 368L520 368C524.4 368 528 371.6 528 376L528 408C528 412.4 524.4 416.1 520 415.3C499.9 411.9 484 396.1 480.6 375.9zM256 448C256 412.7 284.7 384 320 384C355.3 384 384 412.7 384 448C384 483.3 355.3 512 320 512C284.7 512 256 483.3 256 448z"
							/>
						</svg>
					</div>
					<p class="text-sm font-medium text-gray-700">351 $</p>
				</div>
			</div>
		</Card.Content>

		<Card.Footer class="px-4">
			<p class="text-sm text-muted-foreground">
				Created at: {new Date().toLocaleDateString()}
			</p>
		</Card.Footer>
	</Card.Root>
</a>

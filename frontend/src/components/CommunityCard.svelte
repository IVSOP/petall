<script lang="ts">
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	const { community } = $props();
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
							viewBox="0 0 640 640"
							class="h-10 w-10 text-gray-600"
						>
							<path
								d="M320 80C377.4 80 424 126.6 424 184C424 241.4 377.4 288 320 288C262.6 288 216 241.4 216 184C216 126.6 262.6 80 320 80zM96 152C135.8 152 168 184.2 168 224C168 263.8 135.8 296 96 296C56.2 296 24 263.8 24 224C24 184.2 56.2 152 96 152zM0 480C0 409.3 57.3 352 128 352C140.8 352 153.2 353.9 164.9 357.4C132 394.2 112 442.8 112 496L112 512C112 523.4 114.4 534.2 118.7 544L32 544C14.3 544 0 529.7 0 512L0 480zM521.3 544C525.6 534.2 528 523.4 528 512L528 496C528 442.8 508 394.2 475.1 357.4C486.8 353.9 499.2 352 512 352C582.7 352 640 409.3 640 480L640 512C640 529.7 625.7 544 608 544L521.3 544zM472 224C472 184.2 504.2 152 544 152C583.8 152 616 184.2 616 224C616 263.8 583.8 296 544 296C504.2 296 472 263.8 472 224zM160 496C160 407.6 231.6 336 320 336C408.4 336 480 407.6 480 496L480 512C480 529.7 465.7 544 448 544L192 544C174.3 544 160 529.7 160 512L160 496z"
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
							viewBox="0 0 640 640"
							class="h-11 w-11 text-gray-600"
						>
							<path
								d="M256 192C256 156.7 284.7 128 320 128C355.3 128 384 156.7 384 192C384 227.3 355.3 256 320 256C284.7 256 256 227.3 256 192zM208 400C208 338.1 258.1 288 320 288C381.9 288 432 338.1 432 400L432 408C432 421.3 421.3 432 408 432L232 432C218.7 432 208 421.3 208 408L208 400zM424 208C424 177.1 449.1 152 480 152C510.9 152 536 177.1 536 208C536 238.9 510.9 264 480 264C449.1 264 424 238.9 424 208zM451.2 308.4C460.3 305.5 470 304 480 304C533 304 576 347 576 400L576 410.7C576 422.5 566.4 432 554.7 432L475.9 432C478.6 424.5 480 416.4 480 408L480 400C480 365.9 469.4 334.3 451.2 308.4zM188.8 308.4C170.6 334.4 160 365.9 160 400L160 408C160 416.4 161.4 424.5 164.1 432L85.3 432C73.6 432 64 422.4 64 410.7L64 400C64 347 107 304 160 304C170 304 179.7 305.5 188.8 308.4zM104 208C104 177.1 129.1 152 160 152C190.9 152 216 177.1 216 208C216 238.9 190.9 264 160 264C129.1 264 104 238.9 104 208zM32 504C32 490.7 42.7 480 56 480L584 480C597.3 480 608 490.7 608 504C608 517.3 597.3 528 584 528L56 528C42.7 528 32 517.3 32 504z"
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
				Created: {new Date().toLocaleDateString()}
			</p>
		</Card.Footer>
	</Card.Root>
</a>

<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import Users from '@lucide/svelte/icons/users';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import Trash_2 from '@lucide/svelte/icons/trash-2';
	import UserPlus from '@tabler/icons-svelte/icons/user-plus';

	let { open = $bindable(), members, title, type } = $props();

    function removeMember(member_id: String) {
        console.log(member_id);
    };
</script>

<Card.Root class="w-full gap-0 pb-0 lg:w-1/2">
    <Card.Header class="flex items-center justify-between border-b px-3">
        <div class="flex items-center gap-2">
            <Users />
            <h2 class="text-lg font-semibold">
                {title}
            </h2>
            <span class="text-sm text-muted-foreground">
                {members.length} members
            </span>
        </div>
        <Button class="cursor-pointer" onclick={() => (open = true)}>
            <UserPlus />
            Add {type}
        </Button>
    </Card.Header>

    <Card.Content class="max-h-[400px] overflow-hidden p-0">
        <ScrollArea class="h-[400px] w-full rounded-lg">
            {#each members as member}
                <div
                    class="flex items-center justify-between gap-3 border-b p-3 transition-colors hover:bg-accent/50"
                >
                    <div class="flex items-center gap-3">
                        <Avatar.Root class="h-10 w-10">
                            <Avatar.Fallback class="bg-primary/10 text-sm font-medium">
                                {member.name.slice(0, 2).toUpperCase()}
                            </Avatar.Fallback>
                        </Avatar.Root>
                        <span class="font-medium">{member.name}</span>
                    </div>

                    <Button
                        variant="destructive"
                        class="cursor-pointer text-sm font-medium"
                        onclick={() => removeMember(member.id)}
                    >
                        <Trash_2 />
                        Remove
                    </Button>
                </div>
            {/each}
        </ScrollArea>
    </Card.Content>
</Card.Root>
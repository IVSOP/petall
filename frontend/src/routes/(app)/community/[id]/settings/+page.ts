import type { Community, Participant } from '$lib';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const community: Community = await fetch(`/api/community/${params.id}`).then((res) => res.json());

	const participants: Participant[] = [
		{
			name: 'Digo',
			id: 'https://github.com/DigoqueDigo.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{ name: 'IVSOP', id: 'https://github.com/ivsop.png', role: 'user', email: 'random@mail.com' },
		{
			name: 'Chico',
			id: 'https://github.com/chicoferreira.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{
			name: 'Pedro',
			id: 'https://github.com/pedroocarvalhoo.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{
			name: 'Digo1',
			id: 'https://github.com/DigoqueDigo.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{ name: 'IVSOP1', id: 'https://github.com/ivsop.png', role: 'user', email: 'random@mail.com' },
		{
			name: 'Chico1',
			id: 'https://github.com/chicoferreira.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{
			name: 'Pedro1',
			id: 'https://github.com/pedroocarvalhoo.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{
			name: 'Digo2',
			id: 'https://github.com/DigoqueDigo.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{ name: 'IVSOP2', id: 'https://github.com/ivsop.png', role: 'user', email: 'random@mail.com' },
		{
			name: 'Chico2',
			id: 'https://github.com/chicoferreira.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{
			name: 'Pedro2',
			id: 'https://github.com/pedroocarvalhoo.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{
			name: 'Digo3',
			id: 'https://github.com/DigoqueDigo.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{ name: 'IVSOP3', id: 'https://github.com/ivsop.png', role: 'user', email: 'random@mail.com' },
		{
			name: 'Chico3',
			id: 'https://github.com/chicoferreira.png',
			role: 'user',
			email: 'random@mail.com'
		},
		{
			name: 'Pedro3',
			id: 'https://github.com/pedroocarvalhoo.png',
			role: 'user',
			email: 'random@mail.com'
		}
	];

	return {
		community,
		participants
	};
};

import { getContext, setContext } from 'svelte';

import type { Container } from '$lib/container';

const CONTAINER_CONTEXT_KEY = Symbol('container');

export function setContainer(container: Container): void {
	setContext(CONTAINER_CONTEXT_KEY, container);
}

export function getContainer(): Container {
	const container = getContext<Container | undefined>(CONTAINER_CONTEXT_KEY);
	if (!container) {
		throw new Error('Container context is not available');
	}

	return container;
}

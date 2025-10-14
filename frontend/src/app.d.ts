// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}

		interface Locals {
			container: import('$lib/container').Container;
		}

		// interface PageData {}
		// interface PageState {}
		// interface Platform {}

		declare const __VERSION__: string;
	}

	interface Window {
		dataLayer: unknown[];
	}
}

export {};

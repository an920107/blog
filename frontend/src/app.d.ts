// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}

		interface Locals {
			postListBloc: import('$lib/post/adapter/presenter/postListBloc').PostListBloc;
			postBloc: import('$lib/post/adapter/presenter/postBloc').PostBloc;
		}

		// interface PageData {}
		// interface PageState {}
		// interface Platform {}

		declare const __VERSION__: string;
	}
}

export {};

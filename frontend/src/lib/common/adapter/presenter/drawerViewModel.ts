import type { Snippet } from 'svelte';

export class DrawerViewModel {
	readonly isOpen: boolean;
	readonly actions: Snippet | null;
	readonly content: Snippet | null;

	private constructor(props: {
		isOpen: boolean;
		actions: Snippet | null;
		content: Snippet | null;
	}) {
		this.isOpen = props.isOpen;
		this.actions = props.actions;
		this.content = props.content;
	}

	static empty(): DrawerViewModel {
		return new DrawerViewModel({ isOpen: false, actions: null, content: null });
	}

	copyWith(props: {
		isOpen?: boolean;
		actions?: Snippet | null;
		content?: Snippet | null;
	}): DrawerViewModel {
		return new DrawerViewModel({
			isOpen: props.isOpen !== undefined ? props.isOpen : this.isOpen,
			actions: props.actions !== undefined ? props.actions : this.actions,
			content: props.content !== undefined ? props.content : this.content,
		});
	}
}

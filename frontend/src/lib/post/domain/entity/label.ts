import type { Color } from '$lib/post/domain/entity/color';

export class Label {
	readonly id: number;
	readonly name: string;
	readonly color: Color;

	constructor(props: { id: number; name: string; color: Color }) {
		this.id = props.id;
		this.name = props.name;
		this.color = props.color;
	}
}

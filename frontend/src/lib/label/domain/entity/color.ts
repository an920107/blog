export class Color {
	readonly red: number;
	readonly green: number;
	readonly blue: number;
	readonly alpha: number;

	constructor(props: { red: number; green: number; blue: number; alpha: number }) {
		this.red = props.red;
		this.green = props.green;
		this.blue = props.blue;
		this.alpha = props.alpha;
	}
}

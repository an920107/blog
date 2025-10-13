export class User {
	readonly id: number;
	readonly name: string;
	readonly email: string;

	constructor(props: { id: number; name: string; email: string }) {
		this.id = props.id;
		this.name = props.name;
		this.email = props.email;
	}
}

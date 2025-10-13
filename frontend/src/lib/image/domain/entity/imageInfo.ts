export class ImageInfo {
	readonly id: number;
	readonly mimeType: string;
	readonly url: URL;

	constructor(props: { id: number; mimeType: string; url: URL }) {
		this.id = props.id;
		this.mimeType = props.mimeType;
		this.url = props.url;
	}
}

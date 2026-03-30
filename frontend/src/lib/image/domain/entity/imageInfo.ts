export class ImageInfo {
	readonly id: number;
	readonly mimeType: string;
	readonly url: URL;
	readonly isReferred: boolean;

	constructor(props: { id: number; mimeType: string; url: URL; isReferred: boolean }) {
		this.id = props.id;
		this.mimeType = props.mimeType;
		this.url = props.url;
		this.isReferred = props.isReferred;
	}
}

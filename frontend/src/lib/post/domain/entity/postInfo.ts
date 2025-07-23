import type { Label } from '$lib/post/domain/entity/label';

export class PostInfo {
	readonly id: number;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: URL;
	readonly labels: readonly Label[];
	readonly publishedTime: Date;

	constructor(props: {
		id: number;
		title: string;
		description: string;
		previewImageUrl: URL;
		labels: readonly Label[];
		publishedTime: Date;
	}) {
		this.id = props.id;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
	}
}

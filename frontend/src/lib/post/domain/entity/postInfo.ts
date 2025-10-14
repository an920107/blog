import type { Label } from '$lib/label/domain/entity/label';

export class PostInfo {
	readonly id: number;
	readonly semanticId: string;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: URL | null;
	readonly labels: readonly Label[];
	readonly publishedTime: Date | null;

	constructor(props: {
		id: number;
		semanticId: string;
		title: string;
		description: string;
		previewImageUrl: URL | null;
		labels: readonly Label[];
		publishedTime: Date | null;
	}) {
		this.id = props.id;
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
	}
}

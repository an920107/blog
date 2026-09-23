import type { Label } from '$lib/label/domain/entity/label';

export class PostInfo {
	readonly id: number;
	readonly semanticId: string;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: string | null;
	readonly previewImageMimeType: string | null;
	readonly previewImageSize: number | null;
	readonly labels: readonly Label[];
	readonly publishedTime: Date | null;
	readonly updatedTime: Date | null;

	constructor(props: {
		id: number;
		semanticId: string;
		title: string;
		description: string;
		previewImageUrl: string | null;
		previewImageMimeType: string | null;
		previewImageSize: number | null;
		labels: readonly Label[];
		publishedTime: Date | null;
		updatedTime: Date | null;
	}) {
		this.id = props.id;
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.previewImageMimeType = props.previewImageMimeType;
		this.previewImageSize = props.previewImageSize;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
		this.updatedTime = props.updatedTime;
	}
}

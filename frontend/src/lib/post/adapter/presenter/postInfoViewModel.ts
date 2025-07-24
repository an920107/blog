import {
	LabelViewModel,
	type DehydratedLabelProps
} from '$lib/post/adapter/presenter/labelViewModel';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class PostInfoViewModel {
	readonly id: number;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: URL;
	readonly labels: readonly LabelViewModel[];
	readonly publishedTime: Date;

	private constructor(props: {
		id: number;
		title: string;
		description: string;
		previewImageUrl: URL;
		labels: readonly LabelViewModel[];
		publishedTime: Date;
	}) {
		this.id = props.id;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
	}

	static fromEntity(postInfo: PostInfo): PostInfoViewModel {
		return new PostInfoViewModel({
			id: postInfo.id,
			title: postInfo.title,
			description: postInfo.description,
			previewImageUrl: postInfo.previewImageUrl,
			labels: postInfo.labels.map((label) => LabelViewModel.fromEntity(label)),
			publishedTime: postInfo.publishedTime
		});
	}

	static rehydrate(props: DehydratedPostInfoProps): PostInfoViewModel {
		return new PostInfoViewModel({
			id: props.id,
			title: props.title,
			description: props.description,
			previewImageUrl: new URL(props.previewImageUrl),
			labels: props.labels.map((label) => LabelViewModel.rehydrate(label)),
			publishedTime: new Date(props.publishedTime)
		});
	}

	get formattedPublishedTime(): string {
		return this.publishedTime.toISOString().slice(0, 10);
	}

	dehydrate(): DehydratedPostInfoProps {
		return {
			id: this.id,
			title: this.title,
			description: this.description,
			previewImageUrl: this.previewImageUrl.href,
			labels: this.labels.map((label) => label.dehydrate()),
			publishedTime: this.publishedTime.getTime()
		};
	}
}

export interface DehydratedPostInfoProps {
	id: number;
	title: string;
	description: string;
	previewImageUrl: string;
	labels: DehydratedLabelProps[];
	publishedTime: number;
}

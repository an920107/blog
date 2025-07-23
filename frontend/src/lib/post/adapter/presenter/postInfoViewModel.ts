import { LabelViewModel } from '$lib/post/adapter/presenter/labelViewModel';
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
}

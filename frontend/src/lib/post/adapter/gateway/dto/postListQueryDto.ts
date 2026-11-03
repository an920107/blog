export class PostListQueryDto {
	readonly showUnpublished: boolean;
	readonly labelId?: number;
	readonly keyword?: string;

	constructor(props: { showUnpublished: boolean; labelId?: number; keyword?: string }) {
		this.showUnpublished = props.showUnpublished;
		this.labelId = props.labelId;
		this.keyword = props.keyword;
	}

	toSearchParams(): URLSearchParams {
		const params = new URLSearchParams();
		params.append('is_published_only', (!this.showUnpublished).toString());
		if (this.labelId !== undefined) {
			params.append('label_id', this.labelId.toString());
		}
		if (this.keyword !== undefined) {
			params.append('keyword', this.keyword);
		}
		return params;
	}
}

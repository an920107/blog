export class PostListQueryDto {
	readonly showUnpublished: boolean;
	readonly labelId?: number;

	constructor(props: { showUnpublished: boolean; labelId?: number }) {
		this.showUnpublished = props.showUnpublished;
		this.labelId = props.labelId;
	}

	toSearchParams(): URLSearchParams {
		const params = new URLSearchParams();
		params.append('is_published_only', (!this.showUnpublished).toString());
		if (this.labelId !== undefined) {
			params.append('label_id', this.labelId.toString());
		}
		return params;
	}
}

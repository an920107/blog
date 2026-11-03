import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export interface PostRepository {
	getAllPosts(showUnpublished: boolean, labelId?: number, keyword?: string): Promise<PostInfo[]>;
	getPost(id: string | number): Promise<Post | null>;
	createPostDraft(params: CreatePostDraftParams): Promise<Post>;
	updatePost(id: number, params: UpdatePostParams): Promise<Post>;
}

export interface CreatePostDraftParams {
	/** Identity fields for an empty draft; content is filled in when the post is edited. */
	semanticId: string;
	title: string;
}

export interface UpdatePostParams {
	title: string;
	description: string;
	content: string;
	labelIds: number[];
	previewImageUrl: string | null;
	publishedTime: Date | null;
}

import { resolve } from '$app/paths';
import { Environment } from '$lib/environment';
import type { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';
import { PostPage } from '$lib/seo/domain/entity/postPage';

/**
 * Turns published posts into their public pages. The public URL of a post is a frontend
 * routing concern, so this is the single place that knows how to build it. Both sitemap.xml
 * and feed.xml consume the result, so the link is never duplicated in the backend or docs.
 */
export class ListPostPagesUseCase {
	constructor(private readonly getAllPostsUseCase: GetAllPostsUseCase) {}

	async execute(): Promise<readonly PostPage[]> {
		const posts = await this.getAllPostsUseCase.execute(false);

		return posts.flatMap((post) => {
			const { publishedTime } = post;
			if (publishedTime === null) {
				return [];
			}
			return [ListPostPagesUseCase.toPostPage(post, publishedTime)];
		});
	}

	private static toPostPage(post: PostInfo, publishedTime: Date): PostPage {
		return new PostPage({
			url: new URL(resolve('/post/[id]', { id: post.semanticId }), Environment.APP_BASE_URL),
			semanticId: post.semanticId,
			title: post.title,
			description: post.description,
			publishedTime,
			lastModified: post.updatedTime ?? publishedTime,
			previewImageUrl: post.previewImageUrl
				? new URL(post.previewImageUrl, Environment.APP_BASE_URL)
				: null,
			previewImageMimeType: post.previewImageMimeType,
			previewImageSize: post.previewImageSize,
			categories: post.labels.map((label) => label.name),
		});
	}
}

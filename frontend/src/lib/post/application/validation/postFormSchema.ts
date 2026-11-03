import z from 'zod';

export const postFormSchema = z.object({
	semanticId: z
		.string()
		.max(100)
		.regex(/\D/)
		.regex(/^[\da-z_-]+$/i),
	title: z.string().trim().nonempty(),
	description: z
		.string()
		.trim()
		.transform((value) => value.replaceAll('\n', '')),
	content: z.string(),
	labelIds: z.array(z.number()),
	previewImageUrl: z.string().nullable(),
	publishedTime: z.date().nullable(),
});

export type PostFormValues = z.infer<typeof postFormSchema>;

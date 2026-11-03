import z from 'zod';

export const labelFormSchema = z.object({
	name: z.string().trim().nonempty(),
	color: z.string().regex(/^#?[a-f\d]{6}$/i),
});

export type LabelFormValues = z.infer<typeof labelFormSchema>;

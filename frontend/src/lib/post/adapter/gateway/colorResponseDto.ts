import { Color } from '$lib/post/domain/entity/color';
import z from 'zod';

export const ColorResponseSchema = z.object({
	red: z.number().int().min(0).max(255),
	green: z.number().int().min(0).max(255),
	blue: z.number().int().min(0).max(255),
	alpha: z.number().int().min(0).max(255),
});

export class ColorResponseDto {
	readonly red: number;
	readonly green: number;
	readonly blue: number;
	readonly alpha: number;

	private constructor(props: { red: number; green: number; blue: number; alpha: number }) {
		this.red = props.red;
		this.green = props.green;
		this.blue = props.blue;
		this.alpha = props.alpha;
	}

	static fromJson(json: unknown): ColorResponseDto {
		const parsedJson = ColorResponseSchema.parse(json);
		return new ColorResponseDto({
			red: parsedJson.red,
			green: parsedJson.green,
			blue: parsedJson.blue,
			alpha: parsedJson.alpha,
		});
	}

	toEntity(): Color {
		return new Color({
			red: this.red,
			green: this.green,
			blue: this.blue,
			alpha: this.alpha,
		});
	}
}

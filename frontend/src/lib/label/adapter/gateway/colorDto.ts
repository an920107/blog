import { Color } from '$lib/label/domain/entity/color';
import z from 'zod';

export const colorResponseSchema = z.object({
	red: z.number().int().min(0).max(255),
	green: z.number().int().min(0).max(255),
	blue: z.number().int().min(0).max(255),
	alpha: z.number().int().min(0).max(255),
});

export class ColorDto {
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

	static fromEntity(color: Color): ColorDto {
		return new ColorDto(color);
	}

	static fromJson(json: unknown): ColorDto {
		const parsedJson = colorResponseSchema.parse(json);
		return new ColorDto({
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

	toJson() {
		return {
			red: this.red,
			green: this.green,
			blue: this.blue,
			alpha: this.alpha,
		};
	}
}

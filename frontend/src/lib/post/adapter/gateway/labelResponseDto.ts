import { ColorResponseDto, ColorResponseSchema } from '$lib/post/adapter/gateway/colorResponseDto';
import { Label } from '$lib/post/domain/entity/label';
import { z } from 'zod';

export const LabelResponseSchema = z.object({
	id: z.int32(),
	name: z.string(),
	color: ColorResponseSchema
});

export class LabelResponseDto {
	readonly id: number;
	readonly name: string;
	readonly color: ColorResponseDto;

	private constructor(props: { id: number; name: string; color: ColorResponseDto }) {
		this.id = props.id;
		this.name = props.name;
		this.color = props.color;
	}

	static fromJson(json: unknown): LabelResponseDto {
		const parsedJson = LabelResponseSchema.parse(json);
		return new LabelResponseDto({
			id: parsedJson.id,
			name: parsedJson.name,
			color: ColorResponseDto.fromJson(parsedJson.color)
		});
	}

	toEntity(): Label {
		return new Label({
			id: this.id,
			name: this.name,
			color: this.color
		});
	}
}

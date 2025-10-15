import { ColorDto, colorResponseSchema } from '$lib/label/adapter/gateway/colorDto';
import { Label } from '$lib/label/domain/entity/label';
import { z } from 'zod';

export const labelResponseSchema = z.object({
	id: z.int32(),
	name: z.string(),
	color: colorResponseSchema,
});

export class LabelResponseDto {
	readonly id: number;
	readonly name: string;
	readonly color: ColorDto;

	private constructor(props: { id: number; name: string; color: ColorDto }) {
		this.id = props.id;
		this.name = props.name;
		this.color = props.color;
	}

	static fromJson(json: unknown): LabelResponseDto {
		const parsedJson = labelResponseSchema.parse(json);
		return new LabelResponseDto({
			id: parsedJson.id,
			name: parsedJson.name,
			color: ColorDto.fromJson(parsedJson.color),
		});
	}

	toEntity(): Label {
		return new Label({
			id: this.id,
			name: this.name,
			color: this.color,
		});
	}
}

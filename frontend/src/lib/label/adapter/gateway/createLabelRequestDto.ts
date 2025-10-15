import { ColorDto } from '$lib/label/adapter/gateway/colorDto';
import type { CreateLabelParams } from '$lib/label/application/gateway/labelRepository';

export class CreateLabelRequestDto {
	readonly name: string;
	readonly color: ColorDto;

	private constructor(props: { name: string; color: ColorDto }) {
		this.name = props.name;
		this.color = props.color;
	}

	static fromParams(params: CreateLabelParams): CreateLabelRequestDto {
		return new CreateLabelRequestDto({
			name: params.name,
			color: ColorDto.fromEntity(params.color),
		});
	}

	toJson() {
		return {
			name: this.name,
			color: this.color.toJson(),
		};
	}
}

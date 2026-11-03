import type { Color } from '$lib/label/domain/entity/color';
import type { Label } from '$lib/label/domain/entity/label';

export interface LabelRepository {
	getAllLabels(): Promise<Label[]>;
	getLabelById(id: number): Promise<Label | null>;
	createLabel(params: CreateLabelParams): Promise<Label>;
	updateLabel(id: number, params: UpdateLabelParams): Promise<Label>;
}

export interface CreateLabelParams {
	name: string;
	color: Color;
}

export interface UpdateLabelParams {
	name: string;
	color: Color;
}

export enum ElementHeaderTypeLevels {
  Level1 = 1,
  Level2 = 2,
  Level3 = 3,
}

export enum ElementTypes {
  Heading1 = "heading1",
  Heading2 = "heading2",
  Heading3 = "heading3",
}

export const ElementTypesMap = new Map<number, ElementTypes>([
  [ElementHeaderTypeLevels.Level1, ElementTypes.Heading1],
  [ElementHeaderTypeLevels.Level2, ElementTypes.Heading2],
  [ElementHeaderTypeLevels.Level3, ElementTypes.Heading3],
]);

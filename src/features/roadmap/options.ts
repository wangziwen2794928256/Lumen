export const options = (values: Record<string, string>) =>
  Object.entries(values).map(([value, label]) => ({ value, label }));

interface EventChipProps {
  title: string;
}

export function EventChip({ title }: EventChipProps) {
  return <span>{title}</span>;
}

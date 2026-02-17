import { ReactNode } from 'react';

interface MainContentProps {
  children: ReactNode;
}

export function MainContent({ children }: MainContentProps) {
  return <main>{children}</main>;
}

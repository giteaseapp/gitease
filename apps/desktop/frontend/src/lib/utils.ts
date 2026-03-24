import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export type WithElementRef<T, R extends HTMLElement = HTMLElement> = T & { ref?: R | null };
export type WithoutChildren<T extends { children?: unknown }> = Omit<T, 'children'>;
export type WithoutChildrenOrChild<T extends { children?: unknown; child?: unknown }> = Omit<
	T,
	'children' | 'child'
>;

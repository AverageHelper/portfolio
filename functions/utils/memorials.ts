const NAMES = [
	"Terry Pratchett", // 28 April 1948 - 12 March 2015
	"MelodicPony", // 1987 - 12 December 2014
	"Jerry Guy Apalategui, Jr.", // 2 May 1999 - 28 August 2017
	"Nex Benedict", // 11 January 2008 - February 8, 2024
	"Kitty Monroe",
	"Righteous Torrence 'T K' Chevy Hill",
	"Reyna Hernandez",
	"Diamond Cherish Brigman",
	"Alex Franco",
	"Meraxes Medina",
	"Cecilia Gentili",
	"Ryan Zimmerman",
	"Tee Arnold",
	"África Parrilla García",
	"Nevaeh 'River' Goddard",
	"Andrea Doria Dos Passos",
	"Sasha Williams",
	"Kita Bee",
	"Starr Brown",
	"Jazlynn Johnson",
	"Paris Rich Jessup",
	"Kit Nelson-Mora",
	"Niomi Jenkins",
	"Tayy Dior Thomas",
	"Pauly Likens",
] as const;

/**
 * @returns a pseudo-random element of the given array.
 */
function randomElementOfArray<T>(array: readonly [T, ...ReadonlyArray<T>]): T {
	const index = Math.floor(Math.random() * array.length);
	return array[index] ?? array[0];
}

/**
 * @returns a random name to use in a memorial, such as an `X-Clacks-Overhead` header in HTTP messages.
 */
export function randomName(): string {
	return randomElementOfArray(NAMES);
}

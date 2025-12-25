export interface Item {
  id: number,
  name: string,
  date: number,
  tags: string[],
  body: string,
  link: string,
  starred: boolean,
}
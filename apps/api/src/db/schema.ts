import { int, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const EventType = {
  BROWSER_TAB: "browser_tab",
  APP: "app",
  PHONE: "phone",
} as const;

export type EventType = (typeof EventType)[keyof typeof EventType];

export const eventTable = sqliteTable("event_table", {
  id: int().primaryKey({ autoIncrement: true }),
  date: text().notNull(),
  title: text().notNull(),
  type: text().notNull().$type<EventType>(),
  url: text(),
  text: text().notNull(),
});

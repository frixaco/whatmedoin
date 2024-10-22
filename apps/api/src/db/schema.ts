import { int, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const EventType = {
  browser_tab: "browser_tab",
  app: "app",
  phone: "phone",
} as const;

export type EventType = (typeof EventType)[keyof typeof EventType];

export const eventTable = sqliteTable("event_table", {
  id: int().primaryKey({ autoIncrement: true }),
  date: text().notNull(),
  app_name: text().notNull(), // for type: "browser_tab" => browser name
  title: text().notNull(),
  type: text().notNull().$type<EventType>(),
  url: text(),
});

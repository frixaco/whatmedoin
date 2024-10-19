import { Hono } from "hono";
import { drizzle } from "drizzle-orm/libsql";
import { createClient } from "@libsql/client";
import { eventTable } from "./db/schema";
import { desc } from "drizzle-orm";

const app = new Hono();

const client = createClient({
  url: process.env.DATABASE_URL as string,
  authToken: process.env.DATABASE_AUTH_TOKEN as string,
});
const db = drizzle({ client });

app.post("/new-event", async (c) => {
  const { date, title, type, url, text } = await c.req.json();
  const result = await db
    .insert(eventTable)
    .values({ date, title, type, url, text });
  return c.json(result);
});

app.get("/latest-event", async (c) => {
  const result = await db
    .select()
    .from(eventTable)
    .orderBy(desc(eventTable.id))
    .limit(1);
  return c.json(result);
});

export default app;

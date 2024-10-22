import { Hono } from "hono";
import { drizzle } from "drizzle-orm/libsql";
import { createClient } from "@libsql/client";
import { eventTable, EventType } from "./db/schema";
import { desc } from "drizzle-orm";

const app = new Hono();

const client = createClient({
  url: process.env.DATABASE_URL as string,
  authToken: process.env.DATABASE_AUTH_TOKEN as string,
});
const db = drizzle({ client });

type NewEvent = {
  app_name: string;
  title: string;
  type: EventType;
  url: string;
};

app.post("/new-event", async (c) => {
  const { app_name, title, type, url } = await c.req.json<NewEvent>();
  console.log("new api event: ", { app_name, title, type, url });
  const result = await db.insert(eventTable).values({
    app_name,
    title,
    type,
    url,
    date: new Date().toLocaleString("en-US", { timeZone: "UTC" }),
  });

  console.log("db result: ", result);
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

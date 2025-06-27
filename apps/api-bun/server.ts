import { Hono } from "hono";
import { logger } from "hono/logger";
import { cors } from "hono/cors";
import { Schema, model, connect } from "mongoose";

interface Activity {
  platform: "browser" | "mobile" | "windows" | "macos";
  title: string; // tab title, app title
  url?: string; // tab url or empty
  date: string;
}

const activitySchema = new Schema<Activity>({
  platform: { type: String, required: true },
  title: { type: String, required: true },
  url: String,
  date: { type: String, required: true },
});

const Activity = model<Activity>("Activity", activitySchema);

const app = new Hono();

connect(process.env.DATABASE_URL!);

app.use("/*", cors());

app.use(logger());

app.post("/activity", async (c) => {
  const { platform, title, url } = await c.req.json<Activity>();
  console.log("new api event: ", { platform, title, url });
  const result = await Activity.create({
    platform,
    title,
    url,
    date: new Date().toLocaleString("en-US", { timeZone: "UTC" }),
  });
  const savedResult = result.toObject();

  console.log("saved result: ", savedResult);
  return c.json(savedResult, 201);
});

app.get("/activity", async (c) => {
  const result = await Activity.findOne().sort({ _id: -1 });

  console.log("activity result: ", result);
  return c.json(result);
});

export default app;

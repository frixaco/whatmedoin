import { logger, schedules, wait } from "@trigger.dev/sdk/v3";
import fetch from "node-fetch";

export const cleanupTask = schedules.task({
  id: "cleanup-whatmedoin-db",
  // Runs at midnight (00:00) every Sunday
  cron: "0 0 * * 0",
  // cron: "* * * * *",
  maxDuration: 300, // 5 minutes
  run: async (payload, { ctx }) => {
    console.log(
      "Starting deleting all records except last one from whatmedoin database",
    );

    const response = await fetch("https://api.frixaco.com/activity", {
      method: "DELETE",
    });
    const data = await response.json();

    console.log("Finished database cleanup. Last activity:", data);
  },
});

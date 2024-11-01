// firefox/background.ts
browser.tabs.onUpdated.addListener(async (tabId, changeInfo, tab) => {
  if (changeInfo.status == "complete" && tab.active) {
    console.log("\uD83D\uDD35 whatmedoin processing active tab");
    try {
      const sessionValue = await browser.storage.session.get("apiUrl");
      const apiUrl = sessionValue.key;
      if (!apiUrl || apiUrl === "") {
        console.warn("\uD83D\uDFE1 whatmedoin: No API URL provided");
        return;
      }
      const response = await fetch(apiUrl + "/activity", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          platform: "browser",
          title: tab.title,
          url: tab.url
        })
      });
      if (response.ok) {
        console.log("\uD83D\uDFE2 whatmedoin");
      } else {
        console.error("\uD83D\uDD34 whatmedoin API error:", response.statusText);
      }
    } catch (error) {
      console.error("\uD83D\uDD34 whatmedoin extension error:", error);
    }
  }
});

// let API_URL = "http://localhost:3000";

browser.tabs.onUpdated.addListener(async (tabId, changeInfo, tab) => {
  if (changeInfo.status == "complete" && tab.active) {
    console.log("🔵 whatmedoin processing active tab");

    const allowedDomains = [
      "guidetojapanese.org",
      "animelon.com",
      "youtube.com",
    ];

    const url = new URL(tab.url || "");
    const isAllowedDomain = allowedDomains.some(
      (domain) => url.hostname === domain || url.hostname.endsWith("." + domain)
    );

    if (!isAllowedDomain) {
      console.log("🟡 whatmedoin: URL not in allowed domains");
      return;
    }

    try {
      const storageValue = await browser.storage.local.get("apiUrl");
      const apiUrl = storageValue.apiUrl;
      if (!apiUrl || apiUrl === "") {
        console.warn("🟡 whatmedoin: No API URL provided");
        return;
      }
      const response = await fetch(apiUrl + "/activity", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          platform: "browser",
          title: tab.title,
          url: tab.url,
        }),
      });
      if (response.ok) {
        console.log("🟢 whatmedoin");
      } else {
        console.error("🔴 whatmedoin API error:", response.statusText);
      }
    } catch (error) {
      console.error("🔴 whatmedoin extension error:", error);
    }
  }
});

// browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
//   if (message.type === "api-url") {
//     API_URL = message.url;
//   }
//   return true;
// });

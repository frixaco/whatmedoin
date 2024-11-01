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
      const storageValue = await chrome.storage.local.get("apiUrl");
      const apiUrl = storageValue.apiUrl;
      if (!apiUrl || apiUrl === "") {
        console.warn("🟡 whatmedoin: No API URL provided");
        return;
      }

      const payload = {
        platform: "browser",
        title: tab.title,
        url: tab.url,
      };

      console.log("Sending to", apiUrl, payload);

      const response = await fetch(apiUrl, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
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

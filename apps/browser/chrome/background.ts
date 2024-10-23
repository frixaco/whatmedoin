// let API_URL = "http://localhost:3000";

chrome.tabs.onUpdated.addListener(async (tabId, changeInfo, tab) => {
  if (changeInfo.status == "complete" && tab.active) {
    console.log("🔵 whatmedoin processing active tab");
    try {
      const sessionValue = await chrome.storage.session.get("apiUrl");
      const apiUrl = sessionValue.key;
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

// chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
//   if (message.type === "api-url") {
//     API_URL = message.url;
//   }
//   return true;
// });

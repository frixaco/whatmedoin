// chrome/background.ts
chrome.tabs.onActivated.addListener(async (activeInfo) => {
  console.log("\uD83D\uDD35 whatmedoin onActivated", activeInfo);
  handleTabChange(activeInfo.tabId);
});
chrome.tabs.onUpdated.addListener(async (tabId, changeInfo, tab) => {
  console.log("\uD83D\uDD35 whatmedoin onUpdated", { tabId, changeInfo, tab });
  if (changeInfo.status === "complete") {
    handleTabChange(tabId);
  }
});
async function handleTabChange(tabId) {
  const tab = await chrome.tabs.get(tabId);
  if (!tab.url)
    return;
  const allowedDomains = ["guidetojapanese.org", "animelon.com", "youtube.com"];
  const url = new URL(tab.url);
  const isAllowedDomain = allowedDomains.some((domain) => url.hostname === domain || url.hostname.endsWith("." + domain));
  if (!isAllowedDomain) {
    console.log("\uD83D\uDFE1 whatmedoin: URL not in allowed domains", tab);
    return;
  }
  try {
    const storageValue = await chrome.storage.local.get("apiUrl");
    const apiUrl = storageValue.apiUrl;
    if (!apiUrl || apiUrl === "") {
      console.warn("\uD83D\uDFE1 whatmedoin: No API URL provided");
      return;
    }
    const payload = {
      platform: "browser",
      title: tab.title,
      url: tab.url
    };
    console.log("Sending to", apiUrl, payload);
    const response = await fetch(apiUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(payload)
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

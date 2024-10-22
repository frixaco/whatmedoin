const API_URL = "http://localhost:3000";

browser.runtime.onInstalled.addListener(() => {
  console.log("whatmedoin extension installed");
});

browser.tabs.onUpdated.addListener(async (tabId, changeInfo, tab) => {
  if (changeInfo.status == "complete" && tab.active) {
    const response = await fetch(API_URL + "/new-event", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        type: "browser_tab",
        title: tab.title,
        url: tab.url,
        app_name: navigator.userAgent,
      }),
    });
    const data = await response.json();
    console.log(data);
  }
});

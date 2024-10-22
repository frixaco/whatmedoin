browser.runtime.onInstalled.addListener(() => {
  console.log("Extension installed");
});

browser.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  console.log(tab.url, tab.title, tab.id, tabId);
});

// chrome/background.ts
chrome.runtime.onInstalled.addListener(() => {
  console.log("Extension installed");
});
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  console.log(tab.url, tab.title, tab.id, tabId);
});

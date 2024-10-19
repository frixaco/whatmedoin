// chrome/popup.ts
function getTitle() {
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    const activeTab = tabs[0];
    console.log(activeTab.title, activeTab.url);
    const tabTitle = document.getElementById("tab-title");
    if (tabTitle) {
      tabTitle.innerText = activeTab.title || "No title found";
    } else {
      console.error("No tab title element found");
    }
  });
}
document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("get-title")?.addEventListener("click", getTitle);
});

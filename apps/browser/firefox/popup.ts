function getTitle() {
  browser.tabs
    .query({ active: true, currentWindow: true })
    .then((tabs) => {
      const activeTab = tabs[0];
      console.log(activeTab.title, activeTab.url);

      const tabTitle = document.getElementById("tab-title");
      if (tabTitle) {
        tabTitle.innerText = activeTab.title || "No title found";
      } else {
        console.error("No tab title element found");
      }
    })
    .catch((error) => {
      console.error("Error getting tab:", error);
    });
}

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("get-title")?.addEventListener("click", getTitle);
});

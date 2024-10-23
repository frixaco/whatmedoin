function getActiveTab() {
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    const activeTab = tabs[0];

    let tabTitle = "...";
    let tabUrl = "...";

    const tabTitleEl = document.getElementById("active-tab-title");
    if (tabTitleEl) {
      tabTitleEl.innerText = activeTab.title || "Could not get title";
      tabTitle = activeTab.title || "Could not get title";
    } else {
      console.error("No tab title element found");
    }

    const tabUrlEl = document.getElementById("active-tab-url");
    if (tabUrlEl) {
      tabUrlEl.innerText = activeTab.url || "Could not get url";
      tabUrl = activeTab.url || "Could not get url";
    } else {
      console.error("No tab url element found");
    }

    chrome.runtime.sendMessage({
      type: "active-tab-info",
      title: tabTitle,
      url: tabUrl,
    });
  });
}

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("refresh")?.addEventListener("click", getActiveTab);

  document
    .getElementById("set-api-url")
    ?.addEventListener("click", async () => {
      if (
        (document.getElementById("api-url") as HTMLInputElement).value === ""
      ) {
        document.getElementById("set-api-url-status")!.innerText =
          "Provide a URL";
        document.getElementById("set-api-url-status")!.style.color = "white";
        return;
      }

      document.getElementById("set-api-url-status")!.innerText = "Setting...";

      const apiUrl = (document.getElementById("api-url") as HTMLInputElement)
        .value;

      chrome.storage.session.set({ apiUrl });
      // chrome.runtime.sendMessage({
      //   type: "api-url",
      //   url: apiUrl,
      // });

      const storageValueCheck = await chrome.storage.session.get("apiUrl");
      if (storageValueCheck) {
        document.getElementById("set-api-url-status")!.innerText = "Success";
        // make text green and remove text after 2 seconds
        document.getElementById("set-api-url-status")!.style.color = "green";
      } else {
        document.getElementById("set-api-url-status")!.innerText = "Failed";
        document.getElementById("set-api-url-status")!.style.color = "red";
      }
      setTimeout(() => {
        document.getElementById("set-api-url-status")!.innerText = "";
      }, 2000);
    });
});

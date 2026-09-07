"use strict";

(() => {
  const aboutTabs = [...document.querySelectorAll("[data-about-tab]")];
  const aboutPanels = [...document.querySelectorAll("[data-about-panel]")];
  const recommendationButtons = [...document.querySelectorAll("[data-recommend-work]")];
  const recommendationPanels = [...document.querySelectorAll("[data-recommendation-panel]")];
  const dialog = document.getElementById("recent-work-focused");
  let opener = null;

  function selectAbout(tab) {
    for (const button of aboutTabs) {
      const selected = button === tab;
      button.classList.toggle("selected", selected);
      button.setAttribute("aria-selected", String(selected));
      button.tabIndex = selected ? 0 : -1;
    }
    for (const panel of aboutPanels) {
      const selected = panel.dataset.aboutPanel === tab.dataset.aboutTab;
      panel.hidden = !selected;
      panel.classList.toggle("active", selected);
    }
  }

  for (const tab of aboutTabs) {
    tab.addEventListener("click", () => selectAbout(tab));
    tab.addEventListener("keydown", (event) => {
      let index = aboutTabs.indexOf(tab);
      if (event.key === "ArrowRight") index = (index + 1) % aboutTabs.length;
      else if (event.key === "ArrowLeft") index = (index + aboutTabs.length - 1) % aboutTabs.length;
      else if (event.key === "Home") index = 0;
      else if (event.key === "End") index = aboutTabs.length - 1;
      else return;
      event.preventDefault();
      selectAbout(aboutTabs[index]);
      aboutTabs[index].focus();
    });
  }

  for (const button of recommendationButtons) {
    button.addEventListener("click", () => {
      for (const panel of recommendationPanels) {
        panel.hidden = panel.dataset.recommendationPanel !== button.dataset.recommendWork;
      }
      for (const selection of recommendationButtons) {
        selection.setAttribute("aria-pressed", String(selection === button));
      }
    });
  }

  // Delegation also handles galleries cloned from the server-rendered templates.
  document.addEventListener("click", (event) => {
    if (!(event.target instanceof Element)) return;
    const thumbnail = event.target.closest("[data-image-src]");
    if (thumbnail) {
      const panel = thumbnail.closest(".work-spotlight");
      const focused = panel?.querySelector("[data-focused-image]");
      if (focused) {
        focused.src = thumbnail.dataset.imageSrc;
        focused.alt = thumbnail.querySelector("img").alt;
        for (const item of panel.querySelectorAll("[data-image-src]")) {
          item.setAttribute("aria-pressed", String(item === thumbnail));
        }
      }
    }

    const button = event.target.closest("[data-open-work]");
    if (button && dialog) {
      const template = document.getElementById(`work-${button.dataset.openWork}`);
      if (!(template instanceof HTMLTemplateElement)) return;
      dialog.querySelector(".dialog-content").replaceChildren(template.content.cloneNode(true));
      opener = button;
      dialog.showModal();
      document.documentElement.classList.add("modal-open");
      dialog.querySelector("[data-close-dialog]").focus();
    }
  });

  if (dialog) {
    dialog.querySelector("[data-close-dialog]").addEventListener("click", () => dialog.close());
    dialog.addEventListener("click", (event) => {
      if (event.target === dialog) dialog.close();
    });
    // Native <dialog> handles Escape and traps focus; restore the triggering control on close.
    dialog.addEventListener("close", () => {
      document.documentElement.classList.remove("modal-open");
      dialog.querySelector(".dialog-content").replaceChildren();
      opener?.focus();
      opener = null;
    });
  }
})();

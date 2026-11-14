(() => {
  for (const box of document.querySelectorAll(".copiar")) {
    const button = box.querySelector("button[data-copied]");
    const value = box.querySelector(".dato");
    if (!button || !value || !navigator.clipboard) continue;
    const label = button.querySelector("[data-label]");
    const idle = label.textContent;
    let timer = 0;
    button.hidden = false;
    button.addEventListener("click", async () => {
      try {
        await navigator.clipboard.writeText(value.textContent.trim());
      } catch {
        return;
      }
      label.textContent = button.dataset.copied;
      clearTimeout(timer);
      timer = setTimeout(() => {
        label.textContent = idle;
      }, 2000);
    });
  }
})();
(() => {
  for (const box of document.querySelectorAll("label.archivo")) {
    const input = box.querySelector("input[type=file]");
    const name = box.querySelector(".archivo-nombre");
    if (!input || !name) continue;
    const idle = name.textContent;
    input.addEventListener("change", () => {
      name.textContent = input.files.length ? input.files[0].name : idle;
    });
  }
})();

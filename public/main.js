import * as Wasm from "../pkg";

function init() {
  const fileInputEl = document.getElementById("upload");
  const imageEl = document.getElementById("new-img");

  const fileReader = new FileReader();

  fileReader.addEventListener("loadend", () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );

    const result = Wasm.grayscale(base64);

    imageEl.src = result;
  });

  fileInputEl.addEventListener("change", () => {
    fileReader.readAsDataURL(fileInputEl.files[0]);
  });
}

init();

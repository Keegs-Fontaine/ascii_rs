import init, { get_ascii_img } from "../ascii_converter/pkg/ascii_converter.js"

await init()

const fileUploadForm = document.querySelector(".file-upload")
const fileInputEle = document.querySelector(".file-input")
const scaleFactor = document.querySelector(".scale-input")

fileUploadForm.addEventListener("submit", e => {
    e.preventDefault()

    const file = fileInputEle.files[0];

    const fileReader = new FileReader()

    fileReader.onload = (e) => {
        console.log('File content loaded:', e.target.result);
        const arrBuf = new Uint8Array(e.target.result)

        const ascii = get_ascii_img(arrBuf, Number(scaleFactor.value))

        document.querySelector(".ascii-output").innerText = ascii
        console.log(ascii)
    }

    fileReader.readAsArrayBuffer(file)
})
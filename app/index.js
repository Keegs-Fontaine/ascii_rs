import init, { get_ascii_img } from "./pkg/ascii_converter.js"

await init()

const videoEle = document.createElement("video")
const canvas = document.createElement("canvas")
const ctx = canvas.getContext("2d")

const stream = await navigator.mediaDevices.getUserMedia({
    audio: false,
    video: true,
})

videoEle.srcObject = stream;
videoEle.play()

function getFrame() {
    canvas.width = videoEle.videoWidth;
    canvas.height = videoEle.videoHeight;
    ctx.drawImage(videoEle, 0, 0);


    canvas.toBlob((b => {
        b.bytes().then(bytes => {
            const a = get_ascii_img(bytes)
            console.log(a)
        })
    }), "image/png", 1)

}

setInterval(() => getFrame(), 1000)
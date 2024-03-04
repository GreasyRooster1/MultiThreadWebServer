const canvas = document.getElementById('security-matrix');
const ctx = canvas.getContext('2d');

// set the width and height of the canvas
const w = canvas.width;
const h = canvas.height;

// draw a black rectangle of width and height same as that of the canvas
ctx.fillStyle = '#000000';
ctx.fillRect(0, 0, w, h);

const cols = Math.floor(w / 20) + 1;
const ypos = Array(cols).fill(0);
const possible_charsets = [
    "HelloWorldFromRust",
    "SecureDataTransfer",
    "WebsiteByDillonWilson",
    "TryBestToHoldOnToSanity",
    "YouAreAnIdiotHaHaHa",
    "NoEscape.exeMrsMajor.exeILoveYou.txt.vbs",
    "ICouldLiterallyWriteAnythingHereLol",
]
let charset = possible_charsets[Math.floor(Math.random()*possible_charsets.length)];
const offsets = Array(cols).fill(0).map(() => Math.round(Math.random() * charset.length))

function fitToContainer(canvas){
    // Make it visually fill the positioned parent
    canvas.style.width ='100%';
    canvas.style.height='100%';
    // ...then set the internal size to match
    canvas.width  = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;
}

function matrix () {
    // Draw a semitransparent black rectangle on top of previous drawing
    ctx.fillStyle = '#0001';
    ctx.fillRect(0, 0, w, h);

    // Set color to green and font to 15pt monospace in the drawing context
    ctx.fillStyle = '#00ff00';
    ctx.font = '15pt monospace';

    // for each column put a random character at the end
    ypos.forEach((y, ind) => {
        const text = charset[(offsets[ind]+y/20)%charset.length];
        const x = ind * 20;

        ctx.fillText(text, x, y);

        if (y > 100 + Math.random() * 10000) ypos[ind] = 0;
        else ypos[ind] = y + 20;
    });
}

fitToContainer(canvas);
// render the animation at 20 FPS.
setInterval(matrix, 50);
const circleElement = document.querySelector(".circle");

const mouse = {x:0,y:0};
const previousMouse = {x:0,y:0};
const circle = {x:0,y:0,size:1};
const previousCircle = {x:0,y:0};
let currentScale = 0;
let currentAngle = 0;

let expandAnimFrame = 0;
let expandAnimInterval;
let shrinkAnimFrame = 0;
let shrinkAnimInterval;

const expandObjects = [
    document.getElementById("github-item"),
    document.getElementById("title-item"),
]
const maskObjects =  document.getElementsByClassName("mask");

const mask = document.querySelector('#github-mask');

//movement
window.addEventListener("mousemove",(e) => {
    mouse.x = e.x;
    mouse.y = e.y;

})

const speed = 0.17;

const tick = ()=>{
    circle.x += (mouse.x-circle.x)*speed;
    circle.y += (mouse.y-circle.y)*speed;

    const translateTransform = `translate(${circle.x}px, ${circle.y}px)`;

    //squeeze
    const deltaMouseX = mouse.x-previousMouse.x;
    const deltaMouseY = mouse.y-previousMouse.y;
    previousMouse.x = mouse.x;
    previousMouse.y = mouse.y;

    const mouseVel = Math.min(Math.sqrt(deltaMouseX**2+deltaMouseY**2)*4,150)/circle.size;
    const scaleValue = ((mouseVel/150)*0.5)

    currentScale +=((scaleValue-currentScale)*speed)

    const scaleTransform = `scale(${(1+currentScale)*circle.size}, ${(1-currentScale)*circle.size})`;

    //rotate
    const deltaCircleX = circle.x-previousCircle.x;
    const deltaCircleY = circle.y-previousCircle.y;
    previousCircle.x = circle.x;
    previousCircle.y = circle.y;
    const angle = Math.atan2(deltaCircleY,deltaCircleX)*180 /Math.PI;

    if(mouseVel>20){
        currentAngle = angle;
    }

    const rotateTransform = `rotate(${currentAngle}deg)`;

    //apply transforms
    circleElement.style.transform = `${translateTransform} ${rotateTransform} ${scaleTransform}`;

    //apply masks
    let viewportOffset = mask.getBoundingClientRect();
    for(let i=0;i<maskObjects.length;i++) {
        mask.style.setProperty('--mouse-x', (circle.x - viewportOffset.left) + 'px');
        mask.style.setProperty('--mouse-y', (circle.y - viewportOffset.top) + 'px');
        mask.style.setProperty('--mask-size', (circle.size * 20) + 'px');
    }

    window.requestAnimationFrame(tick);
}

tick();

//animation
for(let i=0;i<expandObjects.length;i++) {
    expandObjects[i].addEventListener("mouseenter", (event) => {

        //animate circle
        clearInterval(shrinkAnimInterval);
        shrinkAnimFrame = 0;
        if (!expandAnimFrame) {
            expandAnimInterval = setInterval(() => {
                expandAnimFrame++;
                let maxSize = 10;
                circle.size += Math.abs(circle.size - maxSize) / 20
                if (expandAnimFrame >= 100) {
                    expandAnimFrame = 0;
                    clearInterval(expandAnimInterval);
                }
            }, 10);
        }
    });
    expandObjects[i].addEventListener("mouseleave", (event) => {

        //animate circle
        clearInterval(expandAnimInterval);
        expandAnimFrame = 0;
        if (!shrinkAnimFrame) {
            shrinkAnimInterval = setInterval(() => {
                shrinkAnimFrame+=2;
                circle.size -= Math.abs(circle.size - 1) / 10
                if (shrinkAnimFrame >= 100) {
                    shrinkAnimFrame = 0;
                    clearInterval(shrinkAnimInterval);
                }
            }, 10);
        }
    });
}
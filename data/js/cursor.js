const circleElement = document.querySelector(".circle");

const mouse = {x:0,y:0};
const previousMouse = {x:0,y:0};
const circle = {x:0,y:0};
const previousCircle = {x:0,y:0};
let currentScale = 0;
let currentAngle = 0;

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

    const mouseVel = Math.min(Math.sqrt(deltaMouseX**2+deltaMouseY**2)*4,150);
    const scaleValue = (mouseVel/150)*0.5

    currentScale +=(scaleValue-currentScale)*speed;

    const scaleTransform = `scale(${1+currentScale}, ${1-currentScale})`;

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

    window.requestAnimationFrame(tick);
}

tick();
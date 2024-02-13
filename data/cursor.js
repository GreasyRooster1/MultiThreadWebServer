const circleElement = document.querySelector(".circle");

const mouse = {x:0,y:0};
const circle = {x:0,y:0};

window.addEventListener("mousemove",(e) => {
    mouse.x = e.x;
    mouse.y = e.y;
})

const speed = 0.17;

const tick = ()=>{
    circle.x += (mouse.x-circle.x)*speed;
    circle.y += (mouse.y-circle.y)*speed;

    circleElement.style.transform = `translate(${circle.x}px, ${circle.y}px)`;

    window.requestAnimationFrame(tick);
}

tick();
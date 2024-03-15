import * as THREE from 'three.js';

const container = document.getElementById("rotating-cube")
const scale = container.getBoundingClientRect();

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera( 75, scale.width / scale.height, 0.1, 1000 );

const renderer = new THREE.WebGLRenderer();
renderer.setSize( scale.width, scale.height );
container.appendChild( renderer.domElement );

const geometry =new THREE.BoxGeometry( 3,3,3);
const material = new THREE.MeshBasicMaterial( { color: 0x00ff00 } );
const cube = new THREE.Mesh( geometry, material );
scene.add( cube );

const pointLight1 = new THREE.PointLight( 0xffffff, 3, 0, 0 );
pointLight1.position.set( 50, 50, 50 );
scene.add( pointLight1 );

const pointLight2 = new THREE.PointLight( 0xffffff, 1, 0, 0 );
pointLight2.position.set( - 50, - 50, - 50 );
scene.add( pointLight2 );

camera.position.z = 5;

function animate() {
    requestAnimationFrame( animate );

    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;

    renderer.render( scene, camera );
}

animate();
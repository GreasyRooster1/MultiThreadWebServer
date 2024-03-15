import * as THREE from 'three.js';

const container = document.getElementById("rotating-cube")
const scale = container.getBoundingClientRect();

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera( 75, scale.width / scale.height, 0.1, 1000 );

const renderer = new THREE.WebGLRenderer();
renderer.setSize( scale.width, scale.height );
container.appendChild( renderer.domElement );

const geometry =new THREE.TorusGeometry( 10, 3, 16, 100 );
const material = new THREE.MeshBasicMaterial( { color: 0x00ff00 } );
const cube = new THREE.Mesh( geometry, material );
scene.add( cube );

camera.position.z = 5;

function animate() {
    requestAnimationFrame( animate );

    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;

    renderer.render( scene, camera );
}

animate();
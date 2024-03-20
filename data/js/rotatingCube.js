import * as THREE from 'three.js';
import RenderPass from 'renderpass.js';
import EffectComposer from 'effectcomposer.js';

const container = document.getElementById("rotating-cube")
const scale = container.getBoundingClientRect();

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera( 75, scale.width / scale.height, 0.1, 1000 );

const renderer = new THREE.WebGLRenderer();
renderer.setSize( scale.width, scale.height );
container.appendChild( renderer.domElement );

const renderScene = new RenderPass(scene,camera);
const composer = new EffectComposer(renderer);
composer.addPass(renderScene);

const geometry =new THREE.BoxGeometry( 3,3,3);
const material = new THREE.MeshPhongMaterial( { color: 0x2222ff } );
const cube = new THREE.Mesh( geometry, material );
scene.add( cube );

const pointLight1 = new THREE.PointLight( 0x11ff11, 30, 0, 0 );
pointLight1.position.set( 50, 50, 50 );
scene.add( pointLight1 );

const pointLight2 = new THREE.PointLight( 0xff1111, 20, 0, 0 );
pointLight2.position.set( - 50, - 50, - 20 );
scene.add( pointLight2 );

const pointLight3 = new THREE.PointLight( 0xffffff, 70, 0, 0 );
pointLight3.position.set( 50, 50, - 50 );
scene.add( pointLight3 );

camera.position.z = 5;

function animate() {
    requestAnimationFrame( animate );

    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;



    composer.render( scene, camera );
}

animate();
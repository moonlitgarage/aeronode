// import React, { useRef } from 'react'
// import { useFrame, useLoader } from '@react-three/fiber'
import { OBJLoader } from 'three/addons/loaders/OBJLoader.js';
// import { Group } from 'three'
import { DragControls, TransformControls } from '@react-three/drei'

import { useFrame, useLoader } from "@react-three/fiber"
import { useRef } from "react"
import { Group } from "three"

interface ThreeDModelProps {
  url: string
}




const ThreeDModel: React.FC<ThreeDModelProps> = ({ url }) => {

  const groupRef = useRef<Group>(null)
  const obj = useLoader(OBJLoader, url)

  useFrame(() => {
    if (groupRef.current) {
      // groupRef.current.rotation.y += 0.01
      // groupRef.current.rotation.x += 0.01
    }
  })

  const position = [-5, 16, -1]
  const scale = [1, 1, 1]

  return (
    <TransformControls mode="translate">
      <DragControls ref={groupRef}>
        {/* <mesh>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color="blue" />
        </mesh> */}
      <primitive object={obj} ref={groupRef} scale={scale} position={position} />
      </DragControls>
    </TransformControls>  
  )
    
  // return (
  //   
  // )
}

export default ThreeDModel
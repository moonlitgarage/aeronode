import React, { Suspense } from 'react'
import { Canvas } from '@react-three/fiber'
import ThreeDModel from '@/components/ui/threedmodel'

const ThreeDScence: React.FC = () => {
  return (
    <div className='border border-dashed h-80 w-80'>
        <Canvas>
      <ambientLight intensity={0.5} />
      <spotLight position={[10, 10, 10]} angle={0.15} penumbra={1} />
      <Suspense fallback={null}>
        <ThreeDModel url="https://raw.githubusercontent.com/koerolabs/assets/main/AeroCorev0.obj" />
      </Suspense>
    </Canvas>
    </div>
    
  )
}

export default ThreeDScence
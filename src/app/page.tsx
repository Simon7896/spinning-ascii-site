import Image from 'next/image'

import SpinningAsciiImage from '../components/spinning_ascii_image'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col justify-center items-center">
      <SpinningAsciiImage image="🌎" />
    </main>
  )
}

import data from './api/frames.json'
import SpinningAsciiImage from './components/spinning_ascii_image'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col justify-center items-center">
      <SpinningAsciiImage frames={data.frames} />
    </main>
  )
}

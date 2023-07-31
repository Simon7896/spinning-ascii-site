import data from './api/frames.json'
import SpinningAsciiImage from './components/spinning_ascii_image'

async function getFrames() {
  const res = await fetch("https://spinning-ascii.shuttleapp.rs");

  if (!res.ok) {
    throw new Error("Failed to fetch frames");
  }

  return await res.json();
}

export default async function Home() {

  const data = await getFrames();

  return (
    <main className="flex min-h-screen flex-col justify-center items-center">
      <SpinningAsciiImage frames={data.frames} />
    </main>
  )
}

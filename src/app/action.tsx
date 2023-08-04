'use server'

export default async function processImage(data: FormData) {

  if (!data.has("image")) {
    throw new Error("No image selected");
  }

  const res = await fetch("https://spinning-ascii.shuttleapp.rs/upload", { 
    method: "POST",
    body: data,
    cache: "no-store",
  });

  if (!res.ok) {
    throw new Error("Failed to fetch frames");
  }

  return await await res.json();
}
'use client'
import { useEffect, useRef, useState, useTransition } from 'react';

import SpinningAsciiImage from './components/spinning_ascii_image'
import UploadForm from './components/uploadForm';
import RectButton from './components/rectButton';
import json_data from './api/frames.json';
import processImage from './action';

const Home = () => {
  const initialRender = useRef(true);
  const [isPending, startTransition] = useTransition();
  const [frames_data, setFramesData] = useState(json_data.frames);
  const [formData, setFormData] = useState(new FormData(undefined));

  useEffect(() => {
    if (initialRender.current) {
      initialRender.current = false;
      return;
    }

    startTransition(() => {
      processImage(formData).then(
        (json) => { 
          console.log("Returning fetched JSON!")
          setFramesData(json.frames);
        },
        (reason) => { 
          console.error(reason) 
          console.log("Returning default JSON!");
        }
      )
    });
  }, [formData]);

  if (isPending) {
    return (
      <div className='h-screen flex justify-center items-center'>
        Loading...
      </div>
    )
  }

  return (
    <main className="py-20 flex flex-col justify-center items-center">
      {frames_data? <SpinningAsciiImage frames={frames_data}/> : <>Loading...</>}
      <UploadForm action={ (formData) => setFormData(formData) }>
        <RectButton type="submit">Upload</RectButton>
      </UploadForm>
    </main>
  )
}

export default Home;

'use client'

import { useEffect, useState } from "react";

const SpinnningAsciiImage = (props: { frames: {matrix: string[][]; frame_number:number;}[] }) => {

    const fps = 30; // Frames per second
    const duration = 1000/fps; // Seconds per frame
    const [frameIndex, setFrameIndex] = useState(0);

    useEffect(() => {
        const interval = setInterval(() => {
            setFrameIndex(
                (oldIndex) => { 
                    return (oldIndex+ 1) % props.frames.length;
                }
            );
        }, duration);

        return () => clearInterval(interval);
    });

    return (
        <div className="flex flex-col justify-center items-center">
            { props.frames[frameIndex].matrix.map((row, row_index) => {
                return (
                    <div key={frameIndex+row_index}>{ row }</div>
                )
            })}
        </div>

    )
}

export default SpinnningAsciiImage;
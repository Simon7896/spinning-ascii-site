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
    }, []);

    return (
        <div className="leading-none flex flex-col">
            {props.frames[frameIndex].matrix.map((row, rowIndex) => {
                return (
                    <div key={rowIndex} className="flex">
                        {row.map((char, charIndex) => {
                            return (
                                <div key={charIndex} className="w-4 h-4">
                                    {char}
                                </div>
                            )
                        })}
                    </div>
                )
            })}
        </div>

    )
}

export default SpinnningAsciiImage;
use image::{self, ImageBuffer, Luma};
use std::{f32::consts::PI, error::Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Frame {
    matrix: Vec<Vec<char>>,
    frame_number: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Frames {
    frames: Vec<Frame>,
}

/// Constructs a JSON literal string containing each frame of the spinning animation
/// 
/// # Arguments
/// 
/// * `matrix` - A 2D vector of characters representing the image
/// 
/// # Returns
/// 
/// * A JSON string containing the frames of the animation
/// 
/// # Example
/// 
/// ```
/// let matrix = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
/// let json = run_json(matrix);
/// ```
/// 
/// # Errors
/// 
/// * If the JSON serialization fails
pub fn run_json(matrix: Vec<Vec<char>>) -> Result<String, Box<dyn Error>>{
    let frames = create_frames(matrix.clone());
    return Ok(serde_json::to_string( &frames)?);
}

/// Converts an image into a 2D vector of characters
fn create_frames(matrix: Vec<Vec<char>>) -> Frames {
    let mut frames: Vec<Frame> = Vec::new();

    let mut rad_angle:f32 = 0.0;
    let mut frame_count: u32 = 0;
    let (x_center, y_center) = (matrix[0].len()/2, matrix.len()/2);

    while rad_angle < 2.0*PI {

        // Create a new frame
        let mut frame = Frame {
            matrix: vec![vec![' '; matrix[0].len()]; matrix.len()],
            frame_number: frame_count,
        };

        // Rotate the matrix by the current angle
        for (i, row) in matrix.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {

                let (sina, cosa) = rad_angle.sin_cos(); // Rotation matrix coefficients

                let (x_t, y_t) = (  // Translation matrix coefficients
                    j as f32 - x_center as f32, 
                    i as f32 - y_center as f32
                );

                let (x,y) = ( // Apply rotation and translation
                    (x_t*cosa+y_t*sina)+x_center as f32, 
                    (x_t*sina-y_t*cosa)+y_center as f32
                );

                // Add the character to the frame if it is within the bounds of the frame
                if x >= 0.0 && y >= 0.0 && x < matrix[0].len() as f32 && y < matrix.len() as f32 {
                    frame.matrix[y as usize][x as usize] = *c;
                }
            }
        }

        // Add the frame to the vector of frames
        frames.push(frame);
        frame_count += 1;
        rad_angle += 0.1;
    }

    return Frames { frames };
}

/// Converts an image into a 2D vector of characters
/// 
/// # Arguments
/// 
/// * `img` - An ImageBuffer<Luma<u8>, Vec<u8>> containing the image to be converted
/// * `offset` - The size of the kernel used to average the pixels
/// 
/// # Returns
/// 
/// * A 2D vector of characters representing the image
/// 
/// # Errors
/// 
/// * if the offset fails to convert to usize
pub fn pixels_to_ascii(img: ImageBuffer<Luma<u8>, Vec<u8>>, offset: u32) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let (width,height) = img.dimensions();
    let (mut x, mut y) = (0, 0);

    let mut matrix: Vec<Vec<char>> = vec![vec![' '; (width/offset).try_into()?]; (height/offset).try_into()?];

    while y+offset < height {
        while x+offset < width {
            // Print ascii character based on average pixel value
            let mut avg = 0;
            for m in 0..offset-1 {
                for n in 0..offset-1 {
                    avg += img.get_pixel(x+n, y+m)[0] as u32;
                }
            }
            avg /= offset.pow(2);
            
            // Convert average grayscale pixel value to ascii character
            let ascii = match avg {
                0..=25 => ' ',
                26..=51 => '.',
                52..=77 => '-',
                78..=103 => '~',
                104..=129 => ':',
                130..=155 => '=',
                156..=181 => '+',
                182..=207 => '*',
                208..=233 => '#',
                234..=255 => '@',
                _ => ' ',
            };
            
            // Add the ascii character to the matrix
            matrix[(y/offset) as usize][(x/offset) as usize] = ascii;

            x += offset;
        }
        (x, y) = (0, y+offset);
    }
    return Ok(matrix);
}
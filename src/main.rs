
pub mod ffi;

#[allow(unused_imports)]
use std::convert::AsMut;
use std::{ env, ptr, mem, slice };
use std::ffi::CString;
use std::os::raw::c_int;
use std::default::Default;
use std::fs::{ File, OpenOptions };
use std::io::{Write, Read};

#[allow(unused_imports)]
use ffi::x264::{
    int64_t,
    x264_param_t, x264_picture_t, x264_t, x264_nal_t, 
    
    X264_CSP_I420, 
    
    x264_param_default_preset, x264_param_apply_profile,
    x264_picture_alloc, x264_encoder_open_148, x264_encoder_encode,
    x264_encoder_delayed_frames, x264_encoder_close, 
    x264_picture_clean, 

};

fn main (){
    unsafe {
        let width: c_int  = 1440;
        let height: c_int = 900;

        let mut param: x264_param_t = Default::default();
        let mut nal: x264_nal_t = Default::default();
        let mut i_nal: c_int = 0;

        let ret = x264_param_default_preset(&mut param as *mut x264_param_t, 
            CString::new("medium").unwrap().as_ptr(), ptr::null() );

        param.i_csp = X264_CSP_I420;
        param.i_width  = width;
        param.i_height = height;
        param.b_vfr_input = 0;
        param.b_repeat_headers = 1;
        param.b_annexb = 1;

        let ret = x264_param_apply_profile(&mut param as *mut x264_param_t, 
            CString::new("high").unwrap().as_ptr() );

        let mut pic: x264_picture_t = Default::default();
        let mut pic_out: x264_picture_t = Default::default();

        let ret = x264_picture_alloc(&mut pic as *mut x264_picture_t, 
            param.i_csp, param.i_width, param.i_height);
        
        let h_ptr: *mut x264_t = x264_encoder_open_148( &mut param as *mut x264_param_t );
        let p = h_ptr.as_mut().unwrap();

        let luma_size = (width * height)  as usize;
        let chroma_size = (luma_size / 4) as usize;

        let mut i_frame: int64_t = 0;
        let mut i_frame_size: c_int = -1;

        let mut input  = OpenOptions::new().create(false).read(true).open("me.yuv").unwrap();
        let mut output = OpenOptions::new().create(true).read(true).append(true).open("me.mp4").unwrap();

        // let mut y  = [0; luma_size];
        // let mut uv = [0; chroma_size];

        loop {
            let mut y: Vec<u8> = Vec::with_capacity(luma_size);
            for _i in 0..luma_size {
                y.push(0);
            }
            let ybuff = y.as_mut_slice();

            let mut u: Vec<u8> = Vec::with_capacity(chroma_size);
            for _i in 0..chroma_size {
                u.push(0);
            }
            let ubuff = u.as_mut_slice();

            let mut v: Vec<u8> = Vec::with_capacity(chroma_size);
            for _i in 0..chroma_size {
                v.push(0);
            }
            let vbuff = v.as_mut_slice();

            match input.read( ybuff ) {
                Ok(size) => {
                    if size != luma_size {
                        break;
                    }
                    assert!(size == luma_size);
                    pic.img.plane[0] = ybuff.as_mut_ptr();
                },
                Err(_) => break
            };
            match input.read(ubuff) {
                Ok(size) => {
                    if size != chroma_size {
                        break;
                    }
                    assert!(size == chroma_size);
                    pic.img.plane[1] = ubuff.as_mut_ptr();
                },
                Err(_) => break
            };
            match input.read(vbuff) {
                Ok(size) => {
                    if size != chroma_size {
                        break;
                    }
                    assert!(size == chroma_size);
                    pic.img.plane[2] = vbuff.as_mut_ptr();
                },
                Err(_) => break
            };
            
            pic.i_pts = i_frame;

            let nal_ptr_ptr: *mut *mut x264_nal_t= std::mem::transmute(&mut nal as *mut x264_nal_t);
            i_frame_size = x264_encoder_encode(h_ptr, 
                nal_ptr_ptr, 
                &mut i_nal as *mut c_int, 
                &mut pic as *mut x264_picture_t,
                &mut pic_out as *mut x264_picture_t );
            
            if i_frame_size > 0 {
                println!("[DEBUG] i_frame_size: {:?}", i_frame_size);
                let mp4_bytes = slice::from_raw_parts(
                    nal_ptr_ptr.as_mut().unwrap().as_mut().unwrap().p_payload, i_frame_size as usize);
                output.write( mp4_bytes );
            }
        }
        loop {
            let delayed_frames_count = x264_encoder_delayed_frames(h_ptr);
            
            if delayed_frames_count <= 0 {
                break;
            }

            let nal_ptr_ptr: *mut *mut x264_nal_t= std::mem::transmute(&mut nal as *mut x264_nal_t);
            i_frame_size = x264_encoder_encode(h_ptr, 
                nal_ptr_ptr, 
                &mut i_nal as *mut c_int, 
                ptr::null_mut(),
                &mut pic_out as *mut x264_picture_t );

            if i_frame_size > 0 {
                println!("[DEBUG] delayed frame i_frame_size: {:?}", i_frame_size);
                let mp4_bytes = slice::from_raw_parts(
                    nal_ptr_ptr.as_mut().unwrap().as_mut().unwrap().p_payload, i_frame_size as usize);
                output.write( mp4_bytes );
            }
        }

        x264_encoder_close( h_ptr );
        // drop(pic);
        x264_picture_clean( &mut pic as *mut x264_picture_t );

        println!("[INFO] {:?}", "Done");
    }
}
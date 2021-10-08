use crate::core_foundation_sys::base::{ Boolean, CFTypeID };

use crate::{
    GLenum, GLuint,
    image_buffer::CVImageBufferRef,
};


pub type CVOpenGLESTextureRef = CVImageBufferRef;

extern "C" {
    pub fn CVOpenGLESTextureGetTypeID() -> CFTypeID;

    pub fn CVOpenGLESTextureGetTarget(image: CVOpenGLESTextureRef) -> GLenum;

    pub fn CVOpenGLESTextureGetName(image: CVOpenGLESTextureRef) -> GLuint;

    pub fn CVOpenGLESTextureIsFlipped(image: CVOpenGLESTextureRef) -> Boolean;

//pub fn CVOpenGLESTextureGetCleanTexCoords( image:CVOpenGLESTextureRef  ,
//    GLfloat lowerLeft[ 2],
//    GLfloat lowerRight[ 2],
//    GLfloat upperRight[ 2],
//    GLfloat upperLeft[ 2] );

}

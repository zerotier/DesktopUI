use crate::libc::size_t;
use crate::objc::runtime::Object;
use crate::core_foundation_sys::{
    base::{ CFAllocatorRef, CFTypeRef },
    dictionary::CFDictionaryRef,
};

use crate::{
    GLenum, GLint, GLsizei,
    return_::CVReturn,
    image_buffer::CVImageBufferRef, 
    open_gl_es_texture::CVOpenGLESTextureRef, 
};


pub type CVOpenGLESTextureCacheRef = CFTypeRef;
pub type CVEAGLContext = *mut Object;

extern "C" {
    pub fn CVOpenGLESTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        eaglContext: CVEAGLContext,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVOpenGLESTextureCacheRef,
    ) -> CVReturn;

    pub fn CVOpenGLESTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVOpenGLESTextureCacheRef,
        sourceImage: CVImageBufferRef,
        textureAttributes: CFDictionaryRef,
        target: GLenum,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        planeIndex: size_t,
        textureOut: *mut CVOpenGLESTextureRef,
    ) -> CVReturn;
}

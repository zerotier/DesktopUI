use crate::libc::size_t;
use crate::core_foundation_sys::{
    base::{CFAllocatorRef, CFTypeID, CFTypeRef},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};
use crate::{
    base::CVOptionFlags,
    image_buffer::CVImageBufferRef, 
    metal_texture::CVMetalTextureRef,
    return_::CVReturn,
};


pub type CVMetalTextureCacheRef = CFTypeRef;

extern "C" {
    pub static kCVMetalTextureCacheMaximumTextureAgeKey: CFStringRef;


    pub fn CVMetalTextureCacheGetTypeID() -> CFTypeID;
    pub fn CVMetalTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        metalDevice: metal::Device,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVMetalTextureCacheRef,
    ) -> CVReturn;
    pub fn CVMetalTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVMetalTextureCacheRef,
        sourceImage: CVImageBufferRef,
        textureAttributes: CFDictionaryRef,
        pixelFormat: metal::MTLPixelFormat,
        width: size_t,
        height: size_t,
        planeIndex: size_t,
        textureOut: *mut CVMetalTextureRef,
    ) -> CVReturn;
    pub fn CVMetalTextureCacheFlush(textureCache: CVMetalTextureCacheRef,
                                    options: CVOptionFlags);
}

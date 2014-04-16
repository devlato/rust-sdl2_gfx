pub mod ll {
    /* automatically generated by rust-bindgen */

    use libc::*;
    use sdl2::surface::ll::SDL_Surface;
    extern "C" {
        pub fn rotozoomSurface(src: *mut SDL_Surface, angle: c_double,
                               zoom: c_double, smooth: c_int) -> *mut SDL_Surface;
        pub fn rotozoomSurfaceXY(src: *mut SDL_Surface, angle: c_double,
                                 zoomx: c_double, zoomy: c_double, smooth: c_int)
                                 -> *mut SDL_Surface;
        pub fn rotozoomSurfaceSize(width: c_int, height: c_int, angle: c_double,
                                   zoom: c_double, dstwidth: *mut c_int,
                                   dstheight: *mut c_int);
        pub fn rotozoomSurfaceSizeXY(width: c_int, height: c_int, angle: c_double,
                                     zoomx: c_double, zoomy: c_double,
                                     dstwidth: *mut c_int, dstheight: *mut c_int);
        pub fn zoomSurface(src: *mut SDL_Surface, zoomx: c_double,
                           zoomy: c_double, smooth: c_int) -> *mut SDL_Surface;
        pub fn zoomSurfaceSize(width: c_int, height: c_int, zoomx: c_double,
                               zoomy: c_double, dstwidth: *mut c_int,
                               dstheight: *mut c_int);
        pub fn shrinkSurface(src: *mut SDL_Surface, factorx: c_int,
                             factory: c_int) -> *mut SDL_Surface;
        pub fn rotateSurface90Degrees(src: *mut SDL_Surface,
                                      numClockwiseTurns: c_int) ->
            *mut SDL_Surface;
    }
}
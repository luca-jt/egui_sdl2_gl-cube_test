#ifndef TEST_LIB_H
#define TEST_LIB_H


#include <SDL.h>


struct SDL_comm
{
    SDL_Window * wp;
    SDL_Renderer * rp;
    SDL_Texture * tp;
};

static inline SDL_comm sc = {nullptr, nullptr, nullptr};


extern "C"
{
    int test_func();
    void init_SDL();
    void close_SDL();
    void* get_sdl2_texture();
}


#endif // TEST_LIB_H

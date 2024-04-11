#include "test.h"
#include <cstdint>
#include <SDL.h>
#include <iostream>


extern "C"
{
    int test_func()
    {
        static int32_t i = 0;
        return i++;
    }

    void init_SDL()
    {
        if (SDL_Init(SDL_INIT_VIDEO) != 0)
        {
            std::cerr << "SDL initialization failed: " << SDL_GetError() << std::endl;
            exit(EXIT_FAILURE);
        }
        SDL_Window * sdlWindow = SDL_CreateWindow("Off-screen Rendering", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 800, 600, SDL_WINDOW_HIDDEN);
        if (!sdlWindow)
        {
            std::cerr << "Window creation failed: " << SDL_GetError() << std::endl;
            SDL_Quit();
            exit(EXIT_FAILURE);
        }
        SDL_Renderer * sdlRenderer = SDL_CreateRenderer(sdlWindow, -1, SDL_RENDERER_ACCELERATED);
        if (!sdlRenderer)
        {
            std::cerr << "Renderer creation failed: " << SDL_GetError() << std::endl;
            SDL_DestroyWindow(sdlWindow);
            SDL_Quit();
            exit(EXIT_FAILURE);
        }
        SDL_Texture * sdlTexture = SDL_CreateTexture(sdlRenderer, SDL_PIXELFORMAT_RGBA8888, SDL_TEXTUREACCESS_TARGET, 800, 600);
        if (!sdlTexture)
        {
            std::cerr << "Texture creation failed: " << SDL_GetError() << std::endl;
            SDL_DestroyRenderer(sdlRenderer);
            SDL_DestroyWindow(sdlWindow);
            SDL_Quit();
            exit(EXIT_FAILURE);
        }
        SDL_SetRenderTarget(sdlRenderer, sdlTexture);

        sc = {sdlWindow, sdlRenderer, sdlTexture};
    }

    void close_SDL()
    {
        SDL_SetRenderTarget(sc.rp, nullptr);
        SDL_DestroyTexture(sc.tp);
        SDL_DestroyRenderer(sc.rp);
        SDL_DestroyWindow(sc.wp);
        SDL_Quit();
    }

    void* get_sdl2_texture()
    {
        // Render a red line to the texture
        SDL_SetRenderDrawColor(sc.rp, 255, 0, 0, 255);
        SDL_RenderClear(sc.rp);
        SDL_RenderDrawLine(sc.rp, 400, 300, 500, 400);

        return reinterpret_cast<void*>(sc.tp);
    }
}

use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="bg-slate-900 text-white flex w-screen h-screen">
            <div class="container mx-auto px-8 py-10">
                <div class="flex justify-between items-center mb-8">
                    <h1 class="text-3xl font-bold">{"✨ Creative Corner"}</h1>
                    <Link<Route> to={Route::Login}>
                        <button class="px-4 py-2 rounded-md bg-violet-600 hover:bg-violet-500">{"Back to Login"}</button>
                    </Link<Route>>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="rounded-lg bg-slate-800 p-5 border border-slate-700">
                        <div class="text-2xl mb-2">{"🎨"}</div>
                        <div class="font-semibold mb-1">{"Theme-ready UI"}</div>
                        <div class="text-sm text-slate-300">{"Layout dibuat modular agar mudah ditambah dark/light toggle."}</div>
                    </div>
                    <div class="rounded-lg bg-slate-800 p-5 border border-slate-700">
                        <div class="text-2xl mb-2">{"⚡"}</div>
                        <div class="font-semibold mb-1">{"Quick Message Shortcuts"}</div>
                        <div class="text-sm text-slate-300">{"Pengguna bisa isi pesan cepat dengan template emoji."}</div>
                    </div>
                    <div class="rounded-lg bg-slate-800 p-5 border border-slate-700">
                        <div class="text-2xl mb-2">{"🌐"}</div>
                        <div class="font-semibold mb-1">{"Realtime WebSocket"}</div>
                        <div class="text-sm text-slate-300">{"Pesan tetap broadcast antar user tanpa polling."}</div>
                    </div>
                </div>

                <div class="mt-8 rounded-lg bg-slate-800 p-5 border border-slate-700">
                    <div class="font-semibold mb-2">{"How to explore this app"}</div>
                    <ul class="list-disc pl-6 text-slate-200 text-sm space-y-1">
                        <li>{"Masuk lewat halaman login."}</li>
                        <li>{"Buka halaman chat dan coba tombol quick message."}</li>
                        <li>{"Jalankan beberapa tab browser untuk lihat efek broadcast."}</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

use yew::{function_component, html, Html};

// ========================// 404 //======================== //

#[function_component]
pub fn PageNotFound() -> Html {
    html! {
        <div class="relative flex items-top justify-center min-h-screen bg-slate-100 sm:items-center sm:pt-0">
            <div class="max-w-xl mx-auto sm:px-6 lg:px-8">
                <div class="flex items-center pt-8 sm:justify-start sm:pt-0">
                    <div class="px-4 text-lg text-slate-500 border-r border-slate-400 tracking-wider">
                        {"404"}
                    </div>
                    <div class="ml-4 text-lg text-slate-500 uppercase tracking-wider">
                        {"Not Found"}
                    </div>
                </div>
            </div>
        </div>
    }
}

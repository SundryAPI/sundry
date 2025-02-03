use leptos::prelude::*;

#[component]
pub fn footer() -> impl IntoView {
    view! {
        <footer class="flex w-full justify-between items-center py-4 px-10 bg-gray-800 text-white">
            <div class="flex gap-4">
                <a href="#" aria-label="Facebook">
                    <img src="/images/github.svg" alt="GitHub" class="w-5 h-5" />
                </a>
                <a href="#" aria-label="X">
                    <img src="/images/x.svg" alt="X" class="w-5 h-5" />
                </a>
                <a href="#" aria-label="Instagram">
                    <img src="/images/pgml.svg" alt="pgml" class="w-5 h-5" />
                </a>
            </div>

            <div class="text-xl text-neon-tint-100 font-ibm">sundry_</div>

            <div class="gird grid-cols-2 divide-x divide-neutral-600">
                <a href="#" class="nav px-3">
                    Terms of Service
                </a>
                <a href="#" class="nav px-3">
                    Privacy Policy
                </a>
            </div>
        </footer>
    }
}
